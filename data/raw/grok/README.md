# Grok drop folder

One JSON file per collection window of X activity, named `YYYY-MM-DDTHH-MM.json` (window end, UTC).

- Grok Bot doesn't write here directly. It sends each drop to the inbox Worker; the hourly Live
  workflow checks it and commits it here (see `/docs/grok-bot.md`).
- Format: `/schemas/grok_drop.schema.json`. `moon run cli:validate` checks every file against it.
- Post ids (`sample_post_ids`) are dropped before a file is committed, so a post deleted on X
  never lives on in this public repository.
- A file is never edited once committed; a later window is a new file.
