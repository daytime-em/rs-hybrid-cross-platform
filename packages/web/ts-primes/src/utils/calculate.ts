// import { default as rustlib } from "rustlib";
// import { fastFindPrimes } from "rustlib";
// import { fastFindPrimes } from "rustlib-webpack-ts";

console.warn("I got ran because I got imported (presumably)");

const importWasm = async () => {
  // wasm must be loaded asynchronously
  //  ... although, maybe try out syncWebAssembly or top-level await...
  const wasm = await import("rustlib-webpack-ts");
  console.log("Imported wasm module obj", wasm);
  return wasm;
};

/**
 * 
 * @param n an integer greater than 0
 */
export async function calcAndLog(n: number) {
  try {
    const wasm = await importWasm();
    console.log("calcAndLog(): Imported wasm module obj", wasm);

    const result = wasm.fastFindPrimes(n);
    console.log("Calculated some primes", result)
  } catch(e) {
    console.error("failed to import wasm ", e);
  }

  // console.log("rustlib imported as object ", rustlib);
  // console.log("rustlib imported as object ", fastFindPrimes);

  // const result = fastFindPrimes(n);
  // const result = rustlib.fastFindPrimes(n);

  // TODO - To use wasm, we need to enable some stuff in webpack on our create-react-app

  // console.log("Primes calculated: ", result);
}