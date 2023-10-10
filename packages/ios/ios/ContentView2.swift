//
//  ContentView2.swift
//  ios
//
//  Created by Emily Dixon on 10/9/23.
//

import SwiftUI

struct ContentView2: View {
   
    @EnvironmentObject var primeSieveModel: PrimeSieveModel
    
    var body: some View {
        Spacer()
        if primeSieveModel.calculating {
            Text("Loading...")
        } else {
            NumberInput { upToNumber in
                primeSieveModel.calculating = true
                primeSieveModel.maybeCalculate(primesUpTo: upToNumber)
            }
            .padding()
            if let foundPrimes = primeSieveModel.foundPrimes {
                Text("Found \(foundPrimes.primeCount()) primes between 1 and \(foundPrimes.upToNumber())")
                    .font(.system(size: 12, weight: .light))
            }
        }
        Spacer()
    }
}

fileprivate struct NumberInput: View {
    
    @State private var numberInput: String = ""
    let handleCalculate: (UInt32) -> Void
    
    var body: some View {
        HStack {
            TextField(
                "Positive integer",
                text: $numberInput
            )
            .onSubmit {
                if let num = inputAsInteger() {
                    calculate(number: num)
                }
            }
            Button(action: {
                if let num = inputAsInteger() {
                    calculate(number: num)
                }
            }, label: {
                Text("Go!")
            })
            .disabled(inputAsInteger() == nil)
        }
        .padding()
        .background {
            RoundedRectangle(cornerRadius: 8.0)
                .strokeBorder(style: StrokeStyle(lineWidth: 1.0))
        }
    }
    
    private func calculate(number: UInt32) {
        self.handleCalculate(number)
    }
    
    private func inputAsInteger() -> UInt32? {
        print("NumberInput is: \(String(describing: numberInput))")
        return UInt32(numberInput)
    }
}

#Preview("Screen Content") {
    ContentView2()
}

#Preview("Number Input") {
    VStack(alignment: .leading) {
        Spacer()
        NumberInput(handleCalculate: {_ in })
            .padding()
        Spacer()
    }
}
