import type { ItemView, Store, StoredItem } from "./inbox"

type SqlValue = ArrayBuffer | string | number | null
type Row = Record<string, SqlValue>

/**
 * The part of the Durable Object `SqlStorage` API (`ctx.storage.sql`) the store uses. `exec` runs
 * one statement immediately; the cursor yields its rows. Tests pass Node's `node:sqlite` instead.
 */
export interface Sql {
  exec(query: string, ...bindings: (string | number | null)[]): { toArray(): Row[] }
}

const SCHEMA = [
  `CREATE TABLE IF NOT EXISTS items (
    seq INTEGER PRIMARY KEY,
    id TEXT NOT NULL UNIQUE,
    kind TEXT NOT NULL,
    key TEXT,
    received_at TEXT NOT NULL,
    status TEXT NOT NULL,
    note TEXT,
    ref TEXT,
    resolved_at TEXT,
    payload TEXT
  )`,
  "CREATE INDEX IF NOT EXISTS items_by_status ON items (status, received_at, seq)",
  "CREATE INDEX IF NOT EXISTS items_by_received ON items (received_at, seq)",
]

/** Tables created before items had a key get the column; their old items keep a null key. */
const ADD_KEY = "ALTER TABLE items ADD COLUMN key TEXT"
const KEY_INDEX = "CREATE INDEX IF NOT EXISTS items_by_key ON items (kind, key, status)"

const VIEW_COLUMNS = "id, kind, key, received_at, status, note, ref, resolved_at"

/** The inbox's items in SQLite. `seq` (insertion order) breaks ties between items received in the same second. */
export class SqlStore implements Store {
  readonly #sql: Sql

  constructor(sql: Sql) {
    this.#sql = sql
    for (const statement of SCHEMA) sql.exec(statement)
    const columns = sql.exec("PRAGMA table_info(items)").toArray()
    if (!columns.some((column) => column["name"] === "key")) sql.exec(ADD_KEY)
    sql.exec(KEY_INDEX)
  }

  insert(item: StoredItem): void {
    this.#sql.exec(
      `INSERT INTO items (${VIEW_COLUMNS}, payload) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)`,
      item.id,
      item.kind,
      item.key,
      item.received_at,
      item.status,
      item.note,
      item.ref,
      item.resolved_at,
      item.payload,
    )
  }

  get(id: string): StoredItem | undefined {
    const [row] = this.#sql.exec(`SELECT ${VIEW_COLUMNS}, payload FROM items WHERE id = ?`, id).toArray()
    return row === undefined ? undefined : (row as unknown as StoredItem)
  }

  update(item: StoredItem): void {
    this.#sql.exec(
      "UPDATE items SET status = ?, note = ?, ref = ?, resolved_at = ?, payload = ? WHERE id = ?",
      item.status,
      item.note,
      item.ref,
      item.resolved_at,
      item.payload,
      item.id,
    )
  }

  countPending(): number {
    const [row] = this.#sql.exec("SELECT count(*) AS n FROM items WHERE status = 'pending'").toArray()
    return Number(row?.["n"] ?? 0)
  }

  findSettled(kind: string, key: string): StoredItem | undefined {
    const [row] = this.#sql
      .exec(
        `SELECT ${VIEW_COLUMNS}, payload FROM items
         WHERE kind = ? AND key = ? AND status IN ('accepted', 'proposed')
         ORDER BY received_at DESC, seq DESC LIMIT 1`,
        kind,
        key,
      )
      .toArray()
    return row === undefined ? undefined : (row as unknown as StoredItem)
  }

  listSince(since: string): ItemView[] {
    return this.#sql
      .exec(`SELECT ${VIEW_COLUMNS} FROM items WHERE received_at >= ? ORDER BY received_at DESC, seq DESC`, since)
      .toArray() as unknown as ItemView[]
  }

  listPending(limit: number): StoredItem[] {
    return this.#sql
      .exec(
        `SELECT ${VIEW_COLUMNS}, payload FROM items WHERE status = 'pending' ORDER BY received_at, seq LIMIT ?`,
        limit,
      )
      .toArray() as unknown as StoredItem[]
  }

  purgeBefore(before: string): void {
    this.#sql.exec("DELETE FROM items WHERE received_at < ?", before)
  }
}
