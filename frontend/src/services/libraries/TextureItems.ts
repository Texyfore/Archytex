import brownPlanks from "../../img/texture_thumbnails/brown_planks_07.jpg";
import concreteFloor from "../../img/texture_thumbnails/concrete_floor_worn_001.jpg";
import concreteWall from "../../img/texture_thumbnails/concrete_wall_001.jpg";
import largeFloorTiles from "../../img/texture_thumbnails/large_floor_tiles_02.jpg";
import redBrick from "../../img/texture_thumbnails/red_brick_03.jpg";
import roof from "../../img/texture_thumbnails/roof_09.jpg";
import weatheredBrownPlanks from "../../img/texture_thumbnails/weathered_brown_planks.jpg";

import Texture from "../types/Texture";

export default function getTextures(): Texture[] {
  const textures: Texture[] = [
    {
      id: 1,
      name: "Concrete floor",
      categories: ["Concrete", "Dirty"],
      thumbnail: concreteFloor,
    },
    {
      id: 2,
      name: "Large floor tiles",
      categories: ["Rock", "Dirty"],
      thumbnail: largeFloorTiles,
    },
    {
      id: 3,
      name: "Red brick",
      categories: ["Brick", "Dirty"],
      thumbnail: redBrick,
    },
    {
      id: 4,
      name: "Brown planks",
      categories: ["Wood", "Clean"],
      thumbnail: brownPlanks,
    },
    {
      id: 5,
      name: "Weathered brown planks",
      categories: ["Wood", "Dirty"],
      thumbnail: weatheredBrownPlanks,
    },
    {
      id: 6,
      name: "Concrete wall",
      categories: ["Concrete", "Clean"],
      thumbnail: concreteWall,
    },
    {
      id: 7,
      name: "Roof",
      categories: ["Dirty"],
      thumbnail: roof,
    },
  ];

  return textures;
}
