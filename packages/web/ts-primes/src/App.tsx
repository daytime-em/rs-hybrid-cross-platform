import React from "react";
import "./App.css";
import { PrimeChart } from "./components/PrimeChart";
import { calcAndLog } from "./utils/calculate";

function App() {

  //todo - remove
  calcAndLog(700);

  return (
    <div className="App">
      <header className="App-header">
        <p>Hello World</p>
      </header>
      <main>
        <div className="App-body">
          <PrimeChart approxPrimes={[1, 2, 6, 7, 8]} />
        </div>
      </main>
    </div>
  );
}

export default App;
