export { }

// import React from "react";

// import Box from "@mui/material/Box";

// import LibraryCard from "./LibraryCard";

// import Prop from "../../../services/types/Prop";
// import Category from "../../../services/libraries/Category";
// import getProps from "../../../services/libraries/PropItems";

// interface Props {
//   selected: Prop | undefined;
//   handleSelectionChange: (prop: Prop | undefined) => void;
//   query: string;
//   checkedCategories: Category[];
// }
// export default function PropLibrary({
//   selected,
//   handleSelectionChange,
//   query,
//   checkedCategories,
// }: Props) {
//   const props = getProps();

//   const matchesFilter = (prop: Prop) => {
//     return prop.categories.some((category) =>
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
//       marginTop={3}
//     >
//       {props
//         .filter(
//           (p) =>
//             p.name.toLowerCase().includes(query.toLowerCase()) &&
//             matchesFilter(p)
//         )
//         .map((prop, index) => (
//           <LibraryCard
//             key={index}
//             cardType='prop'
//             item={prop}
//             isSelected={
//               selected === undefined ? false : selected.id === prop.id
//             }
//             handleSelectionChange={handleSelectionChange}
//           />
//         ))}
//     </Box>
//   );
// }
