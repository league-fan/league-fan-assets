export interface Champion {
  id: number;
  name: string;
  alias: string;
  /** Absolute portrait URL after transform. */
  squarePortraitPath: string;
  roles: string[];
  /** Normalized alias key (e.g. monkeyking → wukong). */
  key: string;
}

export type Role =
  | "assassin"
  | "fighter"
  | "mage"
  | "marksman"
  | "support"
  | "tank"
  | (string & {});
