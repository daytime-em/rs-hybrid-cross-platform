//
//  ContentView2.swift
//  ios
//
//  Created by Emily Dixon on 10/9/23.
//

import SwiftUI
import Charts

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
            if
                let foundPrimes = primeSieveModel.calculationResult?.foundPrimes,
                let approxDistribution = primeSieveModel.calculationResult?.approxDistribution
            {
                Text("Found \(foundPrimes.primeCount()) primes between 1 and \(foundPrimes.upToNumber())")
                    .font(
                        .system(
                            size: 12,
                            weight: .light
                        )
                    )
                PrimeDistributionChart(
                    approxDistribution: approxDistribution
                )
            }
        }
        Spacer()
    }
}

fileprivate struct PrimeDistributionChart: View {
    
    let approxDistribution: [Int]
    
    var body: some View {
        Chart {
            ForEach(
                mapToChatItems(from: approxDistribution)
            ) { chartItem in
                LineMark(
                    x: .value("", chartItem.xValue),
                    y: .value("", chartItem.yValue)
                )
            }
                .cornerRadius(20.0, style: .continuous)
        }
        .frame(height: 350)
    }
    
    private func mapToChatItems(from: [Int]) -> [ChartItem] {
        var outList: [ChartItem] = []
        for index in 0..<from.count {
            outList.append(
                ChartItem(
                    yValue: from[index],
                    xValue: index
                )
            )
        }
        print("mapped to chart items - \(outList.map { "x:\($0.xValue) y:\($0.yValue)" })")
        return outList
    }
    
    private struct ChartItem : Identifiable {
        let yValue: Int
        let xValue: Int
        var id: String { String(describing: yValue) }
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
