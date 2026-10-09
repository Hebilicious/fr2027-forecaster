# Grok drop folder

The Grok bot writes one JSON file per collection window here, named `YYYY-MM-DDTHH-MM.json` (window end, UTC).

- Format: see "Data contracts → Grok drop" in `/SPEC.md` (schema will live in `/schemas/grok_drop.schema.json`).
- Never edit a file once pushed; send a new window instead.
- Commit message: `grok: <window end>`.
- Use candidate IDs from `/config/candidates.yaml`; send `null` for anything you can't measure.
