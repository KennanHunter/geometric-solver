import { createRef, FC, useCallback, useEffect } from "react";
import { useSolver } from "../SolverContext";
import { useScreenSize } from "../util/useScreenSize";
import classes from "./Canvas.module.css";

const POINT_RADIUS = 5;
const POINT_COLOR = "red" as const;

type Coords = { x: number; y: number };

export const Canvas: FC = () => {
  const { get_sample_points } = useSolver();

  const canvasRef = createRef<HTMLCanvasElement>();

  const size = useScreenSize();

  const toSolverCoords = useCallback<(canvasCoords: Coords) => Coords>(
    (canvasCoords) => ({
      x: canvasCoords.x - size.width / 2,
      y: canvasCoords.y - size.height / 2,
    }),
    [size]
  );

  const toCanvasCoords = useCallback<(solverCoords: Coords) => Coords>(
    (solverCoords) => ({
      x: solverCoords.x - size.width / 2,
      y: solverCoords.y - size.height / 2,
    }),
    [size]
  );

  useEffect(() => {
    console.log("rerender");

    const [point] = get_sample_points();

    if (!canvasRef.current) return;

    const ctx = canvasRef.current.getContext("2d");

    if (!ctx) return;

    console.log("context created");

    ctx.fillStyle = POINT_COLOR;
    ctx.beginPath();
    ctx.ellipse(
      size.width / 2 + point.x,
      size.height / 2 + point.y,
      POINT_RADIUS,
      POINT_RADIUS,
      0,
      0,
      2 * Math.PI
    );
    ctx.fill();

    canvasRef.current.addEventListener("mousedown", (event) => {
      let solverCoords = toSolverCoords({
        x: event.x,
        y: event.y,
      });

      let clickDistance = Math.sqrt(
        (solverCoords.x - point.x) ** 2 + (solverCoords.y - point.y) ** 2
      );

      console.dir({ clickDistance });

      if (clickDistance < POINT_RADIUS * 1.2)
        console.log("Point intersected circle");

      console.log(`Mouse pressed at ${solverCoords.x} and ${solverCoords.y}`);
    });

    canvasRef.current.addEventListener("mouseup", (event) => {
      console.log(`Mouse up at ${event.x} and ${event.y}`);
    });

    canvasRef.current.addEventListener("mouseleave", (event) => {
      console.log(`Mouse left at ${event.x} and ${event.y}`);
    });
  }, [canvasRef, size]);

  return (
    <>
      <canvas
        className={classes.canvas}
        width={size.width}
        height={size.height}
        ref={canvasRef}
      />
    </>
  );
};
