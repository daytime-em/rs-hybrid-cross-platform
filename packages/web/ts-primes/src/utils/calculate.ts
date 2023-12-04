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
  approxDensities: number[];
};


/**
 * Group a set of found primes into a specified number of regions, returning the size of
 * each region
 */
function groupPrimes(params: {regions: number, upTo: number, primes: number[]}): number[] {
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

  let regionList: number[] = [];
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

export async function calculate(n: number): Promise<PrimesResult | undefined> {
  try {
    const wasm = await importWasm();
    console.log("calculate(): Imported wasm module obj", wasm);

    const result = wasm.fastFindPrimes(n);
    console.log("Calculated some primes", result);
    return {
      primeCount: result.primeCount,
      foundPrimes: result.foundPrimes,
      approxDensities: groupPrimes({
        regions: 50,
        upTo: n,
        primes: result.foundPrimes,
      }),
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
