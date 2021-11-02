import { KeyboardArrowDown, KeyboardArrowRight } from "@mui/icons-material";
import { styled } from "@mui/material/styles";
import {
  Accordion,
  AccordionDetails,
  AccordionSummary,
  IconButton,
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

function createData(name: string, created: string) {
  return {
    name,
    created,
    renders: [
      {
        renderName: name + "-project-render-1",
        status: 100, //percentage
        renderTime: "1 h 40 min 23 sec",
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

  return (
    <Accordion
      disableGutters
      elevation={0}
      expanded={expanded === id}
      onChange={handleChange(id)}
      sx={
        expanded === id
          ? {
              backgroundColor: "#14151A",
              borderRadius: 4,
            }
          : {
              position: "static",
              borderRadius: 4,
              ".MuiAccordionSummary-root:hover": {
                backgroundColor: "#14151A",
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
                    console.log("beg");
                    handleChange(id);
                    console.log("end");
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
        <Table>
          <TableHead>
            <TableRow>
              <TableCell align='left'>Renders</TableCell>
              <TableCell align='left'>Status</TableCell>
              <TableCell align='right'>Render time</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {row.renders.map((render) => (
              <TableRow>
                <TableCell align='left'>{render.renderName}</TableCell>
                <TableCell align='left'>{render.status}</TableCell>
                <TableCell align='right'>{render.renderTime}</TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </AccordionDetails>
    </Accordion>
  );
}
