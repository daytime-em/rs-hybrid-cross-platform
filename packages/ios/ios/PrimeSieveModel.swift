//
//  PrimeSieveModel.swift
//  ios
//
//  Created by Emily Dixon on 10/10/23.
//

import Foundation

/// Model class for the prime sieve. Can do one calculation at a time.
class PrimeSieveModel : ObservableObject {
    @Published var calculationResult: CalculationResult? = nil
    @Published var calculating: Bool = false
    
    /// Calculate primes up to the given integer value. If a calculation is in progress, this will be ignored
    func maybeCalculate(primesUpTo num: UInt32) {
        if calculateTask == nil {
            calculating = true
            calculateTask = Task { [self] in
                let foundPrimes = SimplePrimeFinder().findPrimesWithFastSieve(upTo: UInt64(num))
                print("Found \(foundPrimes.primeCount()) between 1 and \(foundPrimes.upToNumber())")
                let calcRes = CalculationResult(
                    foundPrimes: foundPrimes,
                    approxDistribution: groupPrimes(
                        regions: 50,
                        originalUpTo: Int(num),
                        primes: foundPrimes.primesVec().copyToSwiftArray()
                    )
                )
                Task {
                    await MainActor.run {
                        self.calculationResult = calcRes
                        self.calculateTask = nil
                        self.calculating = false
                    }
                }
            }
        }
    }
    
    private func groupPrimes(regions: Int, originalUpTo: Int, primes: [UInt64]) -> [Int] {
        let regionSize = {
            if (originalUpTo <= regions) {
                return UInt64(1)
            } else {
                return UInt64(ceil(Double(originalUpTo) / Double(regions)))
            }
        }()
        let realRegionCount = {
            if (originalUpTo <= regions) {
                return originalUpTo
            } else {
                return regions
            }
        }()
        print("region size \(regionSize)")
        
        var regionList: [Int] = [Int](repeatElement(0, count: realRegionCount))
        for prime in primes {
            let proportion = prime / regionSize
            let idx = Int(proportion)
            
            regionList[idx] = regionList[idx] + 1
        }
        return regionList
    }
    
    private var calculateTask: Task<(), Never>? = nil
}

struct CalculationResult {
    let foundPrimes: FoundPrimes
    let approxDistribution: [Int]
}
