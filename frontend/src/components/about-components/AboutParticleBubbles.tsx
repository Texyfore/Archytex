import React from "react";

import Particles from "react-tsparticles";

import { ColorMode, useColorMode } from "../../services/colorMode";

import logoLight from "../../img/logoLight.svg";
import logoDark from "../../img/logoDark.svg";

export default function AboutParticleBubbles() {
  const [colorMode, _] = useColorMode();

  return (
    <Particles
      width='100%'
      height='500px'
      id='tsparticles'
      options={{
        fpsLimit: 60,
        fullScreen: {
          enable: false,
          zIndex: -10,
        },
        background: {
          color: "paper",
        },
        interactivity: {
          events: {
            onClick: {
              enable: false,
            },
            onHover: {
              enable: true,
              mode: "bubble",
            },
            resize: true,
          },
          modes: {
            bubble: {
              distance: 200,
              duration: 2,
              opacity: 0,
              size: 0.00001,
            },
          },
        },
        particles: {
          color: {
            value: "#39a0ed",
          },
          collisions: {
            enable: false,
          },
          move: {
            enable: true,
            speed: 0.5,
            direction: "top-right",
            random: true,
            straight: false,
            out_mode: "bounce",
            bounce: true,
          },
          number: {
            value: 100,
            density: {
              enable: true,
              area: 1578,
            },
          },
          opacity: {
            value: 0.5,
            random: false,
          },
          rotate: {
            random: true,
            animation: {
              enable: true,
              speed: 2,
            },
            direction: "random",
          },
          shape: {
            type: "image",
            options: {
              image: [
                {
                  height: 100,
                  width: 100,
                  src: colorMode === ColorMode.Light ? logoLight : logoDark,
                },
              ],
            },
          },
          size: {
            value: 28,
            random: true,
          },
        },
        detectRetina: true,
      }}
    />
  );
}
