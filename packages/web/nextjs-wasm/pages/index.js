import Head from "next/head";
import styles from "../styles/Home.module.css";
import { default as rustlib } from "rustlib";
import { useState } from "react";

/**
 *
 * @param {number} upTo
 */
async function findPrimesAsync(upTo) {
  return await rustlib.findPrimes(upTo);
}

function maybeCalculatePrimes(upTo, primeData, setPrimeData) {
  if (primeData == null) {
    findPrimesAsync(upTo).then((countData) => setPrimeData(countData));
  } else {
    return primeData;
  }
}

function primeCountText(primeData, upTo) {
  if (primeData == null || primeData.primeCount == null) {
    return "Calculating";
  }
  return `There are ${primeData.primeCount} primes between 0 and ${upTo}`;
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

export default function Home() {
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

        <p className={styles.description}>
          {primeCountText(primeData, upToValue)}
        </p>

        <p className={styles.p}>
          {primeListText(primeData)}
        </p>

        {/*TODO: Remove this , show something fun instead*/}
        <p className={styles.description}>
          <button
            onClick={() => {
              rustlib.doACallback(
                "Starting string",
                this,
                str => `${str} Plus some appended text`
              ).then(appended => alert(appended));
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
