import React from "react";
import { Box, Link, Typography } from "@mui/material";
import { SocialIcon } from "react-social-icons";
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
        <Box>
          <Box display='flex' alignItems='center'>
            <ArchytexIcon />
            <Typography variant='h6' color='GrayText'>
              ARCHYTEX
            </Typography>
          </Box>
          {/* Socials */}
          <Box display='flex' flexWrap='nowrap' gap={1} marginTop={4}>
            <SocialIcon
              style={{ height: 40, width: 40 }}
              bgColor='transparent'
              fgColor='GrayText'
              network='facebook'
              url='https://facebook.com'
            ></SocialIcon>
            <SocialIcon
              style={{ height: 40, width: 40 }}
              bgColor='transparent'
              fgColor='GrayText'
              network='instagram'
              url='https://instagram.com'
            ></SocialIcon>
            <SocialIcon
              style={{ height: 40, width: 40 }}
              bgColor='transparent'
              fgColor='GrayText'
              network='twitter'
              url='https://twitter.com'
            ></SocialIcon>
            <SocialIcon
              style={{ height: 40, width: 40 }}
              bgColor='transparent'
              fgColor='GrayText'
              network='github'
              url='https://github.com'
            ></SocialIcon>
          </Box>
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

      {/* Copyright */}
      <Box>
        <Typography textAlign='center' color='GrayText' paddingBottom={5}>
          <small>Copyright &copy; {new Date().getFullYear()}, Archytex</small>
        </Typography>
      </Box>
    </Box>
  );
}
