#!/usr/bin/env node
// Exercise the assembled site in an isolated browser; only localhost is allowed.
const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const http = require('node:http');
const path = require('node:path');

const dist = path.resolve('dist');
const output = path.resolve('artifacts/b2b-preview');
const results = { checks: [], screenshots: [] };
const mime = {
  '.html': 'text/html; charset=utf-8', '.css': 'text/css; charset=utf-8',
  '.js': 'text/javascript; charset=utf-8', '.wasm': 'application/wasm',
  '.svg': 'image/svg+xml', '.jpg': 'image/jpeg', '.ttf': 'font/ttf',
  '.pdf': 'application/pdf', '.txt': 'text/plain; charset=utf-8',
};
const servicesPath = (lang) => `/CV/services/${lang}/`;
const servicesUrl = (lang) => `https://qqrm.github.io${servicesPath(lang)}`;
const cvPath = (lang) => lang === 'en' ? '/CV/' : '/CV/ru/';
const otherLanguage = (lang) => lang === 'en' ? 'ru' : 'en';

const server = http.createServer(async (request, response) => {
  let handle = null;
  try {
    const pathname = decodeURIComponent(new URL(request.url, 'http://localhost').pathname);
    if (!pathname.startsWith('/CV/')) { response.writeHead(404).end(); return; }
    let filename = path.resolve(dist, pathname.slice('/CV/'.length));
    if (filename !== dist && !filename.startsWith(`${dist}${path.sep}`)) {
      response.writeHead(403).end(); return;
    }
    handle = await fs.open(filename, 'r');
    try {
      const stats = await handle.stat();
      if (stats.isDirectory()) {
        await handle.close();
        handle = null;
        filename = path.join(filename, 'index.html');
        handle = await fs.open(filename, 'r');
      }
      const content = await handle.readFile();
      await handle.close();
      handle = null;
      response.writeHead(200, { 'Content-Type': mime[path.extname(filename)] || 'application/octet-stream' });
      response.end(content);
    } catch (inner) {
      if (handle) { try { await handle.close(); } catch {} handle = null; }
      throw inner;
    }
  } catch (error) {
    if (handle) { try { await handle.close(); } catch {} handle = null; }
    response.writeHead(error.code === 'ENOENT' ? 404 : 500).end();
  }
});

async function main() {
  await fs.mkdir(output, { recursive: true });
  let browser;
  try {
    await fs.access(path.join(dist, 'services/en/index.html'));
    const sitemap = await fs.readFile(path.join(dist, 'sitemap.xml'), 'utf8');
    await new Promise((resolve, reject) => {
      server.once('error', reject);
      server.listen(0, '127.0.0.1', resolve);
    });
    const origin = `http://127.0.0.1:${server.address().port}`;
    const { chromium } = require(process.env.CV_PLAYWRIGHT_MODULE || 'playwright');
    browser = await chromium.launch({ headless: true });

    async function screenshot(page, name, fullPage = false) {
      await page.screenshot({ path: path.join(output, name), fullPage, animations: 'disabled' });
      results.screenshots.push(name);
    }

    async function check(name, width, theme, callback, javaScriptEnabled = true) {
      const errors = [];
      const requests = [];
      const context = await browser.newContext({
        viewport: { width, height: width <= 375 ? 1000 : 1100 },
        deviceScaleFactor: 1, colorScheme: theme, reducedMotion: 'reduce', javaScriptEnabled,
        storageState: { cookies: [], origins: [{ origin, localStorage: [{ name: 'cv-theme', value: theme }] }] },
      });
      await context.route('**/*', (route) => {
        if (new URL(route.request().url()).origin === origin) return route.continue();
        errors.push(`External request blocked: ${route.request().url()}`);
        return route.abort();
      });
      const page = await context.newPage();
      page.setDefaultTimeout(15000);
      page.on('request', (request) => requests.push(new URL(request.url()).pathname));
      page.on('pageerror', (error) => errors.push(error.message));
      page.on('requestfailed', (request) => {
        // Chromium may cancel modulepreloads when scripts are disabled. The
        // no-JS fallback must not depend on the CV's WASM loader completing.
        // Missing assets still fail via HTTP errors and the JS-enabled cases.
        if (!javaScriptEnabled && request.resourceType() === 'script') return;
        errors.push(`Request failed: ${request.url()} (${request.failure()?.errorText})`);
      });
      page.on('response', (response) => {
        if (response.status() >= 400) errors.push(`HTTP ${response.status()}: ${response.url()}`);
      });
      try {
        const details = await callback(page, requests);
        assert.deepEqual(errors, [], 'Browser and asset errors');
        results.checks.push({ name, passed: true, ...details });
        console.log(`PASS ${name}`);
      } catch (error) {
        results.checks.push({ name, passed: false, error: error.stack, browserErrors: errors });
        await screenshot(page, `failure-${name}.png`).catch(() => {});
        console.error(`FAIL ${name}: ${error.message}`);
      } finally {
        await context.close();
      }
    }

    async function open(page, pathname) {
      const response = await page.goto(`${origin}${pathname}`, { waitUntil: 'networkidle' });
      assert.equal(response.status(), 200, pathname);
      await page.evaluate(() => document.fonts.ready);
    }

    async function expectPage(page, pathname, lang) {
      await page.waitForURL(`${origin}${pathname}`);
      await page.waitForFunction((expected) => document.documentElement.lang === expected, lang);
      await page.waitForLoadState('networkidle');
      await page.evaluate(() => document.fonts.ready);
    }

    async function inspectServices(page, lang, width, theme) {
      const details = await page.evaluate(async () => {
        await Promise.all([400, 700].map((weight) => document.fonts.load(`${weight} 16px "Noto Sans"`)));
        const root = document.documentElement;
        return {
          lang: root.lang, theme: root.dataset.theme,
          width: root.clientWidth, scrollWidth: Math.max(root.scrollWidth, document.body.scrollWidth),
          font: getComputedStyle(document.body).fontFamily,
          fontLoaded: [400, 700].every((weight) => [...document.fonts].some((face) =>
            face.family.includes('Noto Sans') && face.weight === String(weight) && face.status === 'loaded')),
          columns: [...document.querySelectorAll('.offer')].map((card) =>
            getComputedStyle(card).gridTemplateColumns.trim().split(/\s+/).length),
          missingAnchors: [...document.querySelectorAll('a[href^="#"]')].map((a) => a.hash.slice(1))
            .filter((id) => !document.getElementById(id)),
        };
      });
      assert.equal(details.lang, lang);
      assert.equal(details.theme, theme);
      assert.ok(details.scrollWidth <= details.width + 1, `Horizontal overflow: ${JSON.stringify(details)}`);
      assert.ok(details.font.startsWith('"Noto Sans"') || details.font.startsWith('Noto Sans'));
      assert.ok(details.fontLoaded, 'Noto Sans regular and bold must load');
      assert.deepEqual(details.columns, Array(3).fill(width <= 800 ? 1 : 2));
      assert.deepEqual(details.missingAnchors, []);
      assert.equal(await page.locator('.offer-details li').count(), 12);
      return details;
    }

    async function inspectSearchMetadata(page, lang) {
      const metadata = await page.evaluate((xml) => {
        const meta = (name) => document.querySelector(`head meta[name="${name}"]`)?.content || '';
        const map = new DOMParser().parseFromString(xml, 'application/xml');
        return {
          title: document.title, description: meta('description'), robots: meta('robots'), googlebot: meta('googlebot'),
          canonical: [...document.querySelectorAll('head link[rel="canonical"]')].map((link) => link.href),
          alternates: [...document.querySelectorAll('head link[rel="alternate"][hreflang]')]
            .map((link) => [link.hreflang, link.getAttribute('href')]),
          og: Object.fromEntries([...document.querySelectorAll('head meta[property^="og:"]')]
            .map((tag) => [tag.getAttribute('property'), tag.content])),
          schemas: [...document.querySelectorAll('script[type="application/ld+json"]')].map((script) => JSON.parse(script.textContent)),
          offers: Object.fromEntries([...document.querySelectorAll('.offer')]
            .map((offer) => [offer.id, offer.querySelector('h2').textContent.trim()])),
          mailtos: [...document.querySelectorAll('a[href^="mailto:"]')].map((link) => link.href),
          sitemapError: Boolean(map.querySelector('parsererror')),
          sitemapNamespace: map.documentElement.namespaceURI,
          sitemapUrls: [...map.getElementsByTagName('loc')].map((node) => node.textContent.trim()),
        };
      }, sitemap);
      const canonical = servicesUrl(lang);
      assert.deepEqual(metadata.canonical, [canonical]);
      assert.deepEqual(metadata.alternates.sort(), [
        ['en', servicesUrl('en')], ['ru', servicesUrl('ru')], ['x-default', servicesUrl('en')],
      ]);
      const rules = metadata.robots.toLowerCase().split(/\s*,\s*/);
      assert.ok(rules.includes('index') && rules.includes('follow'), 'Services must permit indexing and link discovery');
      assert.doesNotMatch(`${metadata.robots},${metadata.googlebot}`, /noindex|nofollow|nosnippet|max-snippet\s*:\s*0/i);
      assert.ok(metadata.title.trim() && metadata.description.trim());
      for (const value of [metadata.title, metadata.description]) {
        assert.equal(/[А-Яа-яЁё]/.test(value), lang === 'ru', 'Metadata must match the page language');
      }
      assert.equal(metadata.og['og:url'], canonical);
      assert.equal(metadata.og['og:title'], metadata.title);
      assert.equal(metadata.og['og:description'], metadata.description);
      assert.equal(metadata.schemas.length, 1);
      const schema = metadata.schemas[0];
      assert.equal(schema['@context'], 'https://schema.org');
      assert.equal(schema['@graph'].length, 5);
      const nodes = (type) => schema['@graph'].filter((node) => node['@type'] === type);
      assert.equal(nodes('Person').length, 1);
      assert.equal(nodes('WebPage').length, 1);
      assert.equal(nodes('Service').length, 3);
      const person = nodes('Person')[0];
      const webpage = nodes('WebPage')[0];
      assert.ok(person['@id']);
      assert.equal(webpage['@id'], `${canonical}#webpage`);
      assert.equal(webpage.url, canonical);
      assert.equal(webpage.inLanguage, lang);
      for (const id of ['agentic', 'automation', 'product']) {
        const service = nodes('Service').find((node) => node['@id'] === `${canonical}#${id}`);
        assert.ok(service, `Missing structured service: ${id}`);
        assert.equal(service.url, `${canonical}#${id}`);
        assert.equal(service.name, metadata.offers[id]);
        assert.equal(service.provider['@id'], person['@id']);
      }
      assert.equal(metadata.mailtos.length, 4, 'Each offer and the contact section need an email link');
      for (const href of metadata.mailtos) {
        const mail = new URL(href);
        assert.equal(mail.pathname, 'qqrm@vivaldi.net');
        const subject = mail.searchParams.get('subject') || '';
        assert.ok(subject.includes(`[${lang.toUpperCase()}]`), 'Contact subject must identify the page language');
        assert.ok(subject.replace(/\[(RU|EN)\]/, '').trim().length > 8, 'Contact subject must describe the enquiry');
      }
      assert.equal(metadata.sitemapError, false, 'Sitemap must be valid XML');
      assert.equal(metadata.sitemapNamespace, 'http://www.sitemaps.org/schemas/sitemap/0.9');
      assert.deepEqual(metadata.sitemapUrls.sort(), ['en', 'ru'].map(servicesUrl));
    }

    for (const lang of ['en', 'ru']) {
      for (const width of [320, 375, 768, 1024, 1440]) {
        for (const theme of ['light', 'dark']) {
          await check(`${lang}-${width}-${theme}`, width, theme, async (page, requests) => {
            await open(page, servicesPath(lang));
            const details = await inspectServices(page, lang, width, theme);
            assert.deepEqual(requests.filter((url) => /\.wasm(?:\.(?:gz|br))?$/.test(url)), [], 'Services must not load the CV WASM bundle');
            if (width === 1440 || (width === 375 && theme === 'dark')) {
              await screenshot(page, `${lang}-${width}-${theme}.png`);
            }
            if (width === 1440 && theme === 'dark') {
              await inspectSearchMetadata(page, lang);
              await screenshot(page, `${lang}-full-dark.png`, true);
            }
            return details;
          });
        }
      }

      await check(`${lang}-navigation`, 375, 'dark', async (page) => {
        const other = otherLanguage(lang);
        await open(page, servicesPath(lang));
        for (const id of ['agentic', 'automation', 'product']) {
          await page.locator(`.offer-nav a[href="#${id}"]`).click();
          assert.equal(new URL(page.url()).hash, `#${id}`);
        }
        await page.locator('.header-contact').click();
        assert.equal(new URL(page.url()).hash, '#contact');
        await page.locator(`.language-switch a[hreflang="${other}"]`).click();
        await expectPage(page, servicesPath(other), other);
        assert.equal(await page.locator('html').getAttribute('data-theme'), 'dark');
        await page.locator(`.language-switch a[hreflang="${lang}"]`).click();
        await expectPage(page, servicesPath(lang), lang);
        await page.locator('.site-footer a').click();
        await expectPage(page, cvPath(lang), lang);
        await page.locator(`.actions a[href="${servicesPath(lang)}"]`).click();
        await expectPage(page, servicesPath(lang), lang);
        await page.locator('.identity').click();
        await expectPage(page, cvPath(lang), lang);
        await page.locator('.language-toggle').click();
        await expectPage(page, cvPath(other), other);
        await page.locator(`.actions a[href="${servicesPath(other)}"]`).click();
        await expectPage(page, servicesPath(other), other);
      });

      await check(`${lang}-no-javascript`, 375, 'dark', async (page) => {
        const other = otherLanguage(lang);
        await open(page, servicesPath(lang));
        await inspectServices(page, lang, 375, 'light');
        await page.locator('.header-contact').click();
        assert.equal(new URL(page.url()).hash, '#contact');
        await page.locator(`.language-switch a[hreflang="${other}"]`).click();
        await expectPage(page, servicesPath(other), other);
        await page.locator('.site-footer a').click();
        await page.waitForURL(`${origin}${cvPath(other)}`);
        await page.waitForLoadState('networkidle');
        await page.evaluate(() => document.fonts.ready);
        await page.locator(`noscript a[href="${servicesPath(other)}"]`).click();
        await expectPage(page, servicesPath(other), other);
        await open(page, lang === 'en' ? '/CV/cv.html' : '/CV/cv_ru.html');
        await page.locator(`nav a[href="${servicesPath(lang)}"]`).click();
        await expectPage(page, servicesPath(lang), lang);
      }, false);
    }
  } catch (error) {
    results.fatalError = error.stack;
    console.error(error);
  } finally {
    if (browser) await browser.close();
    if (server.listening) await new Promise((resolve) => server.close(resolve));
    results.passed = results.checks.filter((check) => check.passed).length;
    results.failed = results.checks.filter((check) => !check.passed).length;
    await fs.writeFile(path.join(output, 'results.json'), `${JSON.stringify(results, null, 2)}\n`);
    console.log(`Browser QA: ${results.passed} passed, ${results.failed} failed`);
    if (results.fatalError || results.failed || results.checks.length !== 24) process.exitCode = 1;
  }
}

main().catch((error) => { console.error(error); process.exitCode = 1; });
