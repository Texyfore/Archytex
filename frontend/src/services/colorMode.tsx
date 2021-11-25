import React, { useContext, useReducer } from "react";

enum ColorMode {
    Light,
    Dark
}


function invert(mode: ColorMode): ColorMode {
    switch (mode) {
        case ColorMode.Light:
            return ColorMode.Dark;
        case ColorMode.Dark:
            return ColorMode.Light;
    }
}
function modeToString(mode: ColorMode): "light" | "dark"{
    switch (mode) {
        case ColorMode.Light:
            return "light";
        case ColorMode.Dark:
            return "dark";
    }
}
export { ColorMode, invert, modeToString};

export const ColorModeContext = React.createContext<[ColorMode, React.DispatchWithoutAction] | undefined>(undefined);

function colorModeReducer(state: ColorMode):ColorMode {
    return invert(state);
}

export const ColorModeProvider = ({ children }: React.PropsWithChildren<{}>) => {
    const [state, dispatch] = useReducer(colorModeReducer, ColorMode.Light);
    return <ColorModeContext.Provider value={[state, dispatch]}>
        {children}
    </ColorModeContext.Provider>
}

export const useColorMode = () => {
    const context = useContext(ColorModeContext);
    if (context === undefined) {
        throw new Error("useColorMode must be within ColorModeProvider");
    }
    return context;
}