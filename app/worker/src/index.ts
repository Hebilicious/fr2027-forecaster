import { DurableObject } from "cloudflare:workers"
import { dispatchLiveRun } from "./dispatch"
import { handleInbox } from "./inbox"
import { handleApi } from "./site"
import { SqlStore } from "./sql-store"

/**
 * Set with `wrangler secret put`; any may be missing. Without an inbox token its routes answer
 * 503; without the GitHub App key the cron does nothing.
 */
interface Secrets {
  readonly INBOX_BOT_TOKEN?: string
  readonly INBOX_PIPELINE_TOKEN?: string
  /** The private key of the GitHub App `heb-release`, as a PKCS#8 PEM. */
  readonly GITHUB_APP_PRIVATE_KEY?: string
}

/** The inbox's single instance (named "inbox"), holding its items in SQLite. */
export class Inbox extends DurableObject<Env & Secrets> {
  readonly #store: SqlStore

  constructor(ctx: DurableObjectState, env: Env & Secrets) {
    super(ctx, env)
    this.#store = new SqlStore(ctx.storage.sql)
  }

  override fetch(request: Request): Promise<Response> {
    return handleInbox(request, {
      store: this.#store,
      tokens: { bot: this.env.INBOX_BOT_TOKEN, pipeline: this.env.INBOX_PIPELINE_TOKEN },
      now: () => new Date(),
    })
  }
}

// Only /inbox/* and /api/* reach this script (assets.run_worker_first); the rest of the static
// site never does. The cron (triggers.crons in wrangler.jsonc) starts the Live workflow hourly.
export default {
  fetch(request, env) {
    const path = new URL(request.url).pathname
    if (path.startsWith("/inbox/")) {
      return env.INBOX.get(env.INBOX.idFromName("inbox")).fetch(request)
    }
    if (path.startsWith("/api/")) return handleApi(request, env.ASSETS)
    return env.ASSETS.fetch(request)
  },
  async scheduled(_controller, env) {
    await dispatchLiveRun({
      repository: env.GITHUB_REPOSITORY,
      clientId: env.GITHUB_APP_CLIENT_ID,
      privateKey: env.GITHUB_APP_PRIVATE_KEY,
      fetch: (request) => fetch(request),
      now: () => new Date(),
    })
  },
} satisfies ExportedHandler<Env & Secrets>
