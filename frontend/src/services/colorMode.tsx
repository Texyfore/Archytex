import React, { useContext, useReducer } from "react";
import { ThemeProvider, createTheme } from "@mui/material/styles";
import { useTranslation } from "react-i18next";

enum ColorMode {
  Light,
  Dark,
}

function GetTheme(mode: ColorMode): any {
  // Set different font for Japanese language
  const { i18n } = useTranslation();
  const fontFamily = React.useMemo(() => {
    switch (i18n.language) {
      case "jp":
        return "Noto Sans JP";

      default:
        return "Nagoda";
    }
  }, [i18n.language]);

  // Main theme
  return React.useMemo(
    () =>
      createTheme({
        palette: {
          mode: modeToString(mode),
          primary: {
            main: "#39A0ED",
          },
          secondary: {
            main: "#FF6584",
          },
          text: {
            primary: mode === ColorMode.Dark ? "#F4F4F4" : "#1B1B1A",
          },
          background: {
            default: mode === ColorMode.Dark ? "#1B1B1A" : "#F4F4F4",
            paper: mode === ColorMode.Dark ? "#1B1B1A" : "#F4F4F4",
          },
          divider: mode === ColorMode.Dark ? "#1F1F1F" : "#EBE7EC",
        },
        shape: {
          borderRadius: 2,
        },
        typography: {
          fontFamily: fontFamily,
        },
      }),
    [mode, fontFamily]
  );
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
    modeFromString(localStorage.getItem("colormode")) ?? ColorMode.Light;
  const [state, dispatch] = useReducer(colorModeReducer, mode);
  const theme = GetTheme(state);
  return (
    <ThemeProvider theme={theme}>
      <ColorModeContext.Provider value={[state, dispatch]}>
        {children}
      </ColorModeContext.Provider>
    </ThemeProvider>
  );
};

export const ForceDarkProvider = ({
  children,
}: React.PropsWithChildren<{}>) => {
  const theme = GetTheme(ColorMode.Light);
  return (
    <ThemeProvider theme={theme}>
      <ColorModeContext.Provider value={[ColorMode.Light, () => {}]}>
        {children}
      </ColorModeContext.Provider>
    </ThemeProvider>
  );
};

export const useColorMode = () => {
  const context = useContext(ColorModeContext);
  if (context === undefined) {
    throw new Error("useColorMode must be within ColorModeProvider");
  }
  return context;
};
