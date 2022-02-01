import React, { useState } from "react";
import {
  Menu,
  List,
  ListItem,
  Box,
  Typography,
  Stack,
  Slider,
} from "@mui/material";
import { Grid4x4Rounded, Grid3x3Rounded } from "@mui/icons-material";
import { useTranslation } from "react-i18next";

interface GridSettingsMenuProps {
  gridAnchorEl: Element | ((element: Element) => Element) | null | undefined;
  gridMenuOpen: boolean;
  handleGridMenuClose: () => void;
  gridStep: number;
  handleGridStepChange: (e: any) => void;
}

export default function GridSettingsMenu({
  gridAnchorEl,
  gridMenuOpen,
  gridStep,
  handleGridMenuClose,
  handleGridStepChange,
}: GridSettingsMenuProps) {
  const { t } = useTranslation();

  const [value, setValue] = useState<number>(2);

  const handleChange = (e: any) => {
    setValue(e.target.value);
    handleGridStepChange(e);
  };

  const calculateValue = (value: number) => {
    return 10 ** value;
  };
  const valueLabelFormat = (value: number) => {
    return calculateValue(value) >= 100
      ? `${calculateValue(value) / 100} m`
      : `${calculateValue(value)} cm`;
  };

  return (
    <Menu
      anchorEl={gridAnchorEl}
      id='grid-menu'
      open={gridMenuOpen}
      onClose={handleGridMenuClose}
      PaperProps={{
        elevation: 0,
        sx: {
          overflow: "visible",
          filter: "drop-shadow(0px 2px 8px rgba(0,0,0,0.5))",
          mt: 1.5,
          borderRadius: 2,
          "&:before": {
            content: '""',
            display: "block",
            position: "absolute",
            top: 0,
            right: 25,
            width: 10,
            height: 10,
            bgcolor: "paper.background",
            transform: "translateY(-50%) rotate(45deg)",
            zIndex: 0,
          },
        },
      }}
      transformOrigin={{ horizontal: "right", vertical: "top" }}
      anchorOrigin={{ horizontal: "right", vertical: "bottom" }}
    >
      <List dense>
        <ListItem>
          <Box>
            <Typography gutterBottom>{t("grid_resolution")}</Typography>
            <Box width={150}>
              <Stack
                spacing={2}
                direction='row'
                sx={{ mb: 1 }}
                alignItems='center'
              >
                <Grid4x4Rounded fontSize='small' />
                <Slider
                  value={value}
                  size='small'
                  step={null}
                  min={0}
                  max={3}
                  // scale={calculateValue}
                  valueLabelFormat={() => valueLabelFormat(value)}
                  marks={[
                    { value: 0, label: "" },
                    { value: 1, label: "" },
                    { value: 2, label: "" },
                    { value: 3, label: "" },
                  ]}
                  onChange={handleChange}
                  valueLabelDisplay='auto'
                />
                <Grid3x3Rounded />
              </Stack>
            </Box>
          </Box>
        </ListItem>
      </List>
    </Menu>
  );
}
