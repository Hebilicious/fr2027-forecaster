/**
 * Headers for every JSON answer the Worker builds itself. Workers static assets apply `_headers`
 * (app/web/public) to their own responses; a response the Worker constructs does not get them, so
 * both places that build one take the headers from here.
 */
export const JSON_HEADERS = {
  "content-type": "application/json; charset=utf-8",
  "cache-control": "no-store",
  "x-content-type-options": "nosniff",
  "referrer-policy": "no-referrer",
} as const
