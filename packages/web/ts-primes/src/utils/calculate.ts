// wasm must be loaded asynchronously
//  ... although, maybe try out top-level await...
const importWasm = async () => {
  const wasm = await import("rustlib-webpack-ts");
  console.log("Imported wasm module obj", wasm);
  return wasm;
};

/**
 * Nice return type to wrap the JSValue we got from rust.
 * wasm-bindgen can generate things like this, but they
 * come with some small strings attached. I'm adding the
 * type here because it's simpler
 */
export type PrimesResult = {
  primeCount: number;
  foundPrimes: number[];
};

export async function calculate(n: number): Promise<PrimesResult | undefined> {
  try {
    const wasm = await importWasm();
    console.log("calculate(): Imported wasm module obj", wasm);

    const result = wasm.fastFindPrimes(n);
    console.log("Calculated some primes", result);
    return {
      primeCount: result.primeCount,
      foundPrimes: result.foundPrimes
    };
  } catch (e) {
    console.error("failed to import wasm ", e);
    return undefined;
  }
}

/**
 *
 * @param n an integer greater than 0
 */
export async function calcAndLog(n: number) {
  try {
    const wasm = await importWasm();
    console.log("calcAndLog(): Imported wasm module obj", wasm);

    const result = wasm.fastFindPrimes(n);
    console.log("Calculated some primes", result);
  } catch (e) {
    console.error("failed to import wasm ", e);
  }
}
