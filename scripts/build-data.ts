/**
 * Build pre-normalized JSON snapshots for GitHub Releases.
 * Uses the same transforms as the runtime client.
 */
import { createHash } from "node:crypto";
import { mkdir, writeFile, rm } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";
import {
  DEFAULT_DATA_LANGUAGES,
  PUBLISHED_DATA_CATEGORIES,
} from "../src/catalog.js";
import { getVersionInfo } from "../src/sources/ddragon.js";
import { cdragonDataUrl, defaultPatchForCategory } from "../src/sources/cdragon.js";
import { fetchJson } from "../src/loaders/http.js";
import {
  transformChampions,
  transformLoot,
  transformSkinlines,
  transformSkins,
  transformSummonerEmotes,
  transformSummonerIconSets,
  transformSummonerIcons,
  transformUniverses,
  transformWardSkinSets,
  transformWardSkins,
  computeAdded,
} from "../src/transforms/index.js";
import type { AssetCategory, Lang, Patch } from "../src/types/common.js";
import type { Champion } from "../src/types/champion.js";
import type { Skinline } from "../src/types/skinline.js";
import type { Skins } from "../src/types/skins.js";
import type { Universe } from "../src/types/universe.js";
import type { DataManifest } from "../src/types/version.js";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const ROOT = path.resolve(__dirname, "..");
const OUT = path.join(ROOT, "data", "out");
const VERSION_FILE = path.join(ROOT, "VERSION");

async function loadAndTransform(
  category: AssetCategory,
  lang: Lang,
  patch: Patch,
): Promise<unknown> {
  const url = cdragonDataUrl({ category, lang, patch });
  console.log(`fetch: ${url}`);
  const raw = await fetchJson<unknown>(url);
  const tOpts = { lang, patch };

  switch (category) {
    case "summoner-icons":
      return transformSummonerIcons(raw as unknown[], tOpts);
    case "summoner-icon-sets":
      return transformSummonerIconSets(raw, tOpts);
    case "summoner-emotes":
      return transformSummonerEmotes(raw as unknown[], tOpts);
    case "ward-skins":
      return transformWardSkins(raw as unknown[], tOpts);
    case "ward-skin-sets":
      return transformWardSkinSets(raw, tOpts);
    case "loot":
      return transformLoot(raw, tOpts);
    case "champions":
      return transformChampions(raw as unknown[], tOpts);
    case "skinlines":
      return transformSkinlines(raw as unknown[], tOpts);
    case "universes":
      return transformUniverses(raw as unknown[], tOpts);
    case "skins":
      return transformSkins(raw as Skins, tOpts);
    default:
      throw new Error(`No transform for ${category}`);
  }
}

async function writeJson(filePath: string, data: unknown): Promise<{ bytes: number; sha256: string }> {
  const text = JSON.stringify(data);
  const buf = Buffer.from(text, "utf8");
  await mkdir(path.dirname(filePath), { recursive: true });
  await writeFile(filePath, buf);
  const sha256 = createHash("sha256").update(buf).digest("hex");
  return { bytes: buf.byteLength, sha256 };
}

async function main() {
  const versionInfo = await getVersionInfo();
  const gameVersion = versionInfo.v;
  console.log(`game version: ${gameVersion}`);

  // Skip if VERSION unchanged and --force not set
  const force = process.argv.includes("--force");
  try {
    const { readFile } = await import("node:fs/promises");
    const prev = (await readFile(VERSION_FILE, "utf8")).trim();
    if (!force && prev === gameVersion) {
      console.log("VERSION is up to date; write SKIP and exit");
      await writeFile(path.join(ROOT, "SKIP"), "");
      return;
    }
  } catch {
    // no VERSION file yet
  }

  await rm(OUT, { recursive: true, force: true });
  await mkdir(OUT, { recursive: true });

  const files: DataManifest["files"] = {};
  const languages = [...DEFAULT_DATA_LANGUAGES];

  // Cosmetics use latest; skins domain use pbe for freshest art, but for a
  // consistent data release we pin cosmetics+skins both to "latest" unless
  // LFA_PATCH is set.
  const patchOverride = (process.env.LFA_PATCH as Patch | undefined) ?? "latest";

  let defaultChampions: Champion[] = [];
  let defaultSkinlines: Skinline[] = [];
  let defaultSkins: Skins = {};
  let defaultUniverses: Universe[] = [];

  for (const lang of languages) {
    for (const category of PUBLISHED_DATA_CATEGORIES) {
      if (category === "added") continue;

      const patch =
        patchOverride ||
        defaultPatchForCategory(category as AssetCategory);

      const data = await loadAndTransform(
        category as AssetCategory,
        lang,
        patch,
      );

      if (lang === "default") {
        if (category === "champions") defaultChampions = data as Champion[];
        if (category === "skinlines") defaultSkinlines = data as Skinline[];
        if (category === "skins") defaultSkins = data as Skins;
        if (category === "universes") defaultUniverses = data as Universe[];
      }

      const rel = `${lang}/${category}.json`;
      const meta = await writeJson(path.join(OUT, rel), data);
      files[rel] = meta;
      console.log(`save: ${rel} (${meta.bytes} bytes)`);
    }
  }

  // Compute added: compare current (default) vs CDragon "latest" baseline when
  // building from pbe; when already on latest, added is empty.
  let added = {
    skins: [] as string[],
    champions: [] as number[],
    skinlines: [] as number[],
    universes: [] as number[],
  };

  if (patchOverride === "pbe" || patchOverride !== "latest") {
    console.log("computing added vs latest…");
    const [prevC, prevSl, prevS, prevU] = await Promise.all([
      loadAndTransform("champions", "default", "latest") as Promise<Champion[]>,
      loadAndTransform("skinlines", "default", "latest") as Promise<Skinline[]>,
      loadAndTransform("skins", "default", "latest") as Promise<Skins>,
      loadAndTransform("universes", "default", "latest") as Promise<Universe[]>,
    ]);
    added = computeAdded({
      current: {
        champions: defaultChampions,
        skinlines: defaultSkinlines,
        skins: defaultSkins,
        universes: defaultUniverses,
      },
      previous: {
        champions: prevC,
        skinlines: prevSl,
        skins: prevS,
        universes: prevU,
      },
    });
  }

  const addedRel = "default/added.json";
  files[addedRel] = await writeJson(path.join(OUT, addedRel), added);
  // also mirror under zh_cn for path symmetry
  files["zh_cn/added.json"] = await writeJson(
    path.join(OUT, "zh_cn/added.json"),
    added,
  );

  files["version.json"] = await writeJson(
    path.join(OUT, "version.json"),
    versionInfo,
  );

  const manifest: DataManifest = {
    schemaVersion: 1,
    gameVersion,
    generatedAt: new Date().toISOString(),
    patchUsed: patchOverride,
    languages,
    categories: [...PUBLISHED_DATA_CATEGORIES],
    files,
  };
  files["manifest.json"] = await writeJson(
    path.join(OUT, "manifest.json"),
    manifest,
  );
  // re-write manifest with self file entry — skip circular; keep without self hash update

  await writeFile(VERSION_FILE, `${gameVersion}\n`);
  console.log(`wrote VERSION=${gameVersion}`);
  console.log(`output: ${OUT}`);
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
