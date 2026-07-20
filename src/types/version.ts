export interface VersionInfo {
  /** Game / ddragon version, e.g. 15.24.1 */
  v: string;
  n?: Record<string, string>;
  l?: string;
  cdn?: string;
  dd?: string;
  lg?: string;
  css?: string;
  profileiconmax?: number;
  store?: unknown;
  [key: string]: unknown;
}

export interface DataManifest {
  schemaVersion: number;
  gameVersion: string;
  generatedAt: string;
  patchUsed: string;
  languages: string[];
  categories: string[];
  files: Record<string, { bytes: number; sha256: string }>;
}
