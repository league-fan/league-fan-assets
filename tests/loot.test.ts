import { describe, expect, it } from "vitest";
import { filterLootChests } from "../src/transforms/loot.js";

describe("filterLootChests", () => {
  it("keeps only CHEST_n items from LootItems wrapper", () => {
    const raw = {
      LootItems: [
        { id: "CHEST_224", name: "Masterwork", image: "/lol-game-data/assets/assets/loot/a.png", mappedStoreId: 2 },
        { id: "MATERIAL_KEY", name: "Key", image: "/x.png", mappedStoreId: 1 },
        { id: "CHEST_1", name: "Chest", image: "/lol-game-data/assets/assets/loot/b.png", mappedStoreId: 1 },
      ],
    };
    const out = filterLootChests(raw, { patch: "latest" });
    expect(out).toHaveLength(2);
    expect(out.map((x) => x.id)).toEqual(["CHEST_1", "CHEST_224"]);
    expect(out[0]!.image).toMatch(/^https:\/\/raw\.communitydragon\.org\//);
  });
});
