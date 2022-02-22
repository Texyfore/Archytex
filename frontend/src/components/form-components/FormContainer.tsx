import React from "react";

import Box from "@mui/material/Box";

import MaxHeightContainer from "../general-components/MaxHeightContainter";
import FormHeader from "./FormHeader";
import FormPaper from "./FormPaper";

interface Props {
  title: string;
  children?: JSX.Element | JSX.Element[];
}

export default function FormContainer({ title, children }: Props) {
  return (
    <MaxHeightContainer
      display='flex'
      justifyContent='center'
      alignItems='center'
    >
      <FormPaper>
        <FormHeader title={title} />
        <Box
          display='flex'
          flexDirection='column'
          alignItems='center'
          paddingX={{ sm: 0, md: 6 }}
          mb={4}
        >
          {children}
        </Box>
      </FormPaper>
    </MaxHeightContainer>
  );
}
