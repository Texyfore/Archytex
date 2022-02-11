import React from "react";

import DrawerHeader from "./DrawerHeader";
import DrawerNavButtonList from "./DrawerNavButtonList";
import DrawerBottomButtons from "./DrawerBottomButtons";

export default function DrawerContent() {
  return (
    <>
      <DrawerHeader />
      <DrawerNavButtonList />
      <DrawerBottomButtons />
    </>
  );
}
