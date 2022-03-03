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
      categories: [
        {
          id: 2,
          name: "Concrete",
        },
        {
          id: 4,
          name: "Dirty",
        },
      ],
      thumbnail: concreteFloor,
    },
    {
      id: 2,
      name: "Large floor tiles",
      categories: [
        {
          id: 3,
          name: "Rock",
        },
        {
          id: 4,
          name: "Dirty",
        },
      ],
      thumbnail: largeFloorTiles,
    },
    {
      id: 3,
      name: "Red brick",
      categories: [
        {
          id: 0,
          name: "Brick",
        },
        {
          id: 4,
          name: "Dirty",
        },
      ],
      thumbnail: redBrick,
    },
    {
      id: 4,
      name: "Brown planks",
      categories: [
        {
          id: 1,
          name: "Wood",
        },
        {
          id: 5,
          name: "Clean",
        },
      ],
      thumbnail: brownPlanks,
    },
    {
      id: 5,
      name: "Weathered brown planks",
      categories: [
        {
          id: 1,
          name: "Wood",
        },
        {
          id: 4,
          name: "Dirty",
        },
      ],
      thumbnail: weatheredBrownPlanks,
    },
    {
      id: 6,
      name: "Concrete wall",
      categories: [
        {
          id: 2,
          name: "Concrete",
        },
        {
          id: 5,
          name: "Clean",
        },
      ],
      thumbnail: concreteWall,
    },
    {
      id: 7,
      name: "Roof",
      categories: [
        {
          id: 4,
          name: "Dirty",
        },
      ],
      thumbnail: roof,
    },
  ];

  return textures;
}
