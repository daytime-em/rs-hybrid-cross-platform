//
//  PrimeSieveModel.swift
//  ios
//
//  Created by Emily Dixon on 10/10/23.
//

import Foundation

/// Model class for the prime sieve. Can do one calculation at a time.
class PrimeSieveModel : ObservableObject {
    @Published var foundPrimes: FoundPrimes? = nil
    @Published var calculating: Bool = false
    
    /// Calculate primes up to the given integer value. If a calculation is in progress, this will be ignored
    func maybeCalculate(primesUpTo num: UInt32) {
        if calculateTask == nil {
            calculating = true
            calculateTask = Task { [self] in
                let foundPrimes = SimplePrimeFinder().findPrimesWithFastSieve(upTo: UInt64(num))
                print("Found \(foundPrimes.primeCount()) between 1 and \(foundPrimes.upToNumber())")
                Task {
                    await MainActor.run {
                        self.foundPrimes = foundPrimes
                        self.calculateTask = nil
                        self.calculating = false
                    }
                }
            }
        }
    }
    
    private var calculateTask: Task<(), Never>? = nil
}
