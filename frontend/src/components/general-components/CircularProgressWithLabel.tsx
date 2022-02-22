import React from "react";

import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import CircularProgress, {
  CircularProgressProps,
} from "@mui/material/CircularProgress";

export default function CircularProgressWithLabel(
  props: CircularProgressProps & { value: number }
) {
  return (
    <Box sx={{ position: "relative", display: "inline-flex" }}>
      <CircularProgress variant='determinate' {...props} />
      <Box
        top={0}
        left={0}
        bottom={0}
        right={0}
        position='absolute'
        display='flex'
        alignItems='center'
        justifyContent='center'
      >
        <Typography
          variant='h6'
          component='div'
          color='#f5f0f6'
          fontWeight={200}
        >
          {`${Math.round(props.value)}%`}
        </Typography>
      </Box>
    </Box>
  );
}
