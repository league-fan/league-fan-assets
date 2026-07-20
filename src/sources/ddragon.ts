import type { VersionInfo } from "../types/version.js";
import { fetchJson } from "../loaders/http.js";

export const DDRAGON_REALM_TENCENT =
  "https://ddragon.leagueoflegends.com/realms/tencent.json";

export const DDRAGON_REALM_NA =
  "https://ddragon.leagueoflegends.com/realms/na.json";

export async function getGameVersion(opts?: {
  fetch?: typeof globalThis.fetch;
  realmUrl?: string;
}): Promise<string> {
  const info = await getVersionInfo(opts);
  return info.v;
}

export async function getVersionInfo(opts?: {
  fetch?: typeof globalThis.fetch;
  realmUrl?: string;
}): Promise<VersionInfo> {
  const url = opts?.realmUrl ?? DDRAGON_REALM_TENCENT;
  return fetchJson<VersionInfo>(url, { fetch: opts?.fetch });
}
