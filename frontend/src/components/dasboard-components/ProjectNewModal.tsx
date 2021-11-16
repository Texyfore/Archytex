import { Close } from "@mui/icons-material";
import { Backdrop, Box, Button, Fade, IconButton, Modal, TextField, Typography } from "@mui/material";

const modalStyle = {
    position: "absolute" as "absolute",
    top: "50%",
    left: "50%",
    transform: "translate(-50%, -50%)",
    width: { xs: 400, sm: 500, md: 600, lg: 600 },
    bgcolor: "background.paper",
    border: "1px solid #14151A",
    boxShadow: 24,
    p: 4,
};

interface Parameters {
    modalOpen: boolean,
    handleModalOpen: () => void,
    handleModalClose: () => void
}

export default function ProjectNewModal({ handleModalClose, handleModalOpen, modalOpen, ...params }: Parameters) {
    return <Modal
        open={modalOpen}
        onClose={handleModalClose}
        closeAfterTransition
        BackdropComponent={Backdrop}
        BackdropProps={{
            timeout: 500,
        }}
    >
        <Fade in={modalOpen}>
            <Box
                sx={modalStyle}
                borderRadius={4}
                display='flex'
                flexDirection='column'
                justifyContent='space-between'
            >
                <Box display='flex' justifyContent='space-between'>
                    <Typography
                        id='transition-modal-title'
                        variant='h6'
                        component='h2'
                    >
                        Create new project
                    </Typography>
                    <IconButton onClick={handleModalClose}>
                        <Close />
                    </IconButton>
                </Box>
                <Box display='flex' flexDirection='column' marginBottom={3}>
                    <TextField
                        required
                        id='standard-required'
                        label='Project name'
                        variant='standard'
                        margin='normal'
                    />
                </Box>
                <Box>
                    <Button size='large' variant='contained'>
                        Create
                    </Button>
                </Box>
            </Box>
        </Fade>
    </Modal>;
}