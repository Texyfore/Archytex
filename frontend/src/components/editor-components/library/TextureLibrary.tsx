export { }

// import React from "react";

// import Box from "@mui/material/Box";

// import LibraryCard from "./LibraryCard";

// import Texture from "../../../services/types/Texture";
// import Category from "../../../services/libraries/Category";
// import getTextures from "../../../services/libraries/TextureItems";

// interface Props {
//   selected: Texture | undefined;
//   handleSelectionChange: (texture: Texture | undefined) => void;
//   query: string;
//   checkedCategories: Category[];
// }

// export default function TextureLibrary({
//   selected,
//   handleSelectionChange,
//   query,
//   checkedCategories,
// }: Props) {
//   const textures = getTextures();

//   const matchesFilter = (texture: Texture) => {
//     return texture.categories.some((category) =>
//       checkedCategories.some(
//         (checkedCategory) => checkedCategory.id === category.id
//       )
//     );
//   };
//   return (
//     <Box
//       display='flex'
//       flexWrap='wrap'
//       gap={1}
//       alignItems='start'
//       justifyContent='space-evenly'
//       mt={3}
//     >
//       {textures
//         .filter(
//           (t) =>
//             t.name.toLowerCase().includes(query.toLowerCase()) &&
//             matchesFilter(t)
//         )
//         .map((texture, index) => (
//           <LibraryCard
//             key={index}
//             cardType='texture'
//             item={texture}
//             isSelected={
//               selected === undefined ? false : selected.id === texture.id
//             }
//             handleSelectionChange={handleSelectionChange}
//           />
//         ))}
//     </Box>
//   );
// }
