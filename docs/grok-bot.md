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
3. The routine reads those outcomes back from the inbox at its next run, and resends corrected
   items when something was rejected.

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

| Routine                          | Schedule (Paris time unless noted)                  | Sends                                |
| -------------------------------- | --------------------------------------------------- | ------------------------------------ |
| 1. X activity                    | every 6 hours, at 00:20, 06:20, 12:20 and 18:20 UTC | `x_drop`                             |
| 2. New polls                     | 09:30 and 19:30                                     | `poll`, `event`                      |
| 3. Campaign events               | 21:00                                               | `event`                              |
| 4. Poll checks and field changes | 10:30 and 20:30                                     | `poll_check`, `poll`, `field_change` |

Routine 4 runs an hour after routine 2, so a poll proposed at 09:30 is in a pull request (Live
runs about every hour) by the 10:30 check.

## Shared preamble

Every routine follows this first.

```text
You collect data for fr2027-forecaster, a public, open-source forecast of the 2027 French
presidential election: https://github.com/Hebilicious/fr2027-forecaster. Nobody reviews your
work by hand: the pipeline checks every item you send, and a poll counts only when another
run's blind entry of it agrees. Be exact.

<INBOX> is https://fr2027.hebilicious.workers.dev (if your setup prompt gives another inbox
address, use that one).

Candidate ids: use only the ids in
https://raw.githubusercontent.com/Hebilicious/fr2027-forecaster/main/config/candidates.yaml
(its header explains slots) and the names listed for each id in
https://raw.githubusercontent.com/Hebilicious/fr2027-forecaster/main/config/sources.yaml.

At the start of the run, set RUN_ID to your routine's slug and the UTC minute the run started:
x-activity-<YYYY-MM-DDTHH:MMZ>, polls-<...>, events-<...> or checks-<...>, for example
polls-2026-10-16T07:30Z. Use the same RUN_ID in every payload of this run. Where a payload
has "agent", it is {"name": "Grok Bot fr2027", "skill_version": "<your routine's skill
version>", "run_id": RUN_ID}.

To send something, POST JSON to <INBOX>/inbox/items with the headers
"Authorization: Bearer $INBOX_BOT_TOKEN" and "Content-Type: application/json", using curl in
the shell. The body is {"kind": "<x_drop | poll | event | poll_check | field_change>",
"payload": { ... }}. A 201 answer means received, not accepted. Never print the token.

Before sending, run: curl -s -H "Authorization: Bearer $INBOX_BOT_TOKEN" <INBOX>/inbox/items
It lists every item sent in the last 30 days, by every routine, and what happened to each:
accepted; proposed (a poll waiting for its blind check; it later becomes accepted or
rejected); or rejected, with a note giving the reasons. Fix and resend your own rejected items
when they can be fixed, as your routine's section says. Never resend an item that was accepted
or is proposed.

Rules: report only what you read or measured. Use null for anything you cannot measure; never
estimate a number and present it as measured. Find official documents (Commission des sondages
notices, Conseil constitutionnel decisions, court rulings) through a search or the Wayback
Machine, never by trying URLs one after another on a government site. Never include X post ids,
user names, or any secret. Times are UTC, written like 2026-10-09T18:00:00Z; dates are
YYYY-MM-DD. Write neutral, factual English.
```

## Routine 1: X activity

**Schedule:** every 6 hours, 20 minutes after 00:00, 06:00, 12:00 and 18:00 UTC. Skill version
`x-activity-1`.

```text
Measure activity on X about each candidate during the last full 6-hour window: it ends at the
most recent of 00:00, 06:00, 12:00 or 18:00 UTC and starts 6 hours earlier. Include every
candidate in candidates.yaml whose status is not "withdrawn" or "ineligible".

For each candidate, search X for posts in the window that match any of their names in
sources.yaml (in quotes, joined with OR), and record:
- query: the exact search you ran.
- mentions: the number of posts matching the query in the window. If your native X count gives
  it, use that and set mentions_method to "native_count". If you can only estimate it from what
  you read, set mentions_method to "sample_estimate". If neither, null.
- sample_size: how many posts you actually read for this candidate (aim for 50, mixing the most
  recent and the most engaged).
- sentiment: among the posts you read, the fractions that are positive, negative and neutral
  toward the candidate (pos + neg + neu = 1), and method: "grok-bot reading <N> posts, routine
  v1". null if you read fewer than 20 posts.
- top_topics: up to 5 short topics from the posts you read, in English.
- bot_share_estimate: the fraction of the posts you read that come from accounts that look
  automated or coordinated (0 to 1), or null if you can't judge.
- unique_authors and engagement: null, unless you measured them over every post in the window.
Do not include post ids, user names, or post text.

Then POST one item:
{"kind": "x_drop", "payload": {
  "schema_version": "1.0",
  "collected_at": "<now>",
  "window": {"start": "<window start>", "end": "<window end>"},
  "query_method": "Grok Bot native X search, one query per candidate, routine v1",
  "agent": {"name": "Grok Bot fr2027", "skill_version": "x-activity-1", "run_id": RUN_ID},
  "candidates": [{"candidate_id": "...", "query": "...", "mentions": ..., "mentions_method": "...",
                  "sample_size": ..., "unique_authors": null, "engagement": null,
                  "sentiment": {"pos": ..., "neg": ..., "neu": ..., "method": "..."} or null,
                  "top_topics": [...], "bot_share_estimate": ...}, ...]}}
Send each window once. If the inbox shows this window was already accepted, do nothing.
```

The contract is [`schemas/grok_drop.schema.json`](../schemas/grok_drop.schema.json). The pipeline
drops any `sample_post_ids`, so that a post deleted on X never lives on in the public repository.

**Cost:** each run reads about 50 posts for each of about 25 candidates. Grok Bot's docs warn that
frequent routines can use a week's allowance in a day. If the weekly allowance runs low, change
the schedule to once a day at 00:20 UTC with a 24-hour window. The pipeline accepts any window
length.

## Routine 2: new polls

**Schedule:** every day at 09:30 and 19:30, Paris time. Skill version `polls-2`.

```text
Find voting-intention polls for the 2027 presidential election published since the newest one
in https://github.com/Hebilicious/fr2027-forecaster/tree/main/data/raw/polls. Also list the
open pull requests (https://api.github.com/repos/Hebilicious/fr2027-forecaster/pulls?state=open&per_page=100)
whose titles start with "Polls from Grok Bot", and your inbox items, so you don't send a poll
that is already proposed.

Where to look: the Commission des sondages notices (https://www.commission-des-sondages.fr/notices/),
which every published election poll must have; the pollsters' own sites (Ifop, Elabe, Odoxa,
OpinionWay, Ipsos BVA, Toluna Harris Interactive, Cluster17, Verian, YouGov, CSA); and the poll
list on French Wikipedia
(https://fr.wikipedia.org/wiki/Liste_de_sondages_sur_l%27%C3%A9lection_pr%C3%A9sidentielle_fran%C3%A7aise_de_2027)
as an index only.

For each new poll, read the numbers from the Commission's notice or the pollster's publication
(the PDF), not from an article or Wikipedia, and POST {"kind": "poll", "payload": {...}} with
exactly these fields (open a file in data/raw/polls for the format):
- schema_version "1.0"; poll_id "<firm>-<field_end>-<notice number>", e.g.
  "ifop-2026-09-29-10284" (without a notice number yet: "<firm>-<field_end>-<sponsor slug>");
- firm (a lowercase slug: ifop, elabe, odoxa, opinionway, ipsos-bva, toluna-harris, cluster17,
  verian, yougov, csa, ...), sponsor (or null), field_start, field_end, published_at (the day
  the results were first published), sample_size (the total sample), method (online, phone or
  mixed), population (registered, or likely when the published figures are computed on voters
  certain to vote);
- source_url (the PDF you read), notice_url (the Commission's notice, or null), index_url (or
  null), retrieved_at (now), entry: "primary" if you read the numbers in the notice or the
  pollster's own publication, otherwise "index";
- notes: anything a reader needs, such as how blank votes were treated, in one or two sentences;
- agent: {"name": "Grok Bot fr2027", "skill_version": "polls-2", "run_id": RUN_ID};
- scenarios: one per ballot tested, each {"scenario_id": "S1", "round": 1, "label": "<who was
  tested>", "shares": {"<candidate id>": <percent of expressed votes>}}. Round 2 head-to-heads
  are scenarios with round 2 and exactly two candidates ("H1", "H2", ...). Write "<0.5" or "<1"
  as 0.5, and a dash as 0. Round 1 shares must add up to 85–103, round 2 to 98–102.

If a scenario tests someone who has no id in candidates.yaml, don't send that poll. Send an
event instead (kind "other", see routine 3 for the format) saying who was tested and in which
poll; routine 4 adds them to the list, and you send the poll once they are in it.

Never send a poll_check: checks come from routine 4, another run.

When the inbox shows one of your polls rejected because "a blind check differs", the note lists
which fields and which candidates' shares differ (no values). Re-read exactly those
figures in the PDF. If your figure was wrong, send the corrected poll with the same poll_id: it
replaces the open proposal and waits for a new check. If your figures are what the PDF says, do
nothing; the next check settles it.
```

The pipeline proposes each valid poll in a pull request titled "Polls from Grok Bot: …", whose
description lists each poll's id, PDF links and the run that entered it, but none of its numbers.
Routine 4 enters each one again from the PDF; Live commits it when both agree.

## Routine 3: campaign events

**Schedule:** every day at 21:00, Paris time. Skill version `events-1`.

```text
Report the day's events that change who will be on the ballot or how the field looks:
declarations and withdrawals of candidacy; endorsements of one candidate by another or by a
party; primary results; sponsorship (parrainage) counts and the official list from the Conseil
constitutionnel (from early 2027); court rulings affecting a candidate's eligibility (for
example Marine Le Pen's appeal to the Cour de cassation); and the publication of a major poll,
as kind "poll_published".

Check first: https://github.com/Hebilicious/fr2027-forecaster/tree/main/data/raw/events and
the inbox items, so you don't report the same event twice.

For each event, POST {"kind": "event", "payload": {
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
Report only events from the last 48 hours that you confirmed in at least one primary source or
national outlet. Nothing happened: send nothing.
```

Events are shown on the Signals page as reported. Routine 4 turns the confirmed ones into field
changes.

## Routine 4: poll checks and field changes

**Schedule:** every day at 10:30 and 20:30, Paris time. Skill version `checks-1`. Do the three
parts in this order.

```text
Every payload you send in this run carries
"agent": {"name": "Grok Bot fr2027", "skill_version": "checks-1", "run_id": RUN_ID}.

(a) Blind checks of proposed polls.
List the open pull requests:
https://api.github.com/repos/Hebilicious/fr2027-forecaster/pulls?state=open&per_page=100
Keep those whose head.ref starts with "inbox/polls-". Each one's body has a table with, for
each poll, its poll_id, source_url, notice_url, supersedes and the run that entered it. Read
only that table. Do not open the pull request's files, diff, commits or branch, nor
data/proposals/, and do not look up the poll's numbers anywhere else (articles, Wikipedia, X):
the check is worth something only if you enter the poll yourself from the source.
Skip a poll whose "entered by run" is your RUN_ID, and a poll whose check you already sent
that is still pending.
For each other poll, open the PDF at notice_url (or source_url) and enter the whole poll
exactly as routine 2 does (all the fields of a poll; see "Routine 2: new polls" in this
document), with the same poll_id and the supersedes value from the table (omit supersedes when
the table says none), and your agent. POST it as {"kind": "poll_check", "payload": {...}}.
When the inbox shows an earlier check of yours rejected because it differs, its note lists
which fields and which candidates' shares differ, with no values. Re-read exactly those
figures in the PDF and send a new check with what the PDF says.

(b) Late notices.
In https://github.com/Hebilicious/fr2027-forecaster/tree/main/data/raw/polls, find the files
whose name does not end in a notice number (five digits), for example
"opinionway-2026-10-14-cnews". For each, look for its Commission des sondages notice (same
firm, sponsor and fieldwork). If it is now filed, and no open pull request or inbox item of
yours already has the new poll_id, enter the poll from the notice as routine 2 does, with
poll_id "<firm>-<field_end>-<notice number>", "supersedes": "<the old poll_id>", entry
"primary", notice_url the notice, and your agent, and POST it as {"kind": "poll", ...}.
A later run checks it blind.

(c) Field changes.
Read config/candidates.yaml (the current field; its header explains slots), the events of the
last 7 days in https://github.com/Hebilicious/fr2027-forecaster/tree/main/data/raw/events,
and the inbox items (events, and field changes with their outcomes). For each change in who
runs that at least one primary source or national outlet confirms (a declaration, a
withdrawal, a primary result, an endorsement that ends a candidacy, a ruling on eligibility,
the official list), send one field change. Rules:
- Change a slot's probabilities only for a confirmed fact, never on rumour, polls or markets.
- Every candidate stays in exactly one slot. A candidate who loses or withdraws stays in their
  slot with p 0; a slot's options sum to at most 1 (1 minus the sum is the chance nobody from
  the slot runs).
- A candidate tested in a poll but missing from candidates.yaml (see the "other" events from
  routine 2) gets add_candidate, with a sourced entry, the names that identify them in
  headlines, and a slot.
- Don't send a change_id that was already accepted. If one was rejected, fix what the note
  says and resend it with the same change_id.
POST {"kind": "field_change", "payload": {
  "schema_version": "1.0",
  "change_id": "<YYYY-MM-DD>-<short-slug>", e.g. "2026-10-17-faure-withdraws",
  "event_ids": [ids of accepted events in data/raw/events/ this change relies on; may be empty],
  "sources": [{"url": "<primary source or national outlet>", "title": "<headline>"}, ...],
  "summary": "<one or two neutral sentences, at most 400 characters, on one line>",
  "log": "<one short paragraph for research/LOG.md, on one line: what changed, why, and what
          the sources say>",
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
  matter, X's own counts endpoint (`/2/tweets/counts/recent`, about $0.15 a day for 30 queries)
  can be added as a GitHub Actions collector.
- The docs imply but don't state that a Bot secret is available as an environment variable in
  the shell. The Test run in step 5 settles it. If it isn't, the token can sit in a file under
  /workspace instead. Every Bot on the account can read that file, which is one more reason to
  use a dedicated account.
- Whether a routine keeps memory between runs. The blind check relies on routine 4 entering each
  poll from the PDF; the pipeline enforces only that the check comes from another run.
- The cost of a run is not published. Watch the weekly usage after the first day.
