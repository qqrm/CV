#!/usr/bin/env python3
"""Verify the fixed public Pages URLs, including B2B search discovery."""

from concurrent.futures import ThreadPoolExecutor
from collections import Counter
from dataclasses import dataclass
from email.message import Message
from http.client import HTTPException
from html.parser import HTMLParser
import re
import sys
import time
from urllib.error import HTTPError, URLError
from urllib.parse import urljoin, urlsplit
from urllib.request import HTTPRedirectHandler, Request, build_opener
from xml.etree import ElementTree


ORIGIN = "https://qqrm.github.io"
BASE = f"{ORIGIN}/CV/"
SERVICES = {lang: f"{BASE}services/{lang}/" for lang in ("en", "ru")}
ROOT_ROBOTS = f"{ORIGIN}/robots.txt"
SITEMAP = f"{BASE}sitemap.xml"
ASSETS = (
    "", "ru/", "cv.md", "cv_ru.md", "cv.txt", "cv_ru.txt", "cv.html",
    "cv_ru.html", "llms.txt", "Belyakov_en_light.pdf", "NotoSans-Regular.ttf",
    "NotoSans-Bold.ttf", "NotoSans-Italic.ttf", "NotoSans-BoldItalic.ttf",
    "style.css", "services/services.css", "services/theme.js", "favicon.svg",
)
CRAWL_PATHS = (
    "/CV/services/en/", "/CV/services/ru/", "/CV/sitemap.xml",
    *[f"/CV/{path}" for path in ASSETS if path.endswith((".css", ".js", ".ttf", ".svg"))],
)
CRAWLERS = ("Googlebot", "Bingbot", "YandexBot", "OAI-SearchBot")
URLS = (*SERVICES.values(), SITEMAP, ROOT_ROBOTS, f"{BASE}robots.txt",
        f"{BASE}sitemap.txt", *[f"{BASE}{path}" for path in ASSETS])
MAX_BYTES = 2 * 1024 * 1024
REQUEST_TIMEOUT = 15
ATTEMPTS = 10
RETRY_DELAY = 30
RESTRICTIVE = re.compile(
    r"\b(?:noindex|nofollow|none|nosnippet)\b|\bmax-snippet\s*:\s*0\b|\bunavailable_after\s*:",
    re.IGNORECASE,
)


class VerificationError(Exception):
    """A response does not satisfy the public-site contract."""


def require(condition, message):
    if not condition:
        raise VerificationError(message)


@dataclass
class Response:
    url: str
    status: int
    headers: Message
    body: bytes

    def text(self):
        return self.body.decode("utf-8-sig")


class SameOriginRedirects(HTTPRedirectHandler):
    def redirect_request(self, request, fp, code, message, headers, newurl):
        target = urlsplit(newurl)
        require((target.scheme, target.netloc) == ("https", "qqrm.github.io"),
                f"Redirect outside the fixed Pages origin: {newurl}")
        return super().redirect_request(request, fp, code, message, headers, newurl)


def fetch(url):
    """Bound socket waits, body size and elapsed reads; never fetch PR artifacts."""
    require(url in URLS, f"Unexpected verification URL: {url}")
    request = Request(url, headers={
        "User-Agent": "CV-Post-Deploy-Verify/1.0",
        "Cache-Control": "no-cache", "Accept-Encoding": "identity",
    })
    started = time.monotonic()
    try:
        response = build_opener(SameOriginRedirects()).open(request, timeout=REQUEST_TIMEOUT)
    except HTTPError as error:
        response = error
    with response:
        chunks = []
        size = 0
        while True:
            chunk = response.read1(min(65536, MAX_BYTES + 1 - size))
            require(time.monotonic() - started <= REQUEST_TIMEOUT * 2,
                    f"Response timed out: {url}")
            if not chunk:
                break
            size += len(chunk)
            require(size <= MAX_BYTES, f"Response exceeds {MAX_BYTES} bytes: {url}")
            chunks.append(chunk)
        return Response(response.geturl(), response.status, response.headers, b"".join(chunks))


class ServiceHTML(HTMLParser):
    VOID = {"area", "base", "br", "col", "embed", "hr", "img", "input", "link",
            "meta", "param", "source", "track", "wbr"}

    def __init__(self):
        super().__init__(convert_charrefs=True)
        self.stack = []
        self.languages = []
        self.canonicals = []
        self.alternates = []
        self.switch_links = []
        self.robots = []
        self.bases = []

    def handle_starttag(self, tag, attributes):
        attrs = dict(attributes)
        in_head = any(name == "head" for name, _ in self.stack)
        if tag == "html":
            self.languages.append(attrs.get("lang"))
        if in_head and tag == "base":
            self.bases.append(attrs.get("href"))
        if in_head and tag == "link":
            rels = (attrs.get("rel") or "").lower().split()
            if "canonical" in rels:
                self.canonicals.append(attrs.get("href"))
            if "alternate" in rels and "hreflang" in attrs:
                self.alternates.append((attrs["hreflang"], attrs.get("href")))
        if in_head and tag == "meta":
            name = (attrs.get("name") or "").lower()
            if name in {"robots", "googlebot", "bingbot", "yandex", "yandexbot", "oai-searchbot"}:
                self.robots.append(attrs.get("content") or "")
        if tag == "a" and any(switch for _, switch in self.stack):
            self.switch_links.append(attrs)
        if tag not in self.VOID:
            self.stack.append((tag, "language-switch" in (attrs.get("class") or "").split()))

    def handle_endtag(self, tag):
        for position in range(len(self.stack) - 1, -1, -1):
            if self.stack[position][0] == tag:
                del self.stack[position:]
                break


def check_available(response, url, content_types=(), allow_empty=False):
    require(response.status == 200, f"{url} -> HTTP {response.status}, expected 200")
    require(response.url == url, f"Unexpected redirect: {url} -> {response.url}")
    require(allow_empty or response.body, f"Empty response: {url}")
    if content_types:
        actual = response.headers.get_content_type()
        require(actual in content_types, f"{url} -> Content-Type {actual}, expected {content_types}")


def check_indexable(response):
    for value in response.headers.get_all("X-Robots-Tag", []):
        require(not RESTRICTIVE.search(value), f"{response.url} -> restrictive X-Robots-Tag: {value}")


def check_service(response, lang):
    url = SERVICES[lang]
    check_available(response, url, ("text/html",))
    check_indexable(response)
    page = ServiceHTML()
    page.feed(response.text())
    page.close()
    require(page.languages == [lang], f"{url} -> wrong HTML language: {page.languages}")
    require(not page.bases, f"{url} -> unexpected base URL")
    require(page.canonicals == [url], f"{url} -> wrong canonical: {page.canonicals}")
    expected = sorted([*SERVICES.items(), ("x-default", SERVICES["en"])])
    require(Counter(page.alternates) == Counter(expected), f"{url} -> wrong hreflang alternatives: {page.alternates}")
    require(all(not RESTRICTIVE.search(value) for value in page.robots),
            f"{url} -> restrictive robots meta: {page.robots}")
    switches = [(link.get("hreflang"), link.get("lang"),
                 urljoin(url, link.get("href") or ""), link.get("aria-current"))
                for link in page.switch_links]
    expected_switches = [(other, other, target, "page" if other == lang else None)
                         for other, target in SERVICES.items()]
    require(Counter(switches) == Counter(expected_switches),
            f"{url} -> wrong language-switch links: {switches}")


def check_sitemap(response):
    check_available(response, SITEMAP, ("application/xml", "text/xml"))
    check_indexable(response)
    require(b"<!DOCTYPE" not in response.body.upper(), "Unexpected sitemap DTD")
    root = ElementTree.fromstring(response.body)
    ns = "{http://www.sitemaps.org/schemas/sitemap/0.9}"
    require(root.tag == f"{ns}urlset", "Sitemap must use the sitemap XML namespace")
    entries = list(root)
    locations = [entry.findtext(f"{ns}loc", "").strip() for entry in entries]
    require(all(entry.tag == f"{ns}url" and len(entry.findall(f"{ns}loc")) == 1 for entry in entries)
            and sorted(locations) == sorted(SERVICES.values()),
            f"Sitemap must contain exactly the two services: {locations}")


def normalize_robots_path(path):
    # Fixed checked URLs are ASCII. Decode equivalent unreserved escapes only;
    # encoded reserved separators must not become a different path.
    def unreserved(match):
        char = chr(int(match.group()[1:], 16))
        return char if char.isascii() and (char.isalnum() or char in "-._~") else match.group().upper()
    return re.sub(r"%[0-9a-fA-F]{2}", unreserved, path)


def robots_allowed(text, crawler, path):
    """Apply specific groups, merged matching groups, longest rule and Allow ties.

    Unlike urllib.robotparser's first-rule behavior, this handles the Allow
    exceptions, '*' wildcards and '$' end anchors used by search crawlers.
    https://developers.google.com/crawling/docs/robots-txt/robots-txt-spec
    """
    groups = []
    agents, rules = [], []
    have_records = False
    for line in text.splitlines():
        key, separator, value = line.split("#", 1)[0].strip().partition(":")
        if not separator:
            continue
        key, value = key.strip().lower(), value.strip()
        if key == "user-agent":
            if have_records:
                groups.append((agents, rules))
                agents, rules, have_records = [], [], False
            # Crawler tokens ignore suffixes such as Googlebot/2.1 or Googlebot*.
            token = re.match(r"[a-z_-]+", value.lower())
            agents.append(token.group() if token else value.lower())
        elif agents and key in ("allow", "disallow"):
            have_records = True
            if value.startswith("/"):
                rules.append((normalize_robots_path(value), key == "allow"))
        # Other fields, including sitemap and crawl-delay, do not split groups.
    groups.append((agents, rules))
    crawler = crawler.lower()
    specific = [group for group_agents, group in groups if crawler in group_agents]
    selected = specific or [group for group_agents, group in groups if "*" in group_agents]
    applicable = [rule for group in selected for rule in group]
    matches = []
    path = normalize_robots_path(path)
    for pattern, allow in applicable:
        anchored = pattern.endswith("$")
        literal = pattern[:-1] if anchored else pattern
        expression = "^" + re.escape(literal).replace(r"\*", ".*") + ("$" if anchored else "")
        if re.search(expression, path):
            matches.append((len(pattern.encode("utf-8")), allow))
    return max(matches, default=(0, True))[1]


def check_root_robots(response):
    target = urlsplit(response.url)
    require((target.scheme, target.netloc) == ("https", "qqrm.github.io"),
            f"Root robots.txt left the Pages origin: {response.url}")
    if response.status in (404, 410):
        return f"Root robots.txt HTTP {response.status}: no host rules"
    # A server error/rate limit is uncertainty, never proof of permission.
    check_available(response, response.url, ("text/plain",), allow_empty=True)
    text = response.text()
    for crawler in CRAWLERS:
        for path in CRAWL_PATHS:
            require(robots_allowed(text, crawler, path),
                    f"Root robots.txt blocks {crawler}: {path}")
    return "Root robots.txt permits the checked search crawlers and service assets"


def verify_once(fetcher=fetch):
    """Fetch independent fixed URLs concurrently, then report every failed check."""
    responses, errors = {}, []
    with ThreadPoolExecutor(max_workers=8) as executor:
        pending = {url: executor.submit(fetcher, url) for url in URLS}
        for url, future in pending.items():
            try:
                responses[url] = future.result()
            except (OSError, URLError, HTTPException, VerificationError) as error:
                errors.append(f"{url} -> {error}")
    checks = {url: (lambda response, lang=lang: check_service(response, lang))
              for lang, url in SERVICES.items()}
    checks[SITEMAP] = check_sitemap
    checks[ROOT_ROBOTS] = check_root_robots
    checks[f"{BASE}robots.txt"] = lambda response: (
        check_available(response, f"{BASE}robots.txt", ("text/plain",)),
        require(re.search(r"(?im)^\s*Disallow:\s*/\s*$", response.text()),
                "Nested robots artifact is missing its existing Disallow rule"),
    )
    checks[f"{BASE}sitemap.txt"] = lambda response: require(
        response.status in (404, 410), f"Legacy sitemap.txt -> HTTP {response.status}, expected 404/410")
    for path in ASSETS:
        url = f"{BASE}{path}"
        content_types = ("application/pdf",) if path.endswith(".pdf") else ()
        checks[url] = lambda response, url=url, types=content_types: check_available(response, url, types)
    for url, check in checks.items():
        if url not in responses:
            continue
        try:
            detail = check(responses[url])
            print(f"PASS {url} -> HTTP {responses[url].status}"
                  + (f"; {detail}" if isinstance(detail, str) else ""), flush=True)
        except (VerificationError, UnicodeError, ElementTree.ParseError) as error:
            errors.append(str(error))
    return errors


def main():
    for attempt in range(1, ATTEMPTS + 1):
        print(f"Attempt {attempt}/{ATTEMPTS}", flush=True)
        errors = verify_once()
        if not errors:
            print("Post-deploy verification passed", flush=True)
            return 0
        for error in errors:
            print(f"FAIL {error}", file=sys.stderr, flush=True)
        if attempt < ATTEMPTS:
            print(f"Deployment may still be propagating; retrying in {RETRY_DELAY}s", flush=True)
            time.sleep(RETRY_DELAY)
    print(f"Post-deploy verification failed after {ATTEMPTS} attempts", file=sys.stderr)
    return 1


if __name__ == "__main__":
    sys.exit(main())
