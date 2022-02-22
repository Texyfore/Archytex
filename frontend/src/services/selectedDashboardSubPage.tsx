import React, { useContext } from "react";

type SubPage = "projects" | "settings";

const SubPageContext = React.createContext<
  [SubPage, React.Dispatch<React.SetStateAction<SubPage>>]
>(["projects", () => {}]);

function useSubPage() {
  return useContext(SubPageContext);
}

function SubPageProvider({ children }: { children: JSX.Element }): JSX.Element {
  const value: [SubPage, React.Dispatch<React.SetStateAction<SubPage>>] =
    React.useState("projects" as SubPage);

  return (
    <SubPageContext.Provider value={value}>{children}</SubPageContext.Provider>
  );
}

export { SubPageProvider, SubPageContext, useSubPage };
export type { SubPage };
