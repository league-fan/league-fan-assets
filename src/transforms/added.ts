import type { Added } from "../types/added.js";
import type { Champion } from "../types/champion.js";
import type { Skinline } from "../types/skinline.js";
import type { Skins } from "../types/skins.js";
import type { Universe } from "../types/universe.js";

export function computeAdded(opts: {
  current: {
    champions: Champion[];
    skinlines: Skinline[];
    skins: Skins;
    universes: Universe[];
  };
  previous: {
    champions: Champion[];
    skinlines: Skinline[];
    skins: Skins;
    universes: Universe[];
  };
}): Added {
  const oldSkinIds = new Set(Object.keys(opts.previous.skins));
  const oldChampionIds = new Set(opts.previous.champions.map((c) => c.id));
  const oldSkinlineIds = new Set(opts.previous.skinlines.map((l) => l.id));
  const oldUniverseIds = new Set(opts.previous.universes.map((u) => u.id));

  return {
    skins: Object.keys(opts.current.skins).filter((i) => !oldSkinIds.has(i)),
    champions: opts.current.champions
      .map((c) => c.id)
      .filter((i) => !oldChampionIds.has(i)),
    skinlines: opts.current.skinlines
      .map((l) => l.id)
      .filter((i) => !oldSkinlineIds.has(i)),
    universes: opts.current.universes
      .map((u) => u.id)
      .filter((i) => !oldUniverseIds.has(i)),
  };
}
