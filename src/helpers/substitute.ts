/** Champion alias substitutions used by community tools / wiki. */
export const ALIAS_SUBSTITUTIONS: Record<string, string> = {
  monkeyking: "wukong",
};

export function substitute(
  value: string,
  sets: Record<string, string> = ALIAS_SUBSTITUTIONS,
): string {
  return sets[value] ?? value;
}
