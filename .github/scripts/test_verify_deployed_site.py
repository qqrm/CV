"""Regression checks for live B2B discovery, using no network or browser."""

from contextlib import redirect_stderr, redirect_stdout
from email.message import Message
from http.client import IncompleteRead
import io
from pathlib import Path
import unittest
from unittest.mock import patch
from urllib.error import URLError
from urllib.request import Request
from xml.etree import ElementTree

import verify_deployed_site as verify


ROOT = Path(__file__).resolve().parents[2]


def response(url, body=b"asset", status=200, content_type="text/plain", extra_headers=()):
    headers = Message()
    headers["Content-Type"] = content_type
    for name, value in extra_headers:
        headers[name] = value
    if isinstance(body, str):
        body = body.encode("utf-8")
    return verify.Response(url, status, headers, body)


def service(lang="en", **kwargs):
    body = (ROOT / f"services/{lang}/index.html").read_bytes()
    return response(verify.SERVICES[lang], body, content_type="text/html", **kwargs)


def sitemap(body=None, **kwargs):
    if body is None:
        body = (ROOT / "services/sitemap.xml").read_bytes()
    return response(verify.SITEMAP, body, content_type="application/xml", **kwargs)


def public_responses():
    values = {url: response(url) for url in verify.URLS}
    values.update({url: service(lang) for lang, url in verify.SERVICES.items()})
    values[verify.SITEMAP] = sitemap()
    values[verify.ROOT_ROBOTS] = response(verify.ROOT_ROBOTS, status=404)
    values[f"{verify.BASE}robots.txt"] = response(
        f"{verify.BASE}robots.txt", "User-agent: *\nDisallow: /\n")
    values[f"{verify.BASE}sitemap.txt"] = response(f"{verify.BASE}sitemap.txt", status=404)
    pdf = f"{verify.BASE}Belyakov_en_light.pdf"
    values[pdf] = response(pdf, b"%PDF-1.7", content_type="application/pdf")
    return values


class ServiceChecks(unittest.TestCase):
    def test_current_bilingual_html_contract(self):
        for lang in verify.SERVICES:
            with self.subTest(lang=lang):
                verify.check_service(service(lang), lang)

    def test_soft_404_wrong_language_and_wrong_mime_are_rejected(self):
        for body, content_type in [
            (b'<html lang="en"><head><title>Not found</title></head></html>', "text/html"),
            (service().body.replace(b'<html lang="en"', b'<html lang="ru"'), "text/html"),
            (service().body, "text/plain"),
        ]:
            with self.subTest(body=body[:80], content_type=content_type):
                with self.assertRaises(verify.VerificationError):
                    verify.check_service(response(verify.SERVICES["en"], body,
                                                  content_type=content_type), "en")

    def test_redirect_to_cv_is_rejected(self):
        page = service()
        page.url = verify.BASE
        with self.assertRaisesRegex(verify.VerificationError, "redirect"):
            verify.check_service(page, "en")

    def test_canonical_and_language_link_regressions(self):
        original = service().body
        canonical = b'<link rel="canonical" href="https://qqrm.github.io/CV/services/en/" />'
        replacements = [
            (canonical, canonical.replace(b"services/en/", b"services/ru/")),
            (canonical, canonical + canonical),
            (b'hreflang="x-default" href="https://qqrm.github.io/CV/services/en/"',
             b'hreflang="x-default" href="https://qqrm.github.io/CV/services/ru/"'),
            (b'hreflang="ru" href="https://qqrm.github.io/CV/services/ru/"',
             b'hreflang="ru" href="../ru/"'),
            (b'href="../ru/" lang="ru"', b'href="../../" lang="ru"'),
            (b'lang="ru" hreflang="ru" aria-label=', b'lang="ru" aria-label='),
            (b'<head>', b'<head><base href="https://example.org/">'),
        ]
        for before, after in replacements:
            with self.subTest(before=before):
                self.assertIn(before, original)
                page = service()
                page.body = original.replace(before, after)
                with self.assertRaises(verify.VerificationError):
                    verify.check_service(page, "en")

    def test_restrictive_meta_and_duplicate_http_headers(self):
        for directive in ("NOINDEX", "none", "nofollow", "nosnippet", "max-snippet: 0",
                          "unavailable_after: 01 Jan 2020 00:00:00 GMT"):
            with self.subTest(directive=directive):
                page = service()
                page.body = page.body.replace(b"index, follow", directive.encode())
                with self.assertRaisesRegex(verify.VerificationError, "robots meta"):
                    verify.check_service(page, "en")
                page = service(extra_headers=[("X-Robots-Tag", "index, follow"),
                                               ("X-Robots-Tag", f"googlebot: {directive}")])
                with self.assertRaisesRegex(verify.VerificationError, "X-Robots-Tag"):
                    verify.check_service(page, "en")

    def test_crawler_specific_meta_is_rejected(self):
        page = service()
        page.body = page.body.replace(b"<head>", b'<head><meta name="Googlebot" content="noindex">')
        with self.assertRaisesRegex(verify.VerificationError, "robots meta"):
            verify.check_service(page, "en")


class SitemapChecks(unittest.TestCase):
    def test_current_sitemap(self):
        verify.check_sitemap(sitemap())

    def test_missing_duplicate_extra_and_malformed_urls(self):
        original = sitemap().body
        entry = b"  <url>\n    <loc>https://qqrm.github.io/CV/services/en/</loc>\n  </url>\n"
        for body in (original.replace(entry, b""), original.replace(entry, entry + entry),
                     original.replace(b"services/en/", b"cv.html"), b"<urlset>",
                     original.replace(b"http://www.sitemaps.org/schemas/sitemap/0.9", b"urn:wrong")):
            with self.subTest(body=body):
                with self.assertRaises((verify.VerificationError, ElementTree.ParseError)):
                    verify.check_sitemap(sitemap(body))


class RootRobotsChecks(unittest.TestCase):
    def test_absent_empty_and_permissive_root_are_valid(self):
        for status, body in [(404, "Not found"), (410, "Gone"), (200, ""),
                             (200, "User-agent: *\nDisallow:\n")]:
            with self.subTest(status=status, body=body):
                verify.check_root_robots(response(verify.ROOT_ROBOTS, body, status))

    def test_server_errors_and_access_denials_are_not_absence(self):
        for status in (401, 403, 429, 500, 503):
            with self.subTest(status=status):
                with self.assertRaises(verify.VerificationError):
                    verify.check_root_robots(response(verify.ROOT_ROBOTS, status=status))

    def test_service_asset_or_search_crawler_blocks_fail(self):
        for text in ["User-agent: *\nDisallow: /CV/services/",
                     "User-agent: *\nDisallow: /CV/services/*.css$",
                     "User-agent: OAI-SearchBot\nDisallow: /\n"]:
            with self.subTest(text=text):
                with self.assertRaisesRegex(verify.VerificationError, "blocks"):
                    verify.check_root_robots(response(verify.ROOT_ROBOTS, text))

    def test_training_policy_is_independent_of_search_access(self):
        verify.check_root_robots(response(verify.ROOT_ROBOTS,
            "User-agent: GPTBot\nDisallow: /\nUser-agent: *\nAllow: /CV/\n"))

    def test_specific_search_groups_can_override_wildcard_deny(self):
        text = "User-agent: *\nDisallow: /\n"
        text += "\n".join(f"User-agent: {bot}\nAllow: /CV/\n" for bot in verify.CRAWLERS)
        verify.check_root_robots(response(verify.ROOT_ROBOTS, text))

    def test_unrelated_agent_substring_cannot_override_wildcard_deny(self):
        text = "User-agent: *\nDisallow: /\nUser-agent: bot\nAllow: /\n"
        with self.assertRaisesRegex(verify.VerificationError, "blocks"):
            verify.check_root_robots(response(verify.ROOT_ROBOTS, text))

    def test_longest_rule_and_allow_ties_are_order_independent(self):
        rules = ["Disallow: /", "Allow: /CV/", "Disallow: /CV/private/"]
        for ordered in (rules, list(reversed(rules))):
            text = "User-agent: *\n" + "\n".join(ordered)
            self.assertTrue(verify.robots_allowed(text, "Googlebot", "/CV/services/en/"))
            self.assertFalse(verify.robots_allowed(text, "Googlebot", "/CV/private/file"))
        text = "User-agent: *\nDisallow: /CV/\nAllow: /CV/"
        self.assertTrue(verify.robots_allowed(text, "Googlebot", "/CV/services/en/"))

    def test_matching_groups_merge_and_specific_group_beats_fallback(self):
        text = ("User-agent: *\nDisallow: /\n"
                "User-agent: googlebot\nAllow: /CV/\n"
                "User-agent: Googlebot\nDisallow: /CV/services/en/\n")
        self.assertTrue(verify.robots_allowed(text, "Googlebot", "/CV/services/ru/"))
        self.assertFalse(verify.robots_allowed(text, "Googlebot", "/CV/services/en/"))
        self.assertFalse(verify.robots_allowed(text, "Bingbot", "/CV/services/ru/"))

    def test_wildcards_end_anchors_and_percent_encoded_paths(self):
        text = "User-agent: *\nDisallow: /%43V/services/*/\nAllow: /CV/services/en/$"
        self.assertTrue(verify.robots_allowed(text, "Googlebot", "/CV/services/en/"))
        self.assertFalse(verify.robots_allowed(text, "Googlebot", "/CV/services/en/private"))
        self.assertFalse(verify.robots_allowed(text, "Googlebot", "/CV/services/ru/"))
        self.assertTrue(verify.robots_allowed("User-agent: *\nDisallow: /CV%2F", "Googlebot", "/CV/"))

    def test_wildcards_and_end_anchors_count_for_rule_specificity(self):
        for rules in ("Allow: /CV/\nDisallow: /*ru/",
                      "Allow: /CV/services/ru/\nDisallow: /CV/services/ru/$"):
            with self.subTest(rules=rules):
                self.assertFalse(verify.robots_allowed("User-agent: *\n" + rules,
                                                      "Googlebot", "/CV/services/ru/"))
        # Examples from Google's published rule-precedence contract.
        self.assertFalse(verify.robots_allowed("User-agent: *\nAllow: /page\nDisallow: /*.htm",
                                              "Googlebot", "/page.htm"))
        self.assertTrue(verify.robots_allowed("User-agent: *\nAllow: /page\nDisallow: /*.ph",
                                             "Googlebot", "/page.php5"))

    def test_user_agent_suffixes_and_ignored_fields_do_not_hide_blocks(self):
        for agent in ("Googlebot/2.1", "googlebot*"):
            with self.subTest(agent=agent):
                self.assertFalse(verify.robots_allowed(f"User-agent: {agent}\nDisallow: /",
                                                      "Googlebot", "/CV/services/en/"))
        for field in ("Crawl-delay: 10", "Request-rate: 1/10", "Sitemap: https://example.org/map.xml"):
            with self.subTest(field=field):
                text = f"User-agent: Googlebot\n{field}\nUser-agent: OtherBot\nDisallow: /"
                self.assertFalse(verify.robots_allowed(text, "Googlebot", "/CV/services/en/"))

    def test_root_soft_404_and_cross_origin_redirect_fail(self):
        with self.assertRaises(verify.VerificationError):
            verify.check_root_robots(response(verify.ROOT_ROBOTS, "<html>Not found</html>",
                                              content_type="text/html"))
        with self.assertRaises(verify.VerificationError):
            verify.check_root_robots(response("https://example.org/robots.txt", status=404))


class EndToEndChecks(unittest.TestCase):
    def verify_quietly(self, fetcher):
        with redirect_stdout(io.StringIO()):
            return verify.verify_once(fetcher)

    def test_all_existing_assets_and_new_services_are_checked(self):
        values = public_responses()
        self.assertEqual(self.verify_quietly(values.__getitem__), [])
        for missing in ("cv_ru.md", "Belyakov_en_light.pdf", "NotoSans-Regular.ttf",
                        "services/services.css"):
            with self.subTest(missing=missing):
                altered = dict(values)
                url = f"{verify.BASE}{missing}"
                altered[url] = response(url, status=404)
                self.assertTrue(self.verify_quietly(altered.__getitem__))

    def test_nested_robots_cannot_hide_effective_root_block(self):
        values = public_responses()
        values[verify.ROOT_ROBOTS] = response(verify.ROOT_ROBOTS, "User-agent: *\nDisallow: /")
        self.assertTrue(any("Root robots.txt blocks" in error
                            for error in self.verify_quietly(values.__getitem__)))

    def test_legacy_sitemap_server_error_is_not_success(self):
        values = public_responses()
        url = f"{verify.BASE}sitemap.txt"
        values[url] = response(url, status=503)
        self.assertTrue(any("sitemap.txt" in error for error in self.verify_quietly(values.__getitem__)))

    def test_transport_errors_are_returned_for_retry(self):
        values = public_responses()
        for error in (URLError("temporary timeout"), IncompleteRead(b"partial")):
            with self.subTest(error=error):
                def flaky(url):
                    if url == verify.SERVICES["ru"]:
                        raise error
                    return values[url]
                self.assertTrue(self.verify_quietly(flaky))

    def test_retry_recovers_and_exhaustion_fails(self):
        for outcomes, expected in [([["HTTP 404"], []], 0), ([["HTTP 404"], ["HTTP 404"]], 1)]:
            with self.subTest(outcomes=outcomes), patch.object(verify, "ATTEMPTS", 2), \
                    patch.object(verify, "verify_once", side_effect=outcomes) as attempt, \
                    patch.object(verify.time, "sleep") as sleep, \
                    redirect_stdout(io.StringIO()), redirect_stderr(io.StringIO()):
                self.assertEqual(verify.main(), expected)
                self.assertEqual(attempt.call_count, 2)
                sleep.assert_called_once_with(verify.RETRY_DELAY)

    def test_network_target_and_redirects_stay_on_fixed_origin(self):
        with self.assertRaises(verify.VerificationError):
            verify.fetch("https://example.org/")
        handler = verify.SameOriginRedirects()
        for target in ("https://example.org/", "http://qqrm.github.io/robots.txt"):
            with self.subTest(target=target), self.assertRaises(verify.VerificationError):
                handler.redirect_request(Request(verify.ROOT_ROBOTS), None, 302, "Found", Message(), target)


if __name__ == "__main__":
    unittest.main()
