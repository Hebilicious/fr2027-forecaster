# Grok Bot routines

GitHub Actions does everything that a fixed program can do (markets, news feeds, Wikipedia, the
model, the site; see [operations.md](operations.md)). Grok Bot does the three jobs that need an agent:
measuring X, reading pollsters' publications, and following the campaign. This page is what to set
up in Grok Bot: one Bot, one secret, three routines.

## How Grok Bot talks to the repository

Grok Bot never gets a GitHub credential. GitHub cannot limit a token to one folder of a public
repository, and every Bot on a Grok Bot account shares one computer, so a token held there would be
readable by all of them. Instead:

1. A routine POSTs what it found to the **inbox**, a small endpoint on the project's Cloudflare
   Worker (`app/worker`), with a bearer token that can only add items to the inbox.
2. Every hour the Live workflow pulls the inbox and checks each item with the same code that
   checks every file in the repository:
   - X measurements (`x_drop`) and campaign events (`event`) that pass are committed to `data/raw/grok/`
     and `data/raw/events/`;
   - polls (`poll`) that pass go into a pull request, because a poll moves the forecast and a
     person should check its numbers before it counts;
   - anything that fails is rejected, with the reasons.
3. The routine reads those outcomes back from the inbox at its next run, and resends corrected
   items when something was rejected.

The model does not use X measurements or events yet. They are recorded and shown on the Signals
page so their value can be measured against later polls before they get any weight.

## One-time setup

1. Create a Bot, for example **fr2027**, on a Grok Bot account. Use a dedicated account if you
   run Bots for other work, since Bots on one account share their computer and credentials.
2. In the Bot's **Secrets**, add `INBOX_BOT_TOKEN` with the same value as the GitHub secret of
   that name (see [operations.md](operations.md)). Grok Bot does not show secret values to the
   Bot; it is used as an environment variable in shell commands.
3. In **Approvals**, add an "Allow automatically" rule for running `curl` against
   `https://<your worker>.workers.dev/inbox/`. Work that a routine starts while you are away
   waits about ten minutes for an approval and is then dropped, so without this rule nothing
   would ever be sent.
4. Create the three routines below. Paste each instruction block as written, replacing
   `<INBOX>` with the Worker's address (the `FR2027_INBOX_URL` variable, e.g.
   `https://fr2027.example.workers.dev`).
5. Press **Test** on each routine once and watch the result in the Bot's chat. Then check
   `GET <INBOX>/inbox/items`; a test item will be resolved (accepted or rejected) within the hour.

## Shared preamble

Each routine's instruction starts with this block.

```text
You collect data for fr2027-forecaster, a public, open-source forecast of the 2027 French
presidential election: https://github.com/Hebilicious/fr2027-forecaster.

Candidate ids: use only the ids in
https://raw.githubusercontent.com/Hebilicious/fr2027-forecaster/main/config/candidates.yaml
and the names listed for each id in
https://raw.githubusercontent.com/Hebilicious/fr2027-forecaster/main/config/sources.yaml.

To send something, POST JSON to <INBOX>/inbox/items with the header
"Authorization: Bearer $INBOX_BOT_TOKEN", using curl in the shell. The body is
{"kind": "<x_drop | poll | event>", "payload": { ... }}. A 201 answer means received, not
accepted. Never print the token.

Before sending, run: curl -s -H "Authorization: Bearer $INBOX_BOT_TOKEN" <INBOX>/inbox/items
It lists what you sent in the last 30 days and what happened to each item: accepted, proposed
(a pull request for a person to review) or rejected, with a note giving the reasons. Fix and
resend anything you sent that was rejected, if it can be fixed. Never resend an item that was
accepted or proposed.

Rules: report only what you read or measured. Use null for anything you cannot measure; never
estimate a number and present it as measured. Times are UTC, written like
2026-10-09T18:00:00Z. Write neutral, factual English.
```

## Routine 1: X activity, every 6 hours

**Schedule:** every 6 hours, 20 minutes after 00:00, 06:00, 12:00 and 18:00 UTC.

```text
[shared preamble]

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
  "agent": {"name": "Grok Bot fr2027", "skill_version": "x-activity-1"},
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

## Routine 2: new polls, twice a day

**Schedule:** every day at 09:30 and 19:30, Paris time.

```text
[shared preamble]

Find voting-intention polls for the 2027 presidential election published since the newest one
in https://github.com/Hebilicious/fr2027-forecaster/tree/main/data/raw/polls (also check the
open pull requests whose titles start with "Polls from Grok Bot", so you don't send a poll twice).

Where to look: the Commission des sondages notices (https://www.commission-des-sondages.fr/notices/),
which every published election poll must have; the pollsters' own sites (Ifop, Elabe, Odoxa,
OpinionWay, Ipsos BVA, Toluna Harris Interactive, Cluster17, Verian, YouGov); and the poll list on
French Wikipedia (https://fr.wikipedia.org/wiki/Liste_de_sondages_sur_l%27%C3%A9lection_pr%C3%A9sidentielle_fran%C3%A7aise_de_2027)
as an index only.

For each new poll, read the numbers from the Commission's notice or the pollster's publication
(the PDF), not from an article or Wikipedia, and POST {"kind": "poll", "payload": {...}} with
exactly these fields (copy a file from data/raw/polls for the format):
- schema_version "1.0"; poll_id "<firm>-<field_end>-<notice number>", e.g.
  "ifop-2026-09-29-10284" (without a notice number: "<firm>-<field_end>");
- firm (a lowercase slug: ifop, elabe, odoxa, opinionway, ipsos-bva, toluna-harris, cluster17,
  verian, yougov, ...), sponsor (or null), field_start, field_end, published_at (dates as
  YYYY-MM-DD), sample_size, method (online, phone or mixed), population (registered or likely);
- source_url (the PDF you read), notice_url (the Commission's notice, or null), index_url (or
  null), retrieved_at (now), entry: "primary" if you read the numbers in the notice or the
  pollster's own publication, otherwise "index";
- notes: anything a reader needs, such as how blank votes were treated, in one or two sentences;
- scenarios: one per ballot tested, each {"scenario_id": "S1", "round": 1, "label": "<who was
  tested>", "shares": {"<candidate id>": <percent of expressed votes>}}. Round 2 head-to-heads
  are scenarios with round 2 and exactly two candidates ("H1", "H2", ...). Write "<0.5" as 0.5.
  Round 1 shares must add up to 85–103, round 2 to 98–102.

If a scenario tests someone who has no id in candidates.yaml, don't send that poll. Send an
event instead (kind "other") proposing that they be added, and send the poll once they are in
the list.
```

The pipeline proposes each valid poll in a pull request titled "Polls from Grok Bot: …". You
check the numbers against the linked PDF and merge; the merge refits and republishes the forecast.

## Routine 3: campaign events, every evening

**Schedule:** every day at 21:00, Paris time.

```text
[shared preamble]

Report the day's events that change who will be on the ballot or how the field looks:
declarations and withdrawals of candidacy; endorsements of one candidate by another or by a
party; primary results (the social-democratic primary, any right-wing primary); sponsorship
(parrainage) counts and the official list from the Conseil constitutionnel (from early 2027);
court rulings affecting a candidate's eligibility (for example Marine Le Pen's appeal to the Cour
de cassation); and the publication of a major poll, as kind "poll_published".

Check first: https://github.com/Hebilicious/fr2027-forecaster/tree/main/data/raw/events and
your inbox items, so you don't report the same event twice.

For each event, POST {"kind": "event", "payload": {
  "schema_version": "1.0",
  "event_id": "<date>-<short-slug>", e.g. "2026-10-17-ps-primary-result",
  "date": "<YYYY-MM-DD, when it happened, Paris time>",
  "kind": "declaration" | "withdrawal" | "endorsement" | "primary_result" | "sponsorships" | "legal" | "poll_published" | "other",
  "candidate_ids": [ids involved; may be empty],
  "summary": "<one or two neutral sentences, at most 400 characters>",
  "sources": [{"url": "<primary source or a national outlet>", "title": "<headline>"}],
  "proposed_change": "<optional: the change to config/candidates.yaml this suggests, e.g. 'faure: status withdrawn; ps_primary slot: glucksmann 0.95', or null>",
  "reported_by": "Grok Bot fr2027",
  "reported_at": "<now>"}}
Report only events from the last 48 hours that you confirmed in at least one primary source or
national outlet. Nothing happened: send nothing.
```

Events are shown on the Signals page as reported. A `proposed_change` is a suggestion for a person
to make through a pull request; the pipeline never edits `config/candidates.yaml` by itself.

## Checking that it works

- `curl -s -H "Authorization: Bearer $INBOX_BOT_TOKEN" <INBOX>/inbox/items` lists each item and
  its outcome.
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
- The cost of a run is not published. Watch the weekly usage after the first day.
