import "App.css";
import { Canvas } from "./Canvas";
import { SolverProvider } from "./SolverContext";

export const App = () => {
  return (
    <SolverProvider>
      <Canvas />
    </SolverProvider>
  );
};
