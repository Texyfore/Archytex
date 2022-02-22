import React from "react";

import Particles from "react-tsparticles";

export default function ParticleBubbles() {
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
            // value: 0.09,
            value: 0.09,
            random: false,
          },
          shape: {
            type: "circle",
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
