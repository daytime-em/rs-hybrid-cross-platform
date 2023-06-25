// import("../pkg/index.js")
// .then((module) => {
// module.greet("Trying this now");
// })
// .catch(console.error);

function greetFromRust(str) {
  import("../pkg/index.js")
    .then((module) => {
      module.greet(str);
    })
    .catch(console.error);
}

/**
 *
 * @param {number} upTo
 * @returns {{ primeCount: {number}, foundPrimes: {number[]} }}
 */
async function findPrimes(upTo) {
  const module = await import("../pkg/index.js");

  // JS objects can be created from rust. The JS runtime owns them
  //  Doing this instead of exporting structs keeps you from having to
  //  free the objects that these functions return
  let ret = module.fastFindPrimes(upTo);
  return ret;
}

/**
 * Finds primes via a simple sieve.
 *
 * @param {number} upTo
 */
async function findPrimesInout(upTo) {
  const module = await import("../pkg/index.js");

  // in/out as a param
  let ret = {};
  module.simpleFindPrimesInout(upTo, ret);

  // Don't make callers external to this module have to deal with the inout thing
  return ret;
}

/**
 * 
 * @param {string} startingWith 
*  @param {any} context
 * @param {function callbackFn(string) -> string } 
 */
async function doACallback(startingWith, context, callbackFn) {
  const module = await import("../pkg/index.js");
  return module.stringCallback(
    startingWith,
    context,
    str => callbackFn(str)
  )
}

export default {
  doACallback,
  greetFromRust,
  findPrimes,
  findPrimesInout,
};
