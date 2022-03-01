export default interface Prop {
  id: number;
  name: string;
  thumbnail: string;
  categories: PropFilterOptions[];
}

export type PropFilterOptions = "Furniture" | "Decoration" | "Table" | "Chair";
