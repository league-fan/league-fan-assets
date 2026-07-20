export class FetchError extends Error {
  readonly status: number;
  readonly url: string;

  constructor(url: string, status: number, statusText: string) {
    super(`HTTP ${status} ${statusText} for ${url}`);
    this.name = "FetchError";
    this.status = status;
    this.url = url;
  }
}

export async function fetchJson<T>(
  url: string,
  opts?: { fetch?: typeof globalThis.fetch },
): Promise<T> {
  const fetchImpl = opts?.fetch ?? globalThis.fetch;
  if (!fetchImpl) {
    throw new Error("No fetch implementation available");
  }
  const res = await fetchImpl(url);
  if (!res.ok) {
    throw new FetchError(url, res.status, res.statusText);
  }
  return (await res.json()) as T;
}
