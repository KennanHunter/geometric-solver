import { FC, useEffect } from "react";
import { useSolver } from "./SolverContext";

export const Canvas: FC = () => {
  const {} = useSolver();

  useEffect(() => {});

  return (
    <>
      <canvas />
    </>
  );
};
