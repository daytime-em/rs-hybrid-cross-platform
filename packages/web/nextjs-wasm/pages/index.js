import Head from "next/head";
import styles from "../styles/Home.module.css";
import { default as rustlib } from "rustlib";
import { useState } from "react";
import Chart from "chart.js";

/**
 *
 * @param {number} upTo
 */
async function findPrimesAsync(upTo) {
  let primeResult = await rustlib.findPrimes(upTo);

  return primeResult;
}

function maybeCalculatePrimes(upTo, primeData, setPrimeData) {
  if (primeData == null) {
    findPrimesAsync(upTo).then((countData) => setPrimeData(countData));
  } else {
    return primeData;
  }
}

function ResultView(props) {
  console.log("ResultView: props are", props);
  console.log("ResultView: This is", this);

  const primeData = props.primeData;
  const upTo = props.upTo;

  if (primeData == null || primeData.primeCount == null) {
    return (<p className={styles.description}>Calculating...</p>);
  }
  return (
    <PrimeChart 
      upTo={upTo}
      foundPrimes={primeData.foundPrimes}
      // props={{
        // upTo: upTo,
        // foundPrimes: primeData.foundPrimes
      // }}
    />
    );
}

function primeListText(primeData) {
  if (primeData == null || primeData.foundPrimes == null) {
    return "";
  } else {
    let primes = primeData.foundPrimes;
    if (primes.length <= 50) {
      return primes.join(", ");
    } else {
      let first25 = primes.slice(0, 25).join(", ");
      let last25 = primes.slice(primes.length - 25, primes.length).join(", ");
      return `${first25} ... ${last25}`;
    }
  }
}

/**
 *
 * @param {number[]} approxDensities
 */
function chartData(approxDensities) {
  let data = {};

  data.width = 500;
  data.height = 750;

  data.dataset = approxDensities;
}

/**
 *
 * @param { regions: number, up_to: number, primes: number[]} params
 */
function groupPrimes(params) {
  const regions = params.regions;
  const upTo = params.upTo;
  const primes = params.primes;

  let realRegionCount = regions;
  let regionSize = 1;
  if (upTo <= regions) {
    // not enough primes to bother grouping
    regionSize = 1;
    realRegionCount = upTo;
  } else {
    regionSize = Math.ceil(upTo / regions);
    realRegionCount = regions;
  }

  let regionList = [];
  for (const prime of primes) {
    let proportion = prime / regionSize;
    let idx = Math.floor(proportion);

    if (regionList[idx]) {
      regionList[idx] = regionList[idx] + 1;
    } else {
      regionList[idx] = 1;
    }
  }

  return regionList;
}

/**
 * 
 * @param {upTo: number, foundPrimes: number[]} props 
 * @returns 
 */
function PrimeChart(props) {
  const upTo = props.upTo;
  const foundPrimes = props.foundPrimes;

  console.log("PrimeChart: Init with props: ", props);

  //const labels = Utils.months({ count: 7 });
  let groups = groupPrimes({
    regions: 50,
    upTo: upTo,
    primes: foundPrimes,
  });
  console.log("It is groups: ", groups);

  const data = {
    labels: [],
    datasets: [
      {
        label: "Approximate Primes",
        data: groups,
        fill: false,
        borderColor: "rgb(75, 192, 192)",
        tension: 0.1,
      },
    ],
  };
  const config = {
    type: "line",
    data: data,
  };
  return (
    // <Chart config/>
    <Chart
     type="line"
     data={data}
     />
  );
}

export default function Home() {
  console.log("Home: This is", this);

  let [primeData, setPrimeData] = useState(null);
  const upToValue = 3_000_000;

  maybeCalculatePrimes(upToValue, primeData, setPrimeData);

  return (
    <div className={styles.container}>
      <Head>
        <title>Create Next App</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main>
        <p className={styles.h1}>Prime Counter</p>

        <div>
          <ResultView 
            primeData={primeData}
            upTo={upToValue}
          />
          {/*ResultView(primeData, upToValue)*/}
        </div>

        <p className={styles.p}>{primeListText(primeData)}</p>

        {/*TODO: Remove this , show something fun instead ;) */}
        <p className={styles.description}>
          <button
            onClick={() => {
              rustlib
                .doACallback(
                  "Starting string",
                  this,
                  (str) => `${str} Plus some appended text`
                )
                .then((appended) => alert(appended));
            }}
          >
            Click to test the callback
          </button>
        </p>
      </main>

      <footer>
        <a
          href="https://vercel.com?utm_source=create-next-app&utm_medium=default-template&utm_campaign=create-next-app"
          target="_blank"
          rel="noopener noreferrer"
        >
          Powered by{" "}
          <img src="/vercel.svg" alt="Vercel" className={styles.logo} />
        </a>
      </footer>

      <style jsx>{`
        main {
          padding: 5rem 0;
          flex: 1;
          display: flex;
          flex-direction: column;
          justify-content: center;
          align-items: center;
        }
        footer {
          width: 100%;
          height: 100px;
          border-top: 1px solid #eaeaea;
          display: flex;
          justify-content: center;
          align-items: center;
        }
        footer img {
          margin-left: 0.5rem;
        }
        footer a {
          display: flex;
          justify-content: center;
          align-items: center;
          text-decoration: none;
          color: inherit;
        }
        code {
          background: #fafafa;
          border-radius: 5px;
          padding: 0.75rem;
          font-size: 1.1rem;
          font-family: Menlo, Monaco, Lucida Console, Liberation Mono,
            DejaVu Sans Mono, Bitstream Vera Sans Mono, Courier New, monospace;
        }
      `}</style>

      <style jsx global>{`
        html,
        body {
          padding: 0;
          margin: 0;
          font-family: -apple-system, BlinkMacSystemFont, Segoe UI, Roboto,
            Oxygen, Ubuntu, Cantarell, Fira Sans, Droid Sans, Helvetica Neue,
            sans-serif;
        }
        * {
          box-sizing: border-box;
        }
      `}</style>
    </div>
  );
}
