import { describe, expect, it } from "vitest";
import { assetUrl } from "../src/transforms/paths.js";

describe("assetUrl", () => {
  it("passthrough absolute https", () => {
    expect(assetUrl("https://example.com/a.png")).toBe(
      "https://example.com/a.png",
    );
  });

  it("rewrites lol-game-data paths to cdragon default", () => {
    const url = assetUrl(
      "/lol-game-data/assets/v1/profile-icons/0.jpg",
      { patch: "latest" },
    );
    expect(url).toBe(
      "https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/profile-icons/0.jpg",
    );
  });

  it("uses tencent CDN for zh_cn summoner icons", () => {
    const url = assetUrl(
      "/lol-game-data/assets/v1/profile-icons/1234.png",
      { patch: "latest", lang: "zh_cn", category: "summoner-icons" },
    );
    expect(url).toBe(
      "https://dlied1.qq.com/lolapp/lol/summoner/profileicon/1234.png",
    );
  });

  it("lowercases path segments", () => {
    const url = assetUrl(
      "/lol-game-data/assets/ASSETS/Characters/Aatrox/Skins/Base/Images/AatroxLoadScreen.JPG",
      { patch: "pbe" },
    );
    expect(url).toContain("/pbe/");
    expect(url).toBe(url.toLowerCase());
  });

  it("returns empty for nullish", () => {
    expect(assetUrl(null)).toBe("");
    expect(assetUrl(undefined)).toBe("");
  });
});
