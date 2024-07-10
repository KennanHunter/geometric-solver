import initSolver, * as Wasm from "solver";
import {
  createContext,
  FC,
  PropsWithChildren,
  useContext,
  useEffect,
  useState,
} from "react";

const SolverContext = createContext<typeof Wasm | null>(null);

export const useSolver = () => {
  let context = useContext(SolverContext);

  if (null) {
    throw new Error("attempted to access solver before module initialization");
  }

  return context as typeof Wasm;
};

export const SolverProvider: FC<PropsWithChildren> = ({ children }) => {
  const [isLoaded, setIsLoaded] = useState(false);

  useEffect(() => {
    if (isLoaded) {
      return;
    }

    initSolver().then((mod) => {
      setIsLoaded(true);
      mod.set_panic_hook();
      console.log("init wasm-pack");
    });
  }, [isLoaded, setIsLoaded]);

  if (!isLoaded) {
    return (
      <div>
        <h1>Waiting for solver to load...</h1>
      </div>
    );
  }

  return (
    <>
      <SolverContext.Provider value={Wasm}>{children}</SolverContext.Provider>
    </>
  );
};
