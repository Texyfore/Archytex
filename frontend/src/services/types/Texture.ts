export default interface Texture {
  id: number;
  name: string;
  thumbnail: string;
  categories: TextureFilterOptions[];
}

export type TextureFilterOptions =
  | "Brick"
  | "Wood"
  | "Concrete"
  | "Rock"
  | "Dirty"
  | "Clean";
