import React, { useEffect, useMemo, useState } from "react";

import ReCAPTCHA, { ReCAPTCHAProps } from "react-google-recaptcha";

import { modeToString, useColorMode } from "../../services/colorMode";

export default function ColoredReCaptcha(args: ReCAPTCHAProps) {
  const [colorMode] = useColorMode();
  const colorName = useMemo(() => modeToString(colorMode), [colorMode]);
  const [reloadKey, setReloadKey] = useState(0);
  useEffect(() => {
    return () => {
      setReloadKey(reloadKey + 1);
    };
  }, [colorName, reloadKey]);

  return <ReCAPTCHA key={reloadKey} {...args} theme={colorName} />;
}
