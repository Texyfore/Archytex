import React, { useState } from "react";
import { Box, Button, IconButton, List, Tooltip } from "@mui/material";
import { styled } from "@mui/material/styles";
import houseImage4 from "../../img/4.jpg";
import houseImage5 from "../../img/5.jpg";
import houseImage6 from "../../img/6.jpg";
import houseImage7 from "../../img/7.jpg";
import houseImage8 from "../../img/8.jpg";
import houseImage9 from "../../img/9.jpg";
import houseImage10 from "../../img/10.jpg";
import houseImage11 from "../../img/11.jpg";
import houseImage12 from "../../img/12.jpg";
import RenderRow from "./RenderRow";
import { LibraryAdd } from "@mui/icons-material";
import SearchBar from "../SearchBar";
import { useTheme } from "@mui/material/styles";

const projects = [
  {
    id: Date.now().toString(),
    created: new Date().toLocaleDateString("en-US"),
    name: "Test Project",
    renders: [
      {
        id: Date.now().toString(),
        renderName:
          "Project1-project-render-1-and it's very long so it can be abbreviated",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1 h 40 min 23 sec",
        img: houseImage4,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project1-project-render-2",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage5,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project1-project-render-3",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage6,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project1-project-render-4",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage7,
      },
    ],
  },
  {
    id: Date.now().toString(),
    created: new Date().toLocaleDateString("en-US"),
    name: "Test Project 2",
    renders: [
      {
        id: Date.now().toString(),
        renderName:
          "Project2-project-render-1-and it's very long so it can be abbreviated",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1 h 40 min 23 sec",
        img: houseImage8,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-2",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage9,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-3",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage10,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-4",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage11,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-5",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage12,
      },
    ],
  },
  {
    id: Date.now().toString(),
    created: new Date().toLocaleDateString("en-US"),
    name: "Test Project 2",
    renders: [
      {
        id: Date.now().toString(),
        renderName:
          "Project2-project-render-1-and it's very long so it can be abbreviated",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1 h 40 min 23 sec",
        img: houseImage8,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-2",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage9,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-3",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage10,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-4",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage11,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-5",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage12,
      },
    ],
  },
  {
    id: Date.now().toString(),
    created: new Date().toLocaleDateString("en-US"),
    name: "Test Project 2",
    renders: [
      {
        id: Date.now().toString(),
        renderName:
          "Project2-project-render-1-and it's very long so it can be abbreviated",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1 h 40 min 23 sec",
        img: houseImage8,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-2",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage9,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-3",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage10,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-4",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage11,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-5",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage12,
      },
    ],
  },
  {
    id: Date.now().toString(),
    created: new Date().toLocaleDateString("en-US"),
    name: "Test Project 2",
    renders: [
      {
        id: Date.now().toString(),
        renderName:
          "Project2-project-render-1-and it's very long so it can be abbreviated",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1 h 40 min 23 sec",
        img: houseImage8,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-2",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage9,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-3",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage10,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-4",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage11,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-5",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage12,
      },
    ],
  },
  {
    id: Date.now().toString(),
    created: new Date().toLocaleDateString("en-US"),
    name: "Test Project 2",
    renders: [
      {
        id: Date.now().toString(),
        renderName:
          "Project2-project-render-1-and it's very long so it can be abbreviated",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1 h 40 min 23 sec",
        img: houseImage8,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-2",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage9,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-3",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage10,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-4",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage11,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-5",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage12,
      },
    ],
  },
  {
    id: Date.now().toString(),
    created: new Date().toLocaleDateString("en-US"),
    name: "Test Project 2",
    renders: [
      {
        id: Date.now().toString(),
        renderName:
          "Project2-project-render-1-and it's very long so it can be abbreviated",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1 h 40 min 23 sec",
        img: houseImage8,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-2",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage9,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-3",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage10,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-4",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage11,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-5",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage12,
      },
    ],
  },
  {
    id: Date.now().toString(),
    created: new Date().toLocaleDateString("en-US"),
    name: "Test Project 2",
    renders: [
      {
        id: Date.now().toString(),
        renderName:
          "Project2-project-render-1-and it's very long so it can be abbreviated",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1 h 40 min 23 sec",
        img: houseImage8,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-2",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage9,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-3",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage10,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-4",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage11,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-5",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage12,
      },
    ],
  },
  {
    id: Date.now().toString(),
    created: new Date().toLocaleDateString("en-US"),
    name: "Test Project 2",
    renders: [
      {
        id: Date.now().toString(),
        renderName:
          "Project2-project-render-1-and it's very long so it can be abbreviated",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1 h 40 min 23 sec",
        img: houseImage8,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-2",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage9,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-3",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage10,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-4",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage11,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-5",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage12,
      },
    ],
  },
  {
    id: Date.now().toString(),
    created: new Date().toLocaleDateString("en-US"),
    name: "Test Project 2",
    renders: [
      {
        id: Date.now().toString(),
        renderName:
          "Project2-project-render-1-and it's very long so it can be abbreviated",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1 h 40 min 23 sec",
        img: houseImage8,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-2",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage9,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-3",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage10,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-4",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage11,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-5",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage12,
      },
    ],
  },
  {
    id: Date.now().toString(),
    created: new Date().toLocaleDateString("en-US"),
    name: "Test Project 2",
    renders: [
      {
        id: Date.now().toString(),
        renderName:
          "Project2-project-render-1-and it's very long so it can be abbreviated",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1 h 40 min 23 sec",
        img: houseImage8,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-2",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage9,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-3",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage10,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-4",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage11,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-5",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage12,
      },
    ],
  },
  {
    id: Date.now().toString(),
    created: new Date().toLocaleDateString("en-US"),
    name: "Test Project 2",
    renders: [
      {
        id: Date.now().toString(),
        renderName:
          "Project2-project-render-1-and it's very long so it can be abbreviated",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1 h 40 min 23 sec",
        img: houseImage8,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-2",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage9,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-3",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage10,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-4",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage11,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-5",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage12,
      },
    ],
  },
  {
    id: Date.now().toString(),
    created: new Date().toLocaleDateString("en-US"),
    name: "Test Project 2",
    renders: [
      {
        id: Date.now().toString(),
        renderName:
          "Project2-project-render-1-and it's very long so it can be abbreviated",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1 h 40 min 23 sec",
        img: houseImage8,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-2",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage9,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-3",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage10,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-4",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage11,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-5",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage12,
      },
    ],
  },
  {
    id: Date.now().toString(),
    created: new Date().toLocaleDateString("en-US"),
    name: "Test Project 2",
    renders: [
      {
        id: Date.now().toString(),
        renderName:
          "Project2-project-render-1-and it's very long so it can be abbreviated",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1 h 40 min 23 sec",
        img: houseImage8,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-2",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage9,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-3",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage10,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-4",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage11,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-5",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage12,
      },
    ],
  },
  {
    id: Date.now().toString(),
    created: new Date().toLocaleDateString("en-US"),
    name: "Test Project 2",
    renders: [
      {
        id: Date.now().toString(),
        renderName:
          "Project2-project-render-1-and it's very long so it can be abbreviated",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1 h 40 min 23 sec",
        img: houseImage8,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-2",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage9,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-3",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage10,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-4",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage11,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-5",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage12,
      },
    ],
  },
  {
    id: Date.now().toString(),
    created: new Date().toLocaleDateString("en-US"),
    name: "Test Project 2",
    renders: [
      {
        id: Date.now().toString(),
        renderName:
          "Project2-project-render-1-and it's very long so it can be abbreviated",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1 h 40 min 23 sec",
        img: houseImage8,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-2",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage9,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-3",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage10,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-4",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage11,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-5",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage12,
      },
    ],
  },
  {
    id: Date.now().toString(),
    created: new Date().toLocaleDateString("en-US"),
    name: "Test Project 2",
    renders: [
      {
        id: Date.now().toString(),
        renderName:
          "Project2-project-render-1-and it's very long so it can be abbreviated",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1 h 40 min 23 sec",
        img: houseImage8,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-2",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage9,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-3",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage10,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-4",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage11,
      },
      {
        id: (Date.now() + 1).toString(),
        renderName: "Project2-project-render-5",
        status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
        renderTime: "1000h 35 min 21 sec",
        img: houseImage12,
      },
    ],
  },
];

const headerHeight = 50;
const projectMenuHeight = 60;
const RendersTableContainer = styled(List)(({ theme }) => ({
  border: "none",
  overflowY: "scroll",
  height: `calc(100vh - 56px - ${headerHeight + projectMenuHeight}px)`,
  [`${theme.breakpoints.up("xs")} and (orientation: landscape)`]: {
    height: `calc(100vh - 48px - ${headerHeight + projectMenuHeight}px)`,
  },
  [theme.breakpoints.up("sm")]: {
    height: `calc(100vh - 64px - ${headerHeight + projectMenuHeight}px)`,
  },
  [theme.breakpoints.up("lg")]: {
    height: `calc(100% - ${headerHeight + projectMenuHeight}px)`,
  },
}));

export default function RenderBrowser() {
  const [modalOpen, setModalOpen] = useState(false);
  const handleModalOpen = () => setModalOpen(true);
  return (
    <React.Fragment>
      <Box
        height={projectMenuHeight}
        display='flex'
        justifyContent='space-between'
        paddingX={{ xs: 2, sm: 4 }}
        borderBottom={`1px solid ${
          useTheme().palette.mode === "dark" ? "#1F1F1F" : "#EBE7EC"
        }`}
      >
        <Box display={{ xs: "none", md: "block" }}>
          <Button
            size='large'
            color='inherit'
            startIcon={<LibraryAdd />}
            onClick={handleModalOpen}
          >
            New project
          </Button>
        </Box>
        <Box display={{ xs: "block", md: "none" }}>
          <Tooltip title='Create new project'>
            <IconButton size='large' onClick={handleModalOpen}>
              <LibraryAdd />
            </IconButton>
          </Tooltip>
        </Box>
        <Box>
          <SearchBar />
        </Box>
      </Box>
      <RendersTableContainer>
        {projects.map((project) => (
          <RenderRow project={project} />
        ))}
      </RendersTableContainer>
    </React.Fragment>
  );
}
