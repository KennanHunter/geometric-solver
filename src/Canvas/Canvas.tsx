import { createRef, FC, useEffect } from "react";
import { useSolver } from "../SolverContext";

export const Canvas: FC = () => {
  const {} = useSolver();

  const canvasRef = createRef<HTMLCanvasElement>();

  useEffect(() => {
    if (!canvasRef.current) return;

    const ctx = canvasRef.current.getContext("2d");

    if (!ctx) return;

    console.log("context created");

    ctx.fillStyle = "red";
    ctx.beginPath();
    ctx.ellipse(0, 0, 20, 20, 0, 0, 2 * Math.PI);
    ctx.fill();
  }, [canvasRef]);

  return (
    <>
      <canvas ref={canvasRef} />
    </>
  );
};
