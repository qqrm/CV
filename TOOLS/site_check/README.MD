# site_check

This tool crawls the deployed CV website and validates pages and PDF documents.

## PDF checks

The checker ensures that each PDF:

- is a valid PDF with at least one page;
- contains the candidate's name in the appropriate language;
- includes every company name listed in the corresponding `CV.MD` or `CV_RU.MD`.

Missing items are reported as errors in the log.
