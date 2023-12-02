import React from "react";
import logo from "./logo.svg";
import "./App.css";
import { PrimeChart } from "./components/PrimeChart";

function App() {
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
