import { Box, Tooltip, Typography } from '@mui/material';
import React from 'react';
import ArchytexIcon from './ArchytexIcon';

const ArchytexLogoWithText = () => (
    <Tooltip title='Archytex version 0.0.1' placement='bottom-start'>
        <Box display={{ xs: "none", md: "flex" }} alignItems='center'>
            <ArchytexIcon />
            <Typography
                variant='h6'
                component='h2'
                fontSize='1em'
                sx={{ display: { xs: "none", sm: "block" } }}
            >
                ARCHYTEX
            </Typography>
        </Box>
    </Tooltip>
);

export default ArchytexLogoWithText;