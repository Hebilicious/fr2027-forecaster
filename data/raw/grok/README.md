# Grok drop folder

The Grok bot writes one JSON file per collection window here, named `YYYY-MM-DDTHH-MM.json` (window end, UTC).

- Format: see "Data contracts → Grok drop" in `/SPEC.md`; the schema is `/schemas/grok_drop.schema.json`, and `moon run cli:validate` checks every drop against it.
- Never edit a file once pushed; send a new window instead.
- Commit message: `grok: <window end>`.
- Use candidate IDs from `/config/candidates.yaml`; send `null` for anything you can't measure.
