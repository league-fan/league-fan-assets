import { RarityEnum } from "../types/skins.js";
import { dataRoot } from "../sources/cdragon.js";
import type { Patch } from "../types/common.js";

export const raritiesMap: Partial<Record<RarityEnum, [string, string]>> = {
  [RarityEnum.KUltimate]: ["ultimate.png", "Ultimate"],
  [RarityEnum.KMythic]: ["mythic.png", "Mythic"],
  [RarityEnum.KLegendary]: ["legendary.png", "Legendary"],
  [RarityEnum.KEpic]: ["epic.png", "Epic"],
  [RarityEnum.KTranscendent]: ["transcendent.png", "Transcendent"],
  [RarityEnum.KRare]: ["rare.png", "Rare"],
};

export function getRarityUrl(
  rarity: string,
  opts?: { patch?: Patch },
): { imgUrl: string; name: string } | null {
  const key = rarity as RarityEnum;
  const info = raritiesMap[key];
  if (!info) return null;
  const [imgName, name] = info;
  const imgUrl = `${dataRoot({ patch: opts?.patch ?? "pbe", lang: "default" })}/v1/rarity-gem-icons/${imgName}`;
  return { imgUrl, name };
}

export function modelviewerUrl(skinId: number): string {
  return `https://www.modelviewer.lol/en-US/model-viewer?id=${skinId}`;
}
