import React from "react";

import DrawerHeader from "./DrawerHeader";
import DrawerNavButtonList from "./DrawerNavButtonList";
import DrawerBottomButtons from "./DrawerBottomButtons";

interface Props {
  handleClose: () => void;
}
export default function DrawerContent({ handleClose }: Props) {
  return (
    <>
      <DrawerHeader />
      <DrawerNavButtonList handleClose={handleClose} />
      <DrawerBottomButtons />
    </>
  );
}
