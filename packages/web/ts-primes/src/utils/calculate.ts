// import { default as rustlib } from "rustlib";
import { fastFindPrimes } from "rustlib";

/**
 * 
 * @param n an integer greater than 0
 */
export function calcAndLog(n: number) {
  // console.log("rustlib imported as object ", rustlib);
  console.log("rustlib imported as object ", fastFindPrimes);

  const result = fastFindPrimes(n);
  // const result = rustlib.fastFindPrimes(n);

  // TODO - To use wasm, we need to enable some stuff in webpack on our create-react-app

  console.log("Primes calculated: ", result);
}