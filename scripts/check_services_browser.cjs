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
const cvPath = (lang) => lang === 'en' ? '/CV/' : '/CV/ru/';
const otherLanguage = (lang) => lang === 'en' ? 'ru' : 'en';

const server = http.createServer(async (request, response) => {
  try {
    const pathname = decodeURIComponent(new URL(request.url, 'http://localhost').pathname);
    if (!pathname.startsWith('/CV/')) { response.writeHead(404).end(); return; }
    let filename = path.resolve(dist, pathname.slice('/CV/'.length));
    if (filename !== dist && !filename.startsWith(`${dist}${path.sep}`)) {
      response.writeHead(403).end(); return;
    }
    if ((await fs.stat(filename)).isDirectory()) filename = path.join(filename, 'index.html');
    const content = await fs.readFile(filename);
    response.writeHead(200, { 'Content-Type': mime[path.extname(filename)] || 'application/octet-stream' });
    response.end(content);
  } catch (error) {
    response.writeHead(error.code === 'ENOENT' ? 404 : 500).end();
  }
});

async function main() {
  await fs.mkdir(output, { recursive: true });
  let browser;
  try {
    await fs.access(path.join(dist, 'services/en/index.html'));
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
      page.on('pageerror', (error) => errors.push(error.message));
      page.on('requestfailed', (request) => errors.push(`Request failed: ${request.url()}`));
      page.on('response', (response) => {
        if (response.status() >= 400) errors.push(`HTTP ${response.status()}: ${response.url()}`);
      });
      try {
        const details = await callback(page);
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

    for (const lang of ['en', 'ru']) {
      for (const width of [320, 375, 768, 1024, 1440]) {
        for (const theme of ['light', 'dark']) {
          await check(`${lang}-${width}-${theme}`, width, theme, async (page) => {
            await open(page, servicesPath(lang));
            const details = await inspectServices(page, lang, width, theme);
            if (width === 1440 || (width === 375 && theme === 'dark')) {
              await screenshot(page, `${lang}-${width}-${theme}.png`);
            }
            if (lang === 'en' && width === 1440 && theme === 'dark') {
              await screenshot(page, 'en-full-dark.png', true);
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
