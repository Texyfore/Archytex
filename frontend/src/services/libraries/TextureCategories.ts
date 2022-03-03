import Category from "./Category";
enum TextureFilterOptions {
  brick = "Brick",
  wood = "Wood",
  concrete = "Concrete",
  rock = "Rock",
  dirty = "Dirty",
  clean = "Clean",
}
export default function getTextureCategories(): Category[] {
  const categories: Category[] = [
    {
      id: 0,
      name: "Brick",
    },
    {
      id: 1,
      name: "Wood",
    },
    {
      id: 2,
      name: "Concrete",
    },
    {
      id: 3,
      name: "Rock",
    },

    {
      id: 4,
      name: "Dirty",
    },
    {
      id: 5,
      name: "Clean",
    },
  ];

  return categories;
}
