import React, { useContext, useReducer } from "react";

enum ColorMode {
  Light,
  Dark,
}

function invert(mode: ColorMode): ColorMode {
  switch (mode) {
    case ColorMode.Light:
      return ColorMode.Dark;
    case ColorMode.Dark:
      return ColorMode.Light;
  }
}
function modeToString(mode: ColorMode): "light" | "dark" {
  switch (mode) {
    case ColorMode.Light:
      return "light";
    case ColorMode.Dark:
      return "dark";
  }
}
function modeFromString(mode: string | null): ColorMode | null {
  switch (mode) {
    case "light":
      return ColorMode.Light;
    case "dark":
      return ColorMode.Dark;
    default:
      return null;
  }
}
export { ColorMode, invert, modeToString };

export const ColorModeContext = React.createContext<
  [ColorMode, React.DispatchWithoutAction] | undefined
>(undefined);

function colorModeReducer(state: ColorMode): ColorMode {
  const newState = invert(state);
  localStorage.setItem("colormode", modeToString(newState));
  return newState;
}

export const ColorModeProvider = ({
  children,
}: React.PropsWithChildren<{}>) => {
  let mode =
    modeFromString(localStorage.getItem("colormode")) ?? ColorMode.Dark;
  const [state, dispatch] = useReducer(colorModeReducer, mode);
  return (
    <ColorModeContext.Provider value={[state, dispatch]}>
      {children}
    </ColorModeContext.Provider>
  );
};

export const useColorMode = () => {
  const context = useContext(ColorModeContext);
  if (context === undefined) {
    throw new Error("useColorMode must be within ColorModeProvider");
  }
  return context;
};
