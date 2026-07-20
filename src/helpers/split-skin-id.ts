export interface SplitSkinId {
  champId: number;
  skinIndex: number;
}

/** Skin id layout: champId * 1000 + skinIndex */
export function splitSkinId(id: number): SplitSkinId {
  return {
    champId: Math.floor(id / 1000),
    skinIndex: id % 1000,
  };
}

/** Legacy tuple form used by some scrapers: [champId, skinIndex] */
export function splitId(id: number): [number, number] {
  const { champId, skinIndex } = splitSkinId(id);
  return [champId, skinIndex];
}
