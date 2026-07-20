import type { Champion } from "../types/champion.js";
import type { Lang, Patch } from "../types/common.js";
import { substitute } from "../helpers/substitute.js";
import { assetUrl } from "./paths.js";

export function transformChampions(
  raw: unknown[],
  opts: { lang?: Lang; patch?: Patch } = {},
): Champion[] {
  const patch = opts.patch ?? "pbe";
  return (raw as Partial<Champion>[])
    .filter((d) => d.id !== -1 && d.id != null)
    .map((a) => {
      const alias = a.alias ?? "";
      return {
        id: Number(a.id),
        name: a.name ?? "",
        alias,
        squarePortraitPath: assetUrl(a.squarePortraitPath ?? "", {
          patch,
          lang: opts.lang,
          category: "champions",
        }),
        roles: a.roles ?? [],
        key: substitute(alias.toLowerCase()),
      } satisfies Champion;
    })
    .sort((a, b) => (a.name > b.name ? 1 : -1));
}
