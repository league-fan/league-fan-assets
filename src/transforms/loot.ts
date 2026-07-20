import type { LootItem } from "../types/cosmetics.js";
import type { Lang, Patch } from "../types/common.js";
import { assetUrl } from "./paths.js";

const CHEST_ID = /CHEST_[0-9]{1,4}$/;

export function filterLootChests(
  raw: unknown,
  opts: { lang?: Lang; patch?: Patch } = {},
): LootItem[] {
  const patch = opts.patch ?? "latest";
  let items: Record<string, unknown>[] = [];

  if (Array.isArray(raw)) {
    items = raw as Record<string, unknown>[];
  } else if (raw && typeof raw === "object" && "LootItems" in raw) {
    items = (raw as { LootItems: Record<string, unknown>[] }).LootItems ?? [];
  }

  const filtered = items.filter((el) => {
    const id = String(el.id ?? "");
    return CHEST_ID.test(id);
  });

  const mapped: LootItem[] = filtered.map((el) => {
    const image = assetUrl(String(el.image ?? el.imagePath ?? ""), {
      patch,
      lang: opts.lang,
      category: "loot",
    });
    return {
      ...el,
      id: String(el.id),
      name: String(el.name ?? ""),
      description: String(el.description ?? ""),
      image,
    } as LootItem;
  });

  return mapped.sort(
    (a, b) => (a.mappedStoreId ?? 0) - (b.mappedStoreId ?? 0),
  );
}
