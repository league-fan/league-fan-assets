import { describe, expect, it } from "vitest";
import { transformSkins } from "../src/transforms/skins.js";
import { transformChampions } from "../src/transforms/champions.js";
import { splitSkinId } from "../src/helpers/split-skin-id.js";
import type { Skins } from "../src/types/skins.js";

describe("transformChampions", () => {
  it("filters -1, sorts, substitutes key", () => {
    const out = transformChampions(
      [
        {
          id: -1,
          name: "None",
          alias: "None",
          squarePortraitPath: "/lol-game-data/assets/v1/champion-icons/-1.png",
          roles: [],
        },
        {
          id: 62,
          name: "Wukong",
          alias: "MonkeyKing",
          squarePortraitPath: "/lol-game-data/assets/v1/champion-icons/62.png",
          roles: ["fighter"],
        },
        {
          id: 1,
          name: "Annie",
          alias: "Annie",
          squarePortraitPath: "/lol-game-data/assets/v1/champion-icons/1.png",
          roles: ["mage"],
        },
      ],
      { patch: "pbe" },
    );
    expect(out).toHaveLength(2);
    expect(out[0]!.name).toBe("Annie");
    expect(out.find((c) => c.id === 62)!.key).toBe("wukong");
    expect(out[0]!.squarePortraitPath).toMatch(/^https:\/\//);
  });
});

describe("transformSkins", () => {
  it("prefixes base skins and expands quest tiers", () => {
    const raw = {
      "147001": {
        id: 147001,
        isBase: false,
        name: "K/DA ALL OUT Seraphine",
        splashPath: "/lol-game-data/assets/v1/champion-splashes/147001.jpg",
        uncenteredSplashPath: "/lol-game-data/assets/v1/champion-splashes/uncentered/147001.jpg",
        tilePath: "/lol-game-data/assets/v1/champion-tiles/147001.jpg",
        loadScreenPath: "/lol-game-data/assets/ASSETS/Characters/x.png",
        skinType: "",
        rarity: "kLegendary",
        isLegacy: false,
        splashVideoPath: null,
        collectionSplashVideoPath: null,
        featuresText: null,
        emblems: null,
        regionRarityId: 0,
        rarityGemPath: null,
        skinLines: [{ id: 1 }],
        description: null,
        questSkinInfo: {
          name: "q",
          productType: "t",
          collectionDescription: "",
          descriptionInfo: [],
          splashPath: "",
          uncenteredSplashPath: "",
          tilePath: "",
          collectionCardPath: "",
          tiers: [
            {
              id: 147002,
              name: "Indie",
              stage: 1,
              description: "d",
              splashPath: "/lol-game-data/assets/v1/champion-splashes/147002.jpg",
              uncenteredSplashPath: "",
              tilePath: "",
              loadScreenPath: "",
              shortName: "Indie",
              splashVideoPath: null,
              collectionSplashVideoPath: null,
            },
          ],
        },
      },
      "1000": {
        id: 1000,
        isBase: true,
        name: "Annie",
        splashPath: "/lol-game-data/assets/v1/champion-splashes/1000.jpg",
        uncenteredSplashPath: "",
        tilePath: "",
        loadScreenPath: "",
        skinType: "",
        rarity: "kNoRarity",
        isLegacy: false,
        splashVideoPath: null,
        collectionSplashVideoPath: null,
        featuresText: null,
        emblems: null,
        regionRarityId: 0,
        rarityGemPath: null,
        skinLines: null,
        description: null,
      },
    } as unknown as Skins;

    const out = transformSkins(raw, { patch: "pbe" });
    expect(out["1000"]!.name).toBe("Original Annie");
    expect(out["147002"]).toBeDefined();
    expect(out["147002"]!.name).toBe("Indie");
    expect(out["147001"]!.splashPath).toMatch(/^https:\/\/raw\.communitydragon\.org\/pbe\//);
  });
});

describe("splitSkinId", () => {
  it("splits champ and skin index", () => {
    expect(splitSkinId(147001)).toEqual({ champId: 147, skinIndex: 1 });
  });
});
