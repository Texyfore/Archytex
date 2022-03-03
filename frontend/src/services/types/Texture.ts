import Category from "../libraries/Category";

export default interface Texture {
  id: number;
  name: string;
  thumbnail: string;
  categories: Category[];
}
