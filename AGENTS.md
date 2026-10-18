# Agent instructions

fr2027-forecaster is a public, self-updating forecast of the 2027 French presidential election. Read
[README.md](README.md) and [SPEC.md](SPEC.md) first; [docs/operations.md](docs/operations.md) lists what runs on its own
and when.

## Branches and pull requests

- Every issue or task gets its own branch from `main`, checked out in a new worktree under `.worktrees/`
  (`git worktree add .worktrees/<branch> -b <branch> main`), and all its work happens with that worktree as the working
  directory. Do not work directly on `main` unless the user explicitly directs the current session to.
- A session's files belong to its worktree, and the main checkout keeps the contents it had: scratch scripts, logs and
  screenshots go under `.artifacts/` in the task worktree, or into the commit when they are review evidence.
- When spawning a subagent, state whether it shares the parent's branch and worktree or give it its own, and start it
  there; a subagent never infers this.
- Every change reaches `main` through a GitHub pull request, opened before the first handoff with
  `gh pr create --base main --head <branch>`. A task has exactly one pull request, or one stack of them. A pull request
  that closes an issue says `Closes #<number>` in its body.
- Every handoff message includes the clickable URL of the task's pull request, or of every pull request in its stack,
  each time, including handoffs after a review correction. Copy the URL from `gh pr view --json url` or `gh stack view`
  rather than retyping it.
- Every handoff of a change a visitor can see (the web UI, the Worker's routes, what the exported site shows) also
  includes the URL of the pull request's deployed preview, for the commit being handed off, each time. The Preview
  workflow (`.github/workflows/preview.yml`) deploys every pull request that touches `app/web`, `app/worker`, `cli`,
  `model`, `config` or `schemas` as the Worker `fr2027-pr-<number>`, posts its `workers.dev` URL on the pull request
  and deletes it when the pull request closes. Wait for that run on the head commit, open the preview and check the
  change there, then copy the URL from the comment. A handoff without it is incomplete: when the run fails, fix it, or
  deploy by hand with `moon run worker:preview-deploy -- --name fr2027-pr-<number>`; during a publication blackout,
  say there is no preview.
- Use `gh stack` whenever the work splits into layers a reviewer can read one at a time, and always for a change too
  large for one pull request: one branch and pull request per layer, all in the task's worktree, submitted with
  `gh stack submit`. [.agents/skills/gh-stack/SKILL.md](.agents/skills/gh-stack/SKILL.md) covers the commands; it is
  upstream's skill from `github/gh-stack`, kept byte-identical and excluded from oxfmt.
- A human reviews every pull request before it merges. Merge only after the human accepts the handed-off work, or when
  their current message explicitly says to merge without review; a request for changes, approval of a plan, or an
  instruction given before the work exists is not acceptance.
- The author of a branch owns it until it merges: keep it rebased on `main`, resolve its conflicts and make its CI
  (`moon ci`) pass. Never skip or weaken a check to make it pass, and never push to, rebase or merge another session's
  branch.
- Once a pull request or stack merges, remove its worktree (`git worktree remove .worktrees/<branch>`) and bring the
  main checkout's `main` up to `origin/main`: `git -C <repository root> pull --ff-only origin main`. Pull only with
  `--ff-only`; when it refuses, stop and tell the user rather than resetting, stashing or merging.

Install the git hooks once per clone with `moon run repo:hooks-install`.

Run project tools through their moon tasks (`moon run web:dev`, `moon run cli:serve`, `moon run cli:backtest`, …); call
`vite`, `cargo` or the `fr2027` binary directly only for something no task covers, such as a flag a task does not pass.
`moon tasks` lists them.

## Reporting

- When a backtest, a gate or a rule would leave a feature the owner asked for with no effect, stop and report it to the
  owner before finishing the work; don't deliver the feature switched off.

## UI languages

The web UI (`app/web`) is written in fourteen languages with [Intlayer](https://intlayer.org): French, the reference,
then English (UK), German, Italian, Spanish (Spain), Portuguese (Portugal), Dutch, Polish, Russian, Ukrainian,
Japanese, Korean, Chinese (simplified) and Thai. `app/web/intlayer.config.ts` lists them. Every screen exists in all
fourteen; a change that adds or rewords UI text is not done until it does.

- Every word a visitor reads or hears comes from a dictionary: text, `aria-label`, `title`, `alt` and the page title.
  `Foo.vue` keeps its words in `Foo.content.ts` beside it and reads them with `useIntlayer("<key>")`; words several
  components share live in `src/common.content.ts`.
- Write the French first, then every other locale in the same `t({ ... })`, translated with the same care: the meaning
  of the French, the terms that language's press uses for French elections, and its own typography. Never ship a locale
  as a copy of the French or English, never make a locale optional, and never weaken the checks below to get a change
  through.
- Put values into a message with `insert(t({ ... }))` and `{{name}}` placeholders, never by joining strings in code, since
  word order differs between languages. Every locale keeps the French message's placeholders.
- Format numbers, percentages, dates and lists with `src/format.ts`, which uses `Intl` for the current locale. Never
  write a locale's separators or a `locale === "fr"` branch by hand.
- Text that reads the same in every language (a file path, a field name, notation) goes in an element marked
  `translate="no"`. Names and data (candidates, pollsters, headlines) are shown as the data gives them.
- Three checks enforce this, and `moon ci` runs them all: `strictMode: "strict"` makes a missing locale a type error
  (`moon run web:typecheck`), `moon run web:i18n-check` runs `intlayer test`, and `src/catalog.test.ts`
  (`moon run web:test`) fails on literal text in a template and on a dictionary entry that is empty or loses a
  placeholder in some locale.
- To add or drop a language, change `intlayer.config.ts`, its name and flag in `src/locales.ts`, and every dictionary.

## Explaining the model

The overview's method section (`app/web/src/components/MethodSection.vue`) explains the model with the numbers of the
forecast on screen, read from the forecast file and the settings it records (`parameters`), never typed into the text.
A change to the method (`model/src`, the settings in `config/model.yaml`, which inputs the model uses) updates that
section's wording in the same pull request. The chain that catches a forgotten setting: `config/model.yaml` is read with
`deny_unknown_fields`, every forecast is validated against `schemas/forecast.schema.json`, which lists the settings
exactly, and `MethodSection.test.ts` requires a label for each of them.

## Automation

Automation, such as the Live workflow's hourly data commits, lands on `main` by itself. It needs no review or
authorization.
