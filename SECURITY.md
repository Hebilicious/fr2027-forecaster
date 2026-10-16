# Security policy

fr2027-forecaster runs itself: a Cloudflare Worker serves the public page and the Grok Bot inbox,
and an hourly GitHub Actions workflow collects, forecasts and publishes. Reports about the Worker,
the site, the pipeline or the workflows are welcome.

## Reporting a vulnerability

Use GitHub's private vulnerability reporting: **Security → Report a vulnerability** on
[this repository](https://github.com/Hebilicious/fr2027-forecaster/security/advisories/new). That
keeps the report private until a fix is out. Please do not open a public issue for a security
problem.

Include what you need to make the report reproducible: the URL or file, the request or payload,
and what you expected instead. A minimal reproduction is worth more than a long description.

## What is in scope

- The Worker `fr2027` and its `/inbox/*` endpoints.
- The public site and its `/api/*.json` documents.
- The Rust pipeline and the data it writes to this repository.
- The GitHub Actions workflows.

## What is not a vulnerability

- The published forecast being wrong, or polls and campaign events it uses being inaccurate: the
  numbers are the product, not a security boundary. **The one exception is data that reached the
  repository without going through the inbox or a pull request; report that.**
- Anything that requires holding `INBOX_BOT_TOKEN`, `INBOX_PIPELINE_TOKEN`, a repository secret, a
  Cloudflare credential or write access to this repository, unless you obtained it in a way that
  should not have been possible.
- Missing security headers or configuration suggestions with no demonstrated impact: open a normal
  issue or a pull request instead.

## What the project does about it

Fixes go through a pull request on a `security-*` branch, and the branch is public like the rest of
the repository once it is pushed. Please say in the report whether the advisory may be published
after the fix lands; the default is to publish it once the fix is on `main`.
