import type { Lang, Patch, AssetCategory } from "./types/common.js";
import type {
  CategoryDataMap,
  SummonerIcon,
  SummonerIconSet,
  SummonerEmote,
  WardSkin,
  WardSkinSet,
  LootItem,
  Champion,
  Skinline,
  Universe,
  Skins,
  Added,
  VersionInfo,
} from "./types/index.js";
import { loadCategory, type DataSource, type LoadOptions } from "./loaders/category.js";

export interface ClientOptions {
  lang?: Lang;
  source?: DataSource;
  fetch?: typeof globalThis.fetch;
  /** Default patch override for CDragon loads / path rewriting. */
  patch?: Patch;
}

export interface LeagueFanAssetsClient {
  readonly lang: Lang;
  readonly source: DataSource;
  load<C extends AssetCategory>(category: C): Promise<CategoryDataMap[C]>;
  summonerIcons(): Promise<SummonerIcon[]>;
  summonerIconSets(): Promise<SummonerIconSet[]>;
  summonerEmotes(): Promise<SummonerEmote[]>;
  wardSkins(): Promise<WardSkin[]>;
  wardSkinSets(): Promise<WardSkinSet[]>;
  loot(): Promise<LootItem[]>;
  champions(): Promise<Champion[]>;
  skinlines(): Promise<Skinline[]>;
  universes(): Promise<Universe[]>;
  skins(): Promise<Skins>;
  added(): Promise<Added>;
  version(): Promise<VersionInfo>;
}

export function createClient(options: ClientOptions = {}): LeagueFanAssetsClient {
  const lang = options.lang ?? "default";
  const source: DataSource = options.source ?? { kind: "cdragon" };
  const base: LoadOptions = {
    lang,
    source,
    fetch: options.fetch,
    patch: options.patch,
  };

  const load = <C extends AssetCategory>(category: C) =>
    loadCategory(category, base);

  return {
    lang,
    source,
    load,
    summonerIcons: () => load("summoner-icons"),
    summonerIconSets: () => load("summoner-icon-sets"),
    summonerEmotes: () => load("summoner-emotes"),
    wardSkins: () => load("ward-skins"),
    wardSkinSets: () => load("ward-skin-sets"),
    loot: () => load("loot"),
    champions: () => load("champions"),
    skinlines: () => load("skinlines"),
    universes: () => load("universes"),
    skins: () => load("skins"),
    added: () => load("added"),
    version: () => load("version"),
  };
}

export type { DataSource, LoadOptions };
