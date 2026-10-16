// The public site's API documents, /api/*.json, served from the static assets. The assets
// answer an unknown path with index.html (not_found_handling: single-page-application), which is
// right for page routes but not for a document, so /api/* runs the Worker first and turns that
// fallback into a 404.

/** The part of the ASSETS binding this module uses. */
export interface Assets {
  fetch(request: Request): Promise<Response>
}

export async function handleApi(request: Request, assets: Assets): Promise<Response> {
  const response = await assets.fetch(request)
  // Every document is JSON, so an HTML answer can only be the single-page fallback.
  const fallback = response.ok && (response.headers.get("content-type") ?? "").startsWith("text/html")
  if (!fallback) return response
  const path = new URL(request.url).pathname
  return new Response(JSON.stringify({ error: `${path}: no such document` }), {
    status: 404,
    // _headers does not apply to responses the Worker builds, so this one sets its own.
    headers: {
      "content-type": "application/json; charset=utf-8",
      "cache-control": "no-store",
      "x-content-type-options": "nosniff",
    },
  })
}
