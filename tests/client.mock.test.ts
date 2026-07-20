import { describe, expect, it, vi } from "vitest";
import { createClient } from "../src/client.js";
import { releaseDataUrl, releaseAssetName } from "../src/sources/release.js";

describe("release urls", () => {
  it("flattens nested paths", () => {
    expect(releaseAssetName("default/skins.json")).toBe("default__skins.json");
    expect(
      releaseDataUrl({ category: "summoner-icons", lang: "zh_cn" }),
    ).toContain("zh_cn__summoner-icons.json");
  });
});

describe("createClient with mock fetch", () => {
  it("loads champions via cdragon and transforms", async () => {
    const fetchMock = vi.fn(async (url: string) => {
      expect(url).toContain("champion-summary.json");
      return {
        ok: true,
        status: 200,
        statusText: "OK",
        json: async () => [
          {
            id: 1,
            name: "Annie",
            alias: "Annie",
            squarePortraitPath:
              "/lol-game-data/assets/v1/champion-icons/1.png",
            roles: ["mage"],
          },
        ],
      } as Response;
    });

    const client = createClient({
      lang: "default",
      source: { kind: "cdragon", patch: "pbe" },
      fetch: fetchMock as unknown as typeof fetch,
    });

    const champs = await client.champions();
    expect(champs).toHaveLength(1);
    expect(champs[0]!.key).toBe("annie");
    expect(champs[0]!.squarePortraitPath).toMatch(/^https:\/\//);
  });

  it("returns release JSON as-is", async () => {
    const payload = [{ id: 1, title: "x", imagePath: "https://x", description: "" }];
    const urls: string[] = [];
    const fetchMock = async (input: RequestInfo | URL) => {
      urls.push(String(input));
      return {
        ok: true,
        status: 200,
        statusText: "OK",
        json: async () => payload,
      } as Response;
    };

    const client = createClient({
      lang: "default",
      source: { kind: "release" },
      fetch: fetchMock as unknown as typeof fetch,
    });

    const icons = await client.summonerIcons();
    expect(icons).toEqual(payload);
    expect(urls[0]).toContain("default__summoner-icons.json");
  });
});
