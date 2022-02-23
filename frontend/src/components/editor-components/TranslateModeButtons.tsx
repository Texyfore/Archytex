import React from "react";

import { useTranslation } from "react-i18next";

import Box from "@mui/material/Box";
import ToggleButton from "@mui/material/ToggleButton";
import ToggleButtonGroup from "@mui/material/ToggleButtonGroup";
import Tooltip from "@mui/material/Tooltip";

import MoveTransformModeIcon from "./icons/MoveTransformModeIcon";
import RotateTransformModeIcon from "./icons/RotateTransformModeIcon";
import ScaleTransformModeIcon from "./icons/ScaleTransformModeIcon";
import SelectTransformModeIcon from "./icons/SelectTransformModeIcon";

export default function TranslateModeButtons() {
  const { t } = useTranslation();
  const selectTooltipText = t("select_translate_mode");
  const moveTooltipText = t("move_translate_mode");
  const rotateTooltipText = t("rotate_translate_mode");
  const scaleTooltipText = t("scale_translate_mode");

  return (
    <Box position='absolute' top='40vh' left={10}>
      <ToggleButtonGroup
        value={() => {}}
        exclusive
        onChange={() => {}}
        color='primary'
        size='small'
        orientation='vertical'
      >
        <ToggleButton value='select'>
          <Tooltip title={selectTooltipText} placement='right'>
            <Box marginTop={0.8} width={36} height={30}>
              <SelectTransformModeIcon />
            </Box>
          </Tooltip>
        </ToggleButton>
        <ToggleButton value='move'>
          <Tooltip title={moveTooltipText} placement='right'>
            <Box marginTop={0.8} width={36} height={30}>
              <MoveTransformModeIcon />
            </Box>
          </Tooltip>
        </ToggleButton>
        <ToggleButton value='rotate'>
          <Tooltip title={rotateTooltipText} placement='right'>
            <Box marginTop={0.8} width={36} height={30}>
              <RotateTransformModeIcon />
            </Box>
          </Tooltip>
        </ToggleButton>
        <ToggleButton value='scale'>
          <Tooltip title={scaleTooltipText} placement='right'>
            <Box marginTop={0.8} width={36} height={30}>
              <ScaleTransformModeIcon />
            </Box>
          </Tooltip>
        </ToggleButton>
      </ToggleButtonGroup>
    </Box>
  );
}
