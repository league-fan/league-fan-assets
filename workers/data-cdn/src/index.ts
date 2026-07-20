/**
 * Public CDN for league-fan prebuilt JSON snapshots.
 *
 * URL map (R2 keys under meta/):
 *   GET /latest/zh_cn__skins.json  → meta/latest/zh_cn__skins.json
 *   GET /v/16.14.1/default__champions.json → meta/v/16.14.1/default__champions.json
 *   GET /health
 *
 * Flat asset names match GitHub Release naming (lang__category.json).
 */

export interface Env {
  BUCKET: R2Bucket;
}

const CORS_HEADERS: Record<string, string> = {
  "Access-Control-Allow-Origin": "*",
  "Access-Control-Allow-Methods": "GET, HEAD, OPTIONS",
  "Access-Control-Allow-Headers": "Content-Type, If-None-Match, Range",
  "Access-Control-Expose-Headers":
    "ETag, Content-Length, Content-Type, Cache-Control",
  "Access-Control-Max-Age": "86400",
};

function isSafeKey(key: string): boolean {
  if (!key || key.includes("..") || key.startsWith("/")) return false;
  // Only serve meta snapshots (not the historical lol-game-data image tree)
  if (!key.startsWith("meta/")) return false;
  if (/[\x00-\x1f]/.test(key)) return false;
  return true;
}

function cacheControlForKey(key: string): string {
  if (key.startsWith("meta/v/")) {
    return "public, max-age=31536000, immutable";
  }
  return "public, max-age=300, s-maxage=600, stale-while-revalidate=86400";
}

/** Map request path → R2 object key. */
function pathToKey(pathname: string): string | null {
  const path = pathname.replace(/^\/+/, "").replace(/\/+/g, "/");
  if (!path || path === "health") return null;
  if (path.startsWith("latest/") || path.startsWith("v/")) {
    return `meta/${path}`;
  }
  return null;
}

export default {
  async fetch(request: Request, env: Env): Promise<Response> {
    if (request.method === "OPTIONS") {
      return new Response(null, { status: 204, headers: CORS_HEADERS });
    }

    if (request.method !== "GET" && request.method !== "HEAD") {
      return new Response("Method Not Allowed", {
        status: 405,
        headers: { ...CORS_HEADERS, Allow: "GET, HEAD, OPTIONS" },
      });
    }

    const url = new URL(request.url);

    if (url.pathname === "/" || url.pathname === "/health") {
      return Response.json(
        {
          ok: true,
          service: "league-fan-data",
          usage: {
            latest: "/latest/{lang}__{category}.json",
            versioned: "/v/{gameVersion}/{lang}__{category}.json",
            examples: [
              "/latest/manifest.json",
              "/latest/version.json",
              "/latest/zh_cn__skins.json",
              "/v/16.14.1/default__champions.json",
            ],
          },
        },
        {
          headers: { ...CORS_HEADERS, "Cache-Control": "public, max-age=60" },
        },
      );
    }

    const key = pathToKey(url.pathname);
    if (!key || !isSafeKey(key)) {
      return new Response("Not Found", {
        status: 404,
        headers: CORS_HEADERS,
      });
    }

    const ifNoneMatch = request.headers.get("If-None-Match");
    const etagToken = ifNoneMatch
      ?.replace(/^W\//, "")
      .replaceAll('"', "")
      .trim();

    const object = await env.BUCKET.get(
      key,
      etagToken
        ? { onlyIf: { etagDoesNotMatch: etagToken } }
        : undefined,
    );

    if (object === null) {
      return new Response("Not Found", {
        status: 404,
        headers: CORS_HEADERS,
      });
    }

    // Precondition matched (etag same) → body omitted
    if (!object.body) {
      return new Response(null, {
        status: 304,
        headers: {
          ...CORS_HEADERS,
          ETag: object.httpEtag,
          "Cache-Control": cacheControlForKey(key),
        },
      });
    }

    const headers = new Headers(CORS_HEADERS);
    object.writeHttpMetadata(headers);
    headers.set("etag", object.httpEtag);
    headers.set("Cache-Control", cacheControlForKey(key));
    if (key.endsWith(".json")) {
      headers.set("Content-Type", "application/json; charset=utf-8");
    }

    if (request.method === "HEAD") {
      return new Response(null, { status: 200, headers });
    }

    return new Response(object.body, { status: 200, headers });
  },
};
