import { KeyboardArrowDown, KeyboardArrowRight } from "@mui/icons-material";
import { styled } from "@mui/material/styles";
import {
  Accordion,
  AccordionDetails,
  AccordionSummary,
  Box,
  IconButton,
  LinearProgress,
  LinearProgressProps,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableRow,
  Typography,
} from "@mui/material";
import React, { SyntheticEvent } from "react";

const ProjectTableCell = styled(TableCell)(({ theme }) => ({
  padding: "none",
  margin: "none",
  borderBottom: "none",
}));

function LinearProgressWithLabel(
  props: LinearProgressProps & { value: number }
) {
  return (
    <Box sx={{ display: "flex", alignItems: "center" }}>
      <Box sx={{ width: "100%", mr: 1 }}>
        <LinearProgress variant='determinate' {...props} />
      </Box>
      <Box sx={{ minWidth: 35 }}>
        <Typography variant='body2' color='text.secondary'>{`${Math.round(
          props.value
        )}%`}</Typography>
      </Box>
    </Box>
  );
}

function createData(name: string, created: string) {
  return {
    name,
    created,
    renders: [
      {
        renderName: name + "- it's very long so it can be abbreviated",
        status: 100, //percentage
        renderTime: "1000 h 40 min 23 sec",
      },
      {
        renderName: name + "-project-render-2",
        status: 45, //percentage
        renderTime: "35 min 21 sec",
      },
    ],
  };
}

export default function ProjectRow(props: {
  id: number;
  row: ReturnType<typeof createData>;
  expanded: number | boolean;
  handleChange: (
    row: number
  ) =>
    | ((event: SyntheticEvent<Element, Event>, expanded: boolean) => void)
    | undefined;
}) {
  const { row, id, expanded, handleChange } = props;

  const [progress, setProgress] = React.useState(10);

  React.useEffect(() => {
    const timer = setInterval(() => {
      setProgress((prevProgress) =>
        prevProgress >= 100 ? 10 : prevProgress + 10
      );
    }, 800);
    return () => {
      clearInterval(timer);
    };
  }, []);

  return (
    <Accordion
      disableGutters
      elevation={0}
      expanded={expanded === id}
      onChange={handleChange(id)}
      sx={
        expanded === id
          ? {
              backgroundColor: "#1F1F1F",
              borderRadius: 4,
            }
          : {
              position: "static",
              borderRadius: 4,
              ".MuiAccordionSummary-root:hover": {
                backgroundColor: "#1F1F1F",
              },
            }
      }
    >
      <AccordionSummary sx={{ paddingX: 0 }}>
        <Table>
          <TableBody>
            <TableRow>
              <ProjectTableCell align='left' width='10%'>
                <IconButton
                  aria-label='expand row'
                  size='small'
                  onClick={() => {
                    handleChange(id);
                  }}
                >
                  {expanded === id ? (
                    <KeyboardArrowDown />
                  ) : (
                    <KeyboardArrowRight />
                  )}
                </IconButton>
              </ProjectTableCell>
              <ProjectTableCell align='left' width='auto'>
                <Typography variant='body1' textAlign='start'>
                  {row.name}
                </Typography>
              </ProjectTableCell>
              <ProjectTableCell align='right'>{row.created}</ProjectTableCell>
            </TableRow>
          </TableBody>
        </Table>
      </AccordionSummary>
      <AccordionDetails sx={{ paddingLeft: { md: 10, lg: 12 } }}>
        {/* TODO: RendersTable component */}
        <Table>
          <TableHead>
            <TableRow sx={{ fontStyle: "italic" }}>
              <ProjectTableCell width='1px' padding='none'></ProjectTableCell>
              <ProjectTableCell align='left'>Renders</ProjectTableCell>
              <ProjectTableCell align='left'>Status</ProjectTableCell>
              <ProjectTableCell align='right'>Render time</ProjectTableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {row.renders.map((render) => (
              <TableRow>
                <ProjectTableCell
                  width='1px'
                  padding='none'
                  sx={{ backgroundColor: "#F5F0F6" }}
                ></ProjectTableCell>
                <ProjectTableCell align='left'>
                  <Box display={{ xs: "none", sm: "block" }}>
                    <Typography
                      noWrap
                      width={{ xs: 200, sm: 200, md: 200, lg: 200, xl: 350 }}
                    >
                      {render.renderName}
                    </Typography>
                  </Box>
                  <Box display={{ xs: "block", sm: "none" }}>
                    <Typography
                      width={{ xs: 200, sm: 200, md: 200, lg: 200, xl: 350 }}
                    >
                      {render.renderName}
                    </Typography>
                  </Box>
                </ProjectTableCell>
                <ProjectTableCell align='left' width='100%'>
                  <Box display={{ xs: "none", md: "block" }}>
                    <LinearProgressWithLabel value={progress} />
                  </Box>
                  <Box display={{ xs: "block", md: "none" }}>
                    <Typography noWrap>{progress}%</Typography>
                  </Box>
                </ProjectTableCell>
                <ProjectTableCell align='right'>
                  <Typography width={{ xs: 50, sm: 150 }}>
                    {render.renderTime}
                  </Typography>
                </ProjectTableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </AccordionDetails>
    </Accordion>
  );
}
