import React, { useEffect, useRef } from "react";
import { useAudioContext, useAudioData } from "../../hooks/audioContext";

export const Oscilloscope = (): JSX.Element => {
  const canvasEl = useRef<HTMLCanvasElement>(null);
  const drawDataRef = useRef<{
    samples: number[];
    state: ReturnType<typeof useAudioContext>["state"];
  }>({ samples: [], state: "play" });
  const { samples } = useAudioData();
  const { state } = useAudioContext();

  useEffect(() => {
    drawDataRef.current = {
      samples,
      state,
    };
  }, [samples, state]);

  useEffect(() => {
    const canvas = canvasEl.current;
    const c = canvasEl.current?.getContext("2d");
    if (canvas && c) {
      c.fillStyle = "#181818";
      c.fillRect(0, 0, canvas.width, canvas.height);
      c.strokeStyle = "#33ee55";
      c.beginPath();
      c.moveTo(0, canvas.height / 2);
      c.lineTo(canvas.width, canvas.height / 2);
      c.stroke();
      const draw = () => {
        const { state, samples } = drawDataRef.current;
        const segmentWidth = (canvas.width / samples.length) * 1.1; // Magic number to stop glitching at edge
        c.fillRect(0, 0, canvas.width, canvas.height);
        c.beginPath();
        c.moveTo(-100, canvas.height / 2);
        let prevNeg = false;
        let foundZero = false;
        let startIdx = 0;
        if (state == "play") {
          for (let i = 1; i < samples.length; i += 1) {
            // Make sure we start at/near a zero crossing.  This should keep the line relatively stable
            if (!foundZero) {
              if (prevNeg && samples[i] >= 0) {
                foundZero = true;
                startIdx = i;
              }
              prevNeg = samples[i] <= 0;
            } else {
              let x = (i - startIdx) * segmentWidth;
              let y = samples[i] * canvas.height * 0.4 + canvas.height / 2;
              c.lineTo(x, y);
            }
          }
        }
        c.lineTo(canvas.width + 100, canvas.height / 2);
        c.stroke();
        requestAnimationFrame(draw);
      };
      requestAnimationFrame(draw);
    }
  }, []);
  return <canvas ref={canvasEl} width="500px" height="200px"></canvas>;
};
