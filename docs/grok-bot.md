# Grok Bot routines

GitHub Actions does everything that a fixed program can do (markets, news feeds, Wikipedia, the
model, the site; see [operations.md](operations.md)). Grok Bot does every job that needs an agent,
with no person in the loop: measuring X, entering polls from pollsters' publications, checking
each poll blind before it counts, following the campaign, and changing the field of candidates
when a fact is confirmed. This page is what to set up in Grok Bot (one Bot, one secret, four
routines) and, below the setup, the full instruction of each routine. The routines fetch this page
at run time and follow its "Shared preamble" and their own section, so a change merged here
reaches the next run without touching Grok Bot.

## How Grok Bot talks to the repository

Grok Bot never gets a GitHub credential. GitHub cannot limit a token to one folder of a public
repository, and every Bot on a Grok Bot account shares one computer, so a token held there would be
readable by all of them. Instead:

1. A routine POSTs what it found to the **inbox**, a small endpoint on the project's Cloudflare
   Worker (`app/worker`), with a bearer token that can only add items to the inbox.
2. Every hour the Live workflow pulls the inbox and checks each item with the same code that
   checks every file in the repository:
   - X measurements (`x_drop`) and campaign events (`event`) that pass are committed to
     `data/raw/grok/` and `data/raw/events/`;
   - polls (`poll`) that pass go into a pull request titled "Polls from Grok Bot: …" (status
     `proposed`). A poll moves the forecast, so it counts only once a **blind check** agrees;
   - poll checks (`poll_check`) are a second entry of a proposed poll, made from the notice or
     publication by another routine run without reading the proposal's numbers. When the check
     agrees with the proposal on the poll id, `supersedes`, fieldwork dates, sample size and every
     scenario's round, candidates and shares, Live commits the proposed file to `data/raw/polls/`
     unchanged, refits, and closes the pull request; both items become `accepted`. When anything
     differs, both items become `rejected`, with notes (and a comment on the pull request) that
     name which fields and which candidates' shares differ, never a value from either entry; the
     PDF is the reference. The proposal stays open for another check;
   - field changes (`field_change`) are sourced changes to `config/candidates.yaml`. Live applies
     them to the blocks they touch (comments and everything else in the file stay as they are),
     records them in `research/LOG.md`, validates the field (every candidate in exactly one slot,
     each slot's options summing to at most 1, the schema), and commits them, or rejects the whole
     change with the reasons;
   - anything that fails is rejected, with the reasons.
3. The routine reads those outcomes back from the inbox at its next run (each row carries the
   item's `key`, since rows have no payload), and resends corrected items when something was
   rejected. The inbox refuses, with 409, an item whose kind and key match an accepted or
   proposed one.

**Same-run rule.** Every `poll`, `poll_check` and `field_change` carries
`agent: {name, skill_version, run_id}`, where `run_id` is the routine's name and the UTC minute
its run started (`polls-2026-10-16T07:30Z`). Live refuses a check whose `run_id` equals the
proposal's, so a poll and its check never come from the same run. Routine 2 proposes polls;
routine 4 checks them, and the polls routine 4 itself proposes (late notices) are checked by a
later run of routine 4, or are refused until then.

The model does not use X measurements or events yet. They are recorded and shown on the Signals
page so their value can be measured against later polls before they get any weight. Field changes
do change the forecast: the field is the first step of every simulation.

The payload contracts are the JSON Schemas in [`schemas/`](../schemas): `grok_drop`, `poll`,
`event`, `poll_check` and `field_change`.

## One-time setup

1. Create a Bot, for example **fr2027**, on a Grok Bot account. Use a dedicated account if you
   run Bots for other work, since Bots on one account share their computer and credentials.
2. In the Bot's **Secrets**, add `INBOX_BOT_TOKEN` with the same value as the GitHub secret of
   that name (see [operations.md](operations.md)). Grok Bot does not show secret values to the
   Bot; it is used as an environment variable in shell commands.
3. In **Approvals**, add "Allow automatically" rules for running `curl` against
   `https://fr2027.hebilicious.workers.dev/inbox/` (the inbox), `https://api.github.com/repos/Hebilicious/fr2027-forecaster/`
   and `https://raw.githubusercontent.com/Hebilicious/fr2027-forecaster/` (reading the
   repository). Work that a routine starts while you are away waits about ten minutes for an
   approval and is then dropped, so without these rules nothing would ever be sent.
4. Create the four routines with the schedules below. Each routine's prompt only says to fetch
   https://raw.githubusercontent.com/Hebilicious/fr2027-forecaster/main/docs/grok-bot.md and
   follow the section "Shared preamble", then the routine's own section (for example
   "Routine 2: new polls").
5. Press **Test** on each routine once and watch the result in the Bot's chat. Then check
   `GET <INBOX>/inbox/items`; a test item will be resolved (accepted, proposed or rejected) within
   the hour.

| Routine                          | Schedule                      | Sends                                |
| -------------------------------- | ----------------------------- | ------------------------------------ |
| 1. X activity                    | 00:20 and 12:20 UTC           | `x_drop`                             |
| 2. New polls                     | 09:30 and 21:30, Europe/Paris | `poll`, `event`                      |
| 3. Campaign events               | 09:00 and 21:00, Europe/Paris | `event`                              |
| 4. Poll checks and field changes | 10:30 and 22:30, Europe/Paris | `poll_check`, `poll`, `field_change` |

Paris times are local: set the routines in the Europe/Paris time zone and the Bot converts them
across the change between summer and winter time (UTC+2 and UTC+1). Routine 4 runs an hour after
each routine 2 run, so a poll proposed at 09:30 is in a pull request (Live runs every hour, at
:17) by the 10:30 check.

## Shared preamble

Every routine follows this first.

```text
PROJECT. fr2027-forecaster, a public forecast of the 2027 French presidential election:
https://github.com/Hebilicious/fr2027-forecaster. Nobody reviews your work by hand. The
pipeline checks every item, and a poll counts only when another run's blind entry agrees.
Be exact.

INBOX. <INBOX> = https://fr2027.hebilicious.workers.dev (a different address in your setup
prompt wins).

IDS. Use only candidate ids from
https://raw.githubusercontent.com/Hebilicious/fr2027-forecaster/main/config/candidates.yaml
(its header explains slots), matched on the names in
https://raw.githubusercontent.com/Hebilicious/fr2027-forecaster/main/config/sources.yaml.

RUN_ID. At the start, set RUN_ID = <routine slug>-<UTC minute the run started>: x-activity-,
polls-, events- or checks-, e.g. polls-2026-10-16T07:30Z. Every "agent" in this run is
{"name": "Grok Bot fr2027", "skill_version": "<your routine's skill version>", "run_id": RUN_ID}.

1. READ THE INBOX FIRST:
   curl -s -H "Authorization: Bearer $INBOX_BOT_TOKEN" <INBOX>/inbox/items
   Each row (last 30 days, every routine) has id, kind, key, status, note, ref. key is what the
   item is about: poll_id (poll, poll_check), "<window.start>/<window.end>" (x_drop), event_id
   (event), change_id (field_change). status is accepted, proposed (a poll waiting for its blind
   check; it later becomes accepted or rejected), rejected (note gives the reasons) or pending
   (not processed yet).
2. DEDUPE ON kind + key. Do not send an item whose kind and key already have a row that is
   accepted, proposed or pending. A rejected one may be fixed and resent with the same key, as
   your routine's section says.
3. SEND: POST <INBOX>/inbox/items with headers "Authorization: Bearer $INBOX_BOT_TOKEN" and
   "Content-Type: application/json", body {"kind": "<x_drop | poll | event | poll_check |
   field_change>", "payload": {...}}, using curl in the shell.
   - 201 {"id", "status": "pending"}: received, not yet accepted.
   - 409 {"error", "id"}: an accepted or proposed item already has this kind and key. Do not
     resend; that item stands.
   - Any other error: read the message, fix the body, retry once.
   Never print the token.

RULES. Report only what you read or measured; use null for anything you cannot measure, and
never present an estimate as measured. A regional outlet carrying an AFP or Reuters wire
counts as a national source. Find official documents (Commission des sondages
notices, Conseil constitutionnel decisions, court rulings) through a search or the Wayback
Machine, never by trying URLs one after another on a government site. Never include X post
ids, user names or any secret. Times are UTC like 2026-10-09T18:00:00Z; dates YYYY-MM-DD.
Neutral, factual English.
```

## Routine 1: X activity

**Schedule:** 00:20 and 12:20 UTC. Skill version `x-activity-2`.

```text
1. WINDOW: the last full 12 hours, ending at the most recent 00:00 or 12:00 UTC. Its key is
   "<start>/<end>". If the inbox has an x_drop with that key, stop.
2. CANDIDATES: every id in candidates.yaml whose status is not "withdrawn" or "ineligible".
3. For each candidate, search X for posts in the window with
   query = (<each name from sources.yaml, in quotes, joined with OR>) lang:fr
   French-language posts only; every query ends with lang:fr. Record:
   - query: the exact search you ran, lang:fr included.
   - mentions: posts matching the query in the window, for every candidate, from X's counts
     endpoint: mentions_method "x_counts_endpoint". If the counts endpoint fails, your native X
     count ("native_count"), else an estimate from what you read ("sample_estimate"), else
     mentions null.
   - sample_size: read 25 posts (mixing most recent and most engaged) only for the 10
     candidates with the most mentions in the window. Every other candidate gets sample_size 0,
     sentiment null and top_topics [].
   - sentiment: among the posts read, fractions positive, negative and neutral toward the
     candidate (pos + neg + neu = 1), method "grok-bot reading <N> posts, routine v2"; null if
     you read fewer than 20 or none were sampled.
   - top_topics: up to 5 short topics, in English.
   - bot_share_estimate: fraction of posts read from accounts that look automated or
     coordinated (0 to 1), or null.
   - unique_authors, engagement: null unless measured over every post in the window.
   No post ids, user names or post text.
4. POST one item:
   {"kind": "x_drop", "payload": {
     "schema_version": "1.0",
     "collected_at": "<now>",
     "window": {"start": "<window start>", "end": "<window end>"},
     "query_method": "Grok Bot native X search, one query per candidate, lang:fr, routine v2",
     "agent": {"name": "Grok Bot fr2027", "skill_version": "x-activity-2", "run_id": RUN_ID},
     "candidates": [{"candidate_id": "...", "query": "... lang:fr", "mentions": ...,
                     "mentions_method": "...", "sample_size": ..., "unique_authors": null,
                     "engagement": null,
                     "sentiment": {"pos": ..., "neg": ..., "neu": ..., "method": "..."} or null,
                     "top_topics": [...], "bot_share_estimate": ...}, ...]}}
```

The contract is [`schemas/grok_drop.schema.json`](../schemas/grok_drop.schema.json). The pipeline
drops any `sample_post_ids`, so that a post deleted on X never lives on in the public repository.

**Cost:** two runs a day, each about 40 X calls: one count per candidate, plus 25 posts read for
each of the 10 most-mentioned candidates (about 500 posts a day). Grok Bot's docs warn that frequent routines can
use a week's allowance in a day; if it still runs low, run once a day at 00:20 UTC with a 24-hour
window. The pipeline accepts any window length.

## Routine 2: new polls

**Schedule:** 09:30 and 21:30, Europe/Paris. Skill version `polls-2`.

```text
1. FIND voting-intention polls for the 2027 presidential election published since the newest
   file in https://github.com/Hebilicious/fr2027-forecaster/tree/main/data/raw/polls. Look in:
   the Commission des sondages notices (https://www.commission-des-sondages.fr/notices/), which
   every published election poll must have; the pollsters' own sites (Ifop, Elabe, Odoxa,
   OpinionWay, Ipsos BVA, Toluna Harris Interactive, Cluster17, Verian, YouGov, CSA); and
   https://fr.wikipedia.org/wiki/Liste_de_sondages_sur_l%27%C3%A9lection_pr%C3%A9sidentielle_fran%C3%A7aise_de_2027
   as an index only.
2. SKIP a poll when:
   - the inbox already has a poll with its poll_id that is accepted, proposed or pending;
   - its notice supersedes a data/raw/polls file whose name has no five-digit notice number
     (e.g. "opinionway-2026-10-14-cnews"): routine 4(b) owns those;
   - a scenario tests someone with no id in candidates.yaml. Send an event instead (kind
     "other", format in routine 3) saying who was tested in which poll; routine 4 adds them, and
     you send the poll once they are in the list.
3. ENTER each remaining poll from the Commission's notice or the pollster's publication (the
   PDF), never from an article or Wikipedia. Commission PDFs often time out: if one won't load,
   use its Wayback Machine copy (https://web.archive.org/web/<notice PDF URL>), then the
   pollster's own PDF; entry stays "primary". Payload fields, exactly (open a file in
   data/raw/polls for the format):
   - schema_version "1.0"
   - poll_id "<firm>-<field_end>-<notice number>", e.g. "ifop-2026-09-29-10284"; without a
     notice number yet "<firm>-<field_end>-<sponsor slug>"
   - firm: lowercase slug (ifop, elabe, odoxa, opinionway, ipsos-bva, toluna-harris, cluster17,
     verian, yougov, csa, ...)
   - sponsor (or null); field_start; field_end; published_at (the day the results were first
     published); sample_size (the total sample)
   - method: online, phone or mixed
   - population: registered, or likely when the published figures are computed on voters
     certain to vote
   - source_url (the PDF you read); notice_url (the Commission's notice, or null); index_url
     (or null); retrieved_at (now)
   - entry: "primary" if you read the numbers in the notice or the pollster's own publication,
     otherwise "index"
   - notes: one or two sentences a reader needs, such as how blank votes were treated
   - agent: {"name": "Grok Bot fr2027", "skill_version": "polls-2", "run_id": RUN_ID}
   - scenarios: one per ballot tested, {"scenario_id": "S1", "round": 1, "label": "<who was
     tested>", "shares": {"<candidate id>": <percent of expressed votes>}}. Round 2
     head-to-heads: round 2, exactly two candidates, ids "H1", "H2", ... Write "<0.5" or "<1"
     as 0.5 and a dash as 0. Round 1 shares add up to 85–103, round 2 to 98–102.
4. POST {"kind": "poll", "payload": {...}}. Never send a poll_check: checks come from
   routine 4, in another run.
5. A REJECTED POLL whose note says a blind check differs names the differing fields and
   candidates, without values. Re-read exactly those figures in the PDF. Wrong: resend the
   corrected poll with the same poll_id (it replaces the open proposal and waits for a new
   check). Right: do nothing; the next check settles it.
```

The pipeline proposes each valid poll in a pull request titled "Polls from Grok Bot: …", whose
description lists each poll's id, PDF links and the run that entered it, but none of its numbers.
Routine 4 enters each one again from the PDF; Live commits it when both agree.

## Routine 3: campaign events

**Schedule:** 09:00 and 21:00, Europe/Paris. Skill version `events-1`.

```text
1. FIND events of the last 24 hours that change who will be on the ballot or how the field
   looks: declarations and withdrawals of candidacy; endorsements of one candidate by another
   or by a party; primary results; sponsorship (parrainage) counts and the official list from
   the Conseil constitutionnel (from early 2027); court rulings on a candidate's eligibility
   (e.g. Marine Le Pen's appeal to the Cour de cassation); the publication of a major poll
   (kind "poll_published").
2. KEEP only events confirmed in at least one primary source or national outlet, whose
   event_id is not in https://github.com/Hebilicious/fr2027-forecaster/tree/main/data/raw/events
   and not an accepted, proposed or pending key in the inbox. A regional outlet carrying an AFP
   or Reuters wire counts as a national outlet. Nothing left: send nothing.
   DECLARATIONS BY SOMEONE NOT IN candidates.yaml: send a kind "other" event proposing they be
   added (proposed_change "add_candidate <name>: <party>, <slot>"), even if the declaration is
   up to 7 days old, unless the inbox or data/raw/events already has it.
3. POST each as {"kind": "event", "payload": {
     "schema_version": "1.0",
     "event_id": "<date>-<short-slug>", e.g. "2026-10-17-faure-withdraws",
     "date": "<YYYY-MM-DD, when it happened, Paris time>",
     "kind": "declaration" | "withdrawal" | "endorsement" | "primary_result" | "sponsorships" | "legal" | "poll_published" | "other",
     "candidate_ids": [ids involved, only ids in candidates.yaml; may be empty],
     "summary": "<one or two neutral sentences, at most 400 characters; name anyone without an id here>",
     "sources": [{"url": "<primary source or a national outlet>", "title": "<headline>"}],
     "proposed_change": "<optional: the change to config/candidates.yaml this suggests, e.g. 'faure: status withdrawn; ps_primary slot: faure 0', or null>",
     "reported_by": "Grok Bot fr2027",
     "reported_at": "<now>"}}
```

Events are shown on the Signals page as reported. Routine 4 turns the confirmed ones into field
changes.

## Routine 4: poll checks and field changes

**Schedule:** 10:30 and 22:30, Europe/Paris, an hour after each routine 2 run. Skill version
`checks-1`. Do (a), (b), (c) in order.

```text
Every payload in this run carries
"agent": {"name": "Grok Bot fr2027", "skill_version": "checks-1", "run_id": RUN_ID}.

(a) BLIND CHECKS OF PROPOSED POLLS.
1. List open pull requests:
   https://api.github.com/repos/Hebilicious/fr2027-forecaster/pulls?state=open&per_page=100
   Keep those whose head.ref starts with "inbox/polls-".
2. Read ONLY the table in each body: poll_id, source_url, notice_url, supersedes, entered by
   run. Never open the pull request's files, diff, commits or branch, nor data/proposals/, and
   never look the numbers up anywhere else (articles, Wikipedia, X). The check is worth
   something only if you enter the poll yourself from the source.
3. Skip a poll whose "entered by run" is your RUN_ID, and one whose poll_check key is already
   accepted, proposed or pending in the inbox.
4. PICK THE OTHER PRIMARY DOCUMENT where both exist:
   - source_url is the notice (same as notice_url): enter from the pollster's own report.
   - source_url is the pollster's report: enter from the notice.
   - Only one exists: enter from it.
   Say which one in notes, e.g. "Checked against the pollster's report (the proposal read the
   notice)." Set source_url to the document you read.
5. Enter the whole poll as routine 2 step 3 does (every field), with the same poll_id, the
   supersedes value from the table (omit it when the table says none), and your agent. POST it
   as {"kind": "poll_check", "payload": {...}}.
6. An earlier check of yours rejected because it differs names the differing fields and
   candidates, without values. Re-read exactly those figures in your document and send a new
   check with what it says.

(b) LATE NOTICES.
1. In https://github.com/Hebilicious/fr2027-forecaster/tree/main/data/raw/polls find the files
   whose name does not end in a five-digit notice number, e.g. "opinionway-2026-10-14-cnews".
2. For each, look for its Commission des sondages notice (same firm, sponsor and fieldwork).
3. If it is filed and no open pull request or inbox row already has the new poll_id, enter the
   poll from the notice as routine 2 step 3 does, with poll_id "<firm>-<field_end>-<notice
   number>", "supersedes": "<the old poll_id>", entry "primary", notice_url the notice, and
   your agent. POST it as {"kind": "poll", ...}. A later run checks it blind.

(c) FIELD CHANGES.
1. Read config/candidates.yaml (the current field), the events of the last 7 days in
   https://github.com/Hebilicious/fr2027-forecaster/tree/main/data/raw/events, and the inbox
   rows (events, and field changes with their outcomes).
2. For each change in who runs that at least one primary source or national outlet confirms
   (a declaration, a withdrawal, a primary result, an endorsement that ends a candidacy, a
   ruling on eligibility, the official list), send one field change. Rules:
   - Change a slot's probabilities only for a confirmed fact, never on rumour, polls or markets.
   - Every candidate stays in exactly one slot. A candidate who loses or withdraws stays in
     their slot with p 0. A slot's options sum to at most 1 (1 minus the sum is the chance
     nobody from the slot runs).
   - A candidate tested in a poll but missing from candidates.yaml (see routine 2's "other"
     events) gets add_candidate, with a sourced entry, the names that identify them in
     headlines, and a slot.
   - Dedupe on change_id. A rejected one: fix what the note says and resend with the same
     change_id.
3. POST {"kind": "field_change", "payload": {
     "schema_version": "1.0",
     "change_id": "<YYYY-MM-DD>-<short-slug>", e.g. "2026-10-17-faure-withdraws",
     "event_ids": [ids of accepted events in data/raw/events/ this change relies on; may be empty],
     "sources": [{"url": "<primary source or national outlet>", "title": "<headline>"}, ...],
     "summary": "<one or two neutral sentences, at most 400 characters, on one line>",
     "log": "<one short paragraph for research/LOG.md, on one line: what changed, why, and what the sources say>",
     "agent": {...},
     "operations": [one or more of:
       {"op": "set_status", "candidate_id": "faure", "status": "declared | likely | possible | withdrawn | ineligible",
        "notes": "<optional: replaces the candidate's notes>", "declared_on": "<optional YYYY-MM-DD>",
        "source": {"url": "...", "date": "YYYY-MM-DD", "note": "<optional>"}},
       {"op": "set_slot", "slot_id": "ps_primary",
        "options": [{"run": ["glucksmann"], "p": 0.95}, {"run": ["faure"], "p": 0}, ...every option, including those at 0],
        "rationale": "<why these probabilities, with dates>"},
       {"op": "add_candidate",
        "candidate": {"id": "<lowercase_id>", "name": "...", "party": "...", "bloc": "far_left | left | centre | right | far_right | other",
                      "status": "...", "declared_on": "<optional>", "notes": "<optional>",
                      "sources": [{"url": "...", "date": "YYYY-MM-DD"}]},
        "names": ["<full name>", "<surname if it is not a common word>"],
        "wikipedia": "<French Wikipedia article title with underscores, or null>",
        "slot": {"slot_id": "<an existing slot, or a new id>", "p": <0 to 1>,
                 "rationale": "<replaces the slot's rationale>", "label": "<optional>"}}
     ]}}
   set_slot replaces a slot's whole options list and rationale, so list every option. Text
   fields hold no line breaks. One field change per fact; nothing confirmed: send nothing.
```

Live checks the field change as a whole and rejects it with every reason (an unknown candidate or
slot, a slot that would lose a candidate or sum past 1, an event id not in `data/raw/events/`,
a schema error), or applies it: the changed candidate and slot blocks, `as_of`, a line in
`config/sources.yaml` for a new candidate, and an entry in `research/LOG.md` marked
"via Grok Bot field_change `<change_id>`", committed with the next data commit.

## Checking that it works

- `curl -s -H "Authorization: Bearer $INBOX_BOT_TOKEN" <INBOX>/inbox/items` lists each item and
  its outcome.
- The open "Polls from Grok Bot" pull requests carry a comment for every check: the differences,
  or the commit that landed the poll.
- The Sources page shows when the last X drop and event arrived, and warns when X data is more
  than 48 hours old.
- The Live workflow's log shows each item it pulled, and the reason for each rejection.

## What is not known yet

These open points come from Grok Bot's documentation as read on 2026-10-09:

- Grok Bot's docs say a Bot can "count recent posts on a topic". They don't say whether the count
  is complete or sampled, or how far back it reaches. That is why each drop records
  `mentions_method`, and why X numbers carry no weight until they are checked. If complete counts
  matter, X's own counts endpoint (`/2/tweets/counts/recent`) gives them; routine 1 now uses it
  (`x_counts_endpoint`) and falls back to the native count.
- The docs imply but don't state that a Bot secret is available as an environment variable in
  the shell. The Test run in step 5 settles it. If it isn't, the token can sit in a file under
  /workspace instead. Every Bot on the account can read that file, which is one more reason to
  use a dedicated account.
- Whether a routine keeps memory between runs. The blind check relies on routine 4 entering each
  poll from the PDF; the pipeline enforces only that the check comes from another run.
- The proposal and its check come from the same agent, so their errors can be correlated: the
  same misreading of a table, or the same convention applied wrongly, would pass the check.
  Entering the check from the other primary document (routine 4(a) step 4) reduces this, but
  does not remove it.
- The cost of a run is not published. Watch the weekly usage after the first day.
