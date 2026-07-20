export interface Universe {
  id: number;
  name: string;
  description: string;
  /** Absolute image URL after transform. */
  imagePath: string;
  skinSets: number[];
}
