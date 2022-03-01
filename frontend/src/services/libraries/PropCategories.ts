import Category from "./Category";

export default function getPropCategories(): Category[] {
  const categories: Category[] = [
    {
      id: 0,
      name: "Furniture",
    },
    {
      id: 1,
      name: "Decoration",
    },
    {
      id: 2,
      name: "Table",
    },
    {
      id: 3,
      name: "Chair",
    },
  ];

  return categories;
}
