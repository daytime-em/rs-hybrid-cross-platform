import React from "react";
import "./App.css";
import { PrimeChart } from "./components/PrimeChart";
import { calcAndLog, PrimesResult, calculate } from "./utils/calculate";

// ====== Hooks and stuff

enum UiState {
  initial, loading, result
}

type AppState = {
  uiState: UiState,
  inputNum: number | undefined,
  primeData: PrimesResult | undefined,
}

type AppStateHook = {
  state: AppState,
  setState: (data: AppState) => void,
};

function useAppState(): AppStateHook {
  const [current, setFn] = React.useState<AppState>({
    uiState: UiState.initial, inputNum: undefined, primeData: undefined
  });
  return {
    state: current,
    setState: setFn,
  };
}

// ====== Components

type ComponentProps = {
  stateHook: AppStateHook
};

function LoadingContent() {
  return (
    "Calculating... "
  );
}

function NumberInput(props: ComponentProps) {
  return (<></>);
}

function InstructionChart(props: ComponentProps) {
  return (<></>);
}

function ResultChart(props: ComponentProps) {
  return (<></>);
}

function MainContent() {
  const stateHook: AppStateHook = useAppState();
  const currentState = stateHook.state;

  if (currentState.uiState === UiState.result) {

  } else if (currentState.uiState === UiState.loading) {
    return (<>
      <p>Calculating...</p>
    </>);
  } else /*if (currentState.uiState === UiState.initial)*/ {

  }

  return (
    <></>
  );
}

function App() {

  //todo - remove
  calcAndLog(700);

  // todo - useAppState hook :)
  // let [ primeData, setPrimeData ] = React.useState(null);

  return (
    <div className="App">
      <header className="App-header">
        <p>sieve of erasthones</p>
      </header>
      <main>
        <div className="App-body">
          <MainContent />
          {/* <PrimeChart approxPrimes={[1, 2, 6, 7, 8]} /> */}
        </div>
      </main>
    </div>
  );
}

export default App;
