import Category from "../libraries/Category";

export default interface Prop {
  id: number;
  name: string;
  thumbnail: string;
  categories: Category[];
}
