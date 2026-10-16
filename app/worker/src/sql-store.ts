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

const VIEW_COLUMNS = "id, kind, received_at, status, note, ref, resolved_at"

/** The inbox's items in SQLite. `seq` (insertion order) breaks ties between items received in the same second. */
export class SqlStore implements Store {
  readonly #sql: Sql

  constructor(sql: Sql) {
    this.#sql = sql
    for (const statement of SCHEMA) sql.exec(statement)
  }

  insert(item: StoredItem): void {
    this.#sql.exec(
      `INSERT INTO items (${VIEW_COLUMNS}, payload) VALUES (?, ?, ?, ?, ?, ?, ?, ?)`,
      item.id,
      item.kind,
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
