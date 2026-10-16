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
- Use `gh stack` whenever the work splits into layers a reviewer can read one at a time, and always for a change too
  large for one pull request: one branch and pull request per layer, all in the task's worktree, submitted with
  `gh stack submit`. [.agents/skills/gh-stack/SKILL.md](.agents/skills/gh-stack/SKILL.md) covers the commands; it is
  upstream's skill from `github/gh-stack`, kept byte-identical and excluded from oxfmt.
- A human reviews every pull request before it merges. Merge only after the human accepts the handed-off work, or when
  their current message explicitly says to merge without review; a request for changes, approval of a plan, or an
  instruction given before the work exists is not acceptance.
- Merge with a squash merge: `gh pr merge <number> --squash`, or `gh stack merge --squash` for a stack. The squashed
  commit's subject is the pull request title.
- The author of a branch owns it until it merges: keep it rebased on `main`, resolve its conflicts and make its CI
  (`moon ci`) pass. Never skip or weaken a check to make it pass, and never push to, rebase or merge another session's
  branch.
- Once a pull request or stack merges, remove its worktree (`git worktree remove .worktrees/<branch>`) and bring the
  main checkout's `main` up to `origin/main`: `git -C <repository root> pull --ff-only origin main`. Pull only with
  `--ff-only`; when it refuses, stop and tell the user rather than resetting, stashing or merging.

## Commits

- Commit titles and pull request titles are Conventional Commits, `type(scope): subject`, for example
  `fix(model): clamp the poll weights`. The types are `build`, `chore`, `ci`, `docs`, `feat`, `fix`, `perf`,
  `refactor`, `revert`, `style` and `test`; the scope is optional and lowercase (a moon project ID fits), and `!` before
  the colon marks a breaking change.
- Commits and pull requests carry no AI attribution: no `Co-Authored-By` line for an AI tool, no "Generated with"
  line. [.gommage.yml](.gommage.yml) sets the rules; `gommage fix` rewrites a branch's commits that break them.
- Install the git hooks once per clone with `moon run repo:hooks-install` ([lefthook.yml](lefthook.yml)): the
  `commit-msg` hook checks the title and the attribution, and `pre-commit` checks the formatting of staged files.
  `moon run repo:semantic-check repo:authorship-check` checks every commit of the current branch.
- The [Commits workflow](.github/workflows/commits.yml) runs the same checks on every pull request. Never bypass the
  hooks with `--no-verify`.

## Automation

These write to `main` by themselves, without a pull request reviewed by a person; that is how the forecast keeps
itself up to date:

- the Live workflow commits data updates (`chore(data): …`) to `main` every hour;
- the scheduled editor routine pushes `claude/editor-*` branches, and the Land workflow checks them and fast-forwards
  `main` ([docs/editor.md](docs/editor.md));
- Live opens Grok Bot's polls as `inbox/polls-*` pull requests, which the editor checks and Live closes once their files
  are on `main` ([docs/grok-bot.md](docs/grok-bot.md)).

Their changes need no authorization: they land without review. Do not hold them for approval or ask before they land. The rules
under "Branches and pull requests" do not apply to them.

The editor's route to `main` belongs to the scheduled editor routine alone. A session never creates, pushes or
names a branch `claude/editor-*` or `inbox/polls-*`, never starts Land or the editor routine, and never uses either to
put its own work on `main`: its work goes through a pull request like any other change.

Because Live pushes to `main` often, rebase a branch on `origin/main` right before it merges.
