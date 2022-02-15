import React from "react";

import { Link as L } from "react-router-dom";

import { useTranslation } from "react-i18next";

import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";

import { NavigateNext } from "@mui/icons-material";

import Footer from "../components/general-components/Footer";
import MaxHeightContainer from "../components/general-components/MaxHeightContainter";

import successImage from "../img/illustrations/successful.svg";

export default function SuccessfulRegistration() {
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
          <img src={successImage} alt='registration_success' width={400} />
          <Box paddingX={{ xs: 2, md: 0 }} maxWidth='400px'>
            <Typography variant='h4' component='h1' color='primary' mb='50px'>
              {t("registration_success")}
            </Typography>
            <Typography variant='body1' textAlign='justify'>
              {t("registration_success_paragraph")}
            </Typography>
            <Box alignSelf='left' mt={5}>
              <Button
                variant='outlined'
                component={L}
                to='/login'
                endIcon={<NavigateNext />}
              >
                {t("login")}
              </Button>
            </Box>
          </Box>
        </Box>
      </MaxHeightContainer>
      <Footer />
    </>
  );
}
