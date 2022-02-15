import React from "react";

import { Link as L } from "react-router-dom";

import { useTranslation } from "react-i18next";

import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";

import { Home } from "@mui/icons-material";

import Footer from "../components/general-components/Footer";
import MaxHeightContainer from "../components/general-components/MaxHeightContainter";

import notFoundImage from "../img/illustrations/summer.svg";

export default function PageNotFound() {
  const { t } = useTranslation();

  return (
    <>
      <MaxHeightContainer
        display='flex'
        justifyContent='center'
        alignItems='center'
      >
        <Box
          display='flex'
          width='100%'
          justifyContent='center'
          gap={{ xs: 10, md: 20 }}
          flexWrap='wrap-reverse'
        >
          <img src={notFoundImage} alt='registration_success' width={400} />
          <Box paddingX={{ xs: 2, md: 0 }} maxWidth='400px'>
            <Typography variant='h2' component='h1' color='primary' mb='50px'>
              404
            </Typography>
            <Typography variant='h4' component='h1' color='primary' mb='50px'>
              {t("page_not_found")}
            </Typography>
            <Typography variant='body1' textAlign='justify'>
              {t("page_not_found_paragraph")}
            </Typography>
            <Box alignSelf='left' mt={5}>
              <Button
                variant='outlined'
                component={L}
                to='/'
                endIcon={<Home />}
              >
                {t("back_to_home")}
              </Button>
            </Box>
          </Box>
        </Box>
      </MaxHeightContainer>
      <Footer />
    </>
  );
}
