import React, { useState } from "react";
import "./App.css";
import { PrimeChart } from "./components/PrimeChart";
import { calcAndLog, PrimesResult, calculate } from "./utils/calculate";
import { stat } from "fs";

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
  let appStateHook = props.stateHook;
  let [input, setInput] = useState<string>(); 

  const beginCalculating = (n: number) => {
    calculate(n)
      .catch((e) => console.error("failed to calculate!", e))
      .then((result) => {
        console.log("App: calculation result", result);
        if (result) {
            let newState = Object.assign({}, appStateHook.state);
            newState.uiState = UiState.result;
            newState.primeData = result;
            appStateHook.setState(newState);
        }
      });
  };

  return (
  <div className="App-Input-Container">
    <p>Enter a number between 1 and like a few hundred million</p>
    <label>
      Number: <input 
        value={input}
        onChange={ev => setInput(ev.target.value)}
      />
    </label>
    <button
      onClick={() => {
        if (input) {
          let parsedInput = parseInt(input);
          if (parsedInput && !Number.isNaN(parsedInput)) {
            // Update the UI with the new state
            let newState = Object.assign({}, appStateHook.state);
            newState.uiState = UiState.loading;
            newState.inputNum = parsedInput;
            appStateHook.setState(newState);

            beginCalculating(parsedInput);
          }
        }
      }} 
    >
      Calculate 📊
    </button>
  </div>
  );
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

  // todo - don't need this
  // calculate(100_000_000)
    // .then((res) => console.log("calculated result", res));

  if (currentState.uiState === UiState.result) {
    const chartItems = currentState.primeData?.approxDensities as number[];
    return (
      <div>
        <NumberInput stateHook={stateHook}/> 
        <PrimeChart approxPrimes={chartItems} />
      </div>
    );
  } else if (currentState.uiState === UiState.loading) {
    return (
    <>
      <p>Calculating...</p>
    </>
    );
  } else /*if (currentState.uiState === UiState.initial)*/ {
    return (
      <>
        <NumberInput stateHook={stateHook}/> 
      </>
    );
  }
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
