import { Box, Divider, Link, Typography } from "@mui/material";
import React from "react";
import ArchytexIcon from "./ArchytexIcon";

export default function ArchytexFooter() {
  return (
    <Box>
      <Box
        display='flex'
        flexDirection={{ xs: "column", md: "row" }}
        justifyContent='space-evenly'
        alignItems={{ xs: "center", md: "start" }}
        paddingX={{ xs: 1, md: 10 }}
        paddingY={8}
        gap={{ xs: 1, md: 5 }}
      >
        {/* Socials */}
        <Box display='flex' alignItems='center'>
          <ArchytexIcon size={40} />
          <Typography gutterBottom variant='body1' color='GrayText'>
            ARCHYTEX
          </Typography>
          {/* Socials */}
        </Box>

        {/* Product */}
        <Box
          display='flex'
          flexDirection='column'
          justifyContent='space-evenly'
          gap={1}
        >
          <Typography gutterBottom variant='subtitle2' color='GrayText'>
            PRODUCT
          </Typography>
          <Link variant='body2' href='#'>
            Features
          </Link>
          <Link variant='body2' href='#'>
            Pricing
          </Link>
          <Link variant='body2' href='#'>
            FAQ
          </Link>
        </Box>

        {/* Company */}
        <Box
          display='flex'
          flexDirection='column'
          justifyContent='space-evenly'
          gap={1}
        >
          <Typography gutterBottom variant='subtitle2' color='GrayText'>
            COMPANY
          </Typography>
          <Link variant='body2' href='#'>
            About
          </Link>
          <Link variant='body2' href='#'>
            Contact
          </Link>
        </Box>
      </Box>
      <Box>
        <Typography textAlign='center' color='GrayText' paddingBottom={5}>
          <small>Copyright &copy; {new Date().getFullYear()}, Archytex</small>
        </Typography>
      </Box>
    </Box>
  );
}
