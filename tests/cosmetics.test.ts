import { describe, expect, it } from "vitest";
import { transformSummonerIcons } from "../src/transforms/cosmetics.js";

describe("transformSummonerIcons", () => {
  it("flattens description and rewrites image", () => {
    const out = transformSummonerIcons(
      [
        {
          id: 2,
          title: "B",
          imagePath: "/lol-game-data/assets/v1/profile-icons/2.jpg",
          descriptions: [{ region: "riot", description: "desc" }],
        },
        {
          id: 1,
          title: "A",
          imagePath: "/lol-game-data/assets/v1/profile-icons/1.jpg",
        },
      ],
      { patch: "latest", lang: "default" },
    );
    expect(out[0]!.id).toBe(2); // sorted desc
    expect(out[0]!.description).toBe("desc");
    expect(out[1]!.description).toBe("");
    expect(out[0]!.imagePath).toMatch(/profile-icons\/2\.jpg$/);
  });
});
