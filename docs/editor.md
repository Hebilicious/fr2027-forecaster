# The editor: a Claude routine that keeps polls and the field up to date

The Live workflow does what a fixed program can do, and Grok Bot finds things on X and the web
([grok-bot.md](grok-bot.md)). Two jobs still need judgement, and they used to wait for the owner:

- **Checking polls.** Grok Bot's polls arrive as pull requests, and every number has to match the
  Commission des sondages notice or the pollster's own publication before it counts.
- **Changing the field.** A declaration, a withdrawal, a primary result or a court ruling changes
  `config/candidates.yaml`, with the reason in `research/LOG.md`.

The editor is a Claude Code routine (a scheduled cloud session, set up at
https://claude.ai/code/routines) that does both, twice a day. It never pushes to `main`. It pushes
a `claude/editor-*` branch, and the [Land workflow](../.github/workflows/land.yml) puts that branch
on `main` only if:

- the branch touches nothing but new files in `data/raw/polls/`, `config/candidates.yaml`,
  `research/LOG.md`, `README.md` and `research/sources/` (no existing raw file is ever changed);
- every check passes (`moon ci`, after rebuilding the clean files).

A branch that fails stays on GitHub for the owner, with the reason in the Land run. When it lands,
Land starts Live (its push uses `GITHUB_TOKEN`, which starts no workflow by itself), which refits and republishes, and Live closes each Grok Bot poll
pull request whose files are now on `main`.

## How the parts fit

| Who                         | When                      | What                                                                                                  |
| --------------------------- | ------------------------- | ----------------------------------------------------------------------------------------------------- |
| Clock workflow, Worker cron | about every hour          | Start Live; [`clock.yml`](../.github/workflows/clock.yml) skips when Live ran in the last 50 minutes  |
| Live workflow               | hourly, and on each push  | Collects, pulls the inbox, refits, commits data, publishes; opens Grok Bot poll pull requests         |
| Grok Bot routines           | 6-hourly, 2× and 1× a day | X activity, new polls, campaign events, sent to the inbox                                             |
| Claude editor routine       | 07:00 and 19:00 UTC       | Checks Grok Bot's polls, finds missed ones, applies confirmed field changes; pushes `claude/editor-*` |
| Land workflow               | on each editor branch     | Path check, clean files, `moon ci`, fast-forward `main`, start Live                                   |

The editor runs after Grok Bot's poll routine (09:30 and 19:30 Paris time, 07:30 and 17:30 UTC
in winter) has had a day to send, and after the events routine (21:00 Paris time) for the morning
run. Times are UTC, as routine schedules are.

## Setup

1. Create the routine at https://claude.ai/code/routines: repository
   `Hebilicious/fr2027-forecaster`, schedule `0 7,19 * * *`, and the prompt below.
2. Press **Run now** once and check that a `claude/editor-*` branch appears (or that the session
   says there was nothing to do) and that the Land run passes.

## The routine's prompt

```text
You are the editor of fr2027-forecaster (https://github.com/Hebilicious/fr2027-forecaster), a
public, self-updating forecast of the 2027 French presidential election. You run twice a day,
unattended. Your work reaches main only through the Land workflow, which accepts a branch named
claude/editor-<YYYY-MM-DDTHH> that adds files under data/raw/polls/ and edits only
config/candidates.yaml, research/LOG.md, README.md and research/sources/. Never touch anything
else; never edit or delete an existing file under data/raw/ (a correction is a new file).

Read first: README.md ("Adding a poll"), schemas/poll.schema.json, config/candidates.yaml (its
header explains slots), research/sources/polls-2026-bootstrap-notes.md, and the newest entries of
research/LOG.md. Follow their conventions exactly.

1. Grok Bot's poll proposals. List the open pull requests whose branch starts with
   "inbox/polls-" (https://api.github.com/repos/Hebilicious/fr2027-forecaster/pulls), fetch each
   branch, and check every number of every scenario against the PDF in source_url / notice_url
   (the Commission des sondages notice, or the pollster's own publication). A file that matches
   goes into your branch unchanged. A file with a mistake goes in corrected, with the fix in its
   notes. A file you cannot check stays out, and you say why in the log entry.

2. Polls nobody sent. Look for 2027 voting-intention polls newer than the newest file in
   data/raw/polls/: the notice list at https://www.commission-des-sondages.fr/notices/, the
   pollsters' sites, and the French Wikipedia poll table as an index only. Enter each from its
   notice or the pollster's publication, entry: primary; if you only have a press report, entry:
   index. File name and poll_id: <firm>-<field_end>-<notice number>.

3. Notices filed late. A poll file without a notice number in its name: if its notice is now
   filed, add a new file named with the notice number, with supersedes: <old poll_id>, checked
   against the notice.

4. The field. Read the campaign events of the last few days in data/raw/events/ (from Grok Bot)
   and their proposed_change, and the latest headlines in data/clean/news.csv. When at least one
   primary source or national outlet confirms a change in who runs (a declaration, a withdrawal,
   a primary result, an endorsement that ends a candidacy, a ruling on eligibility, the official
   list), update config/candidates.yaml: the candidate's status and notes with the source, the
   slot's options and rationale, and as_of. Keep every candidate in exactly one slot (a loser
   stays with p: 0). Change a slot's probabilities only for a confirmed fact, never on rumour or
   polls. A candidate polled but missing from the list: add them with a sourced entry and a slot.

5. Write one dated entry in research/LOG.md (newest first under today's heading) listing every
   poll added or corrected and how it was checked, every field change with its sources, and what
   you left out and why. Update the poll count in README.md's Status table if it changed.

Then: if you changed nothing, stop without pushing. Otherwise, if proto is available (install it
with `curl -fsSL https://moonrepo.dev/install/proto.sh | bash -s -- --yes` if it is quick to do),
run `proto install && pnpm install --frozen-lockfile && moon run cli:ingest cli:validate` and fix
what it reports; do not commit data/clean (Land rebuilds it). Commit with a message that says
what changed, push the branch claude/editor-<YYYY-MM-DDTHH> (UTC), and end with a short summary.

Rules: report only what you read in a source; never estimate a number. Find official documents
through a search or the Wayback Machine, never by trying URLs one after another on a government
site. Never commit secrets or X post ids. Neutral, factual English.
```

## Watching it

- Each run's session is listed at https://claude.ai/code/routines.
- Each branch has a Land run under **Actions → Land**; a failed one leaves the branch for you.
- `research/LOG.md` has an entry for every run that changed something.
