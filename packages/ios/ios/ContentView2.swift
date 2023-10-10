//
//  ContentView2.swift
//  ios
//
//  Created by Emily Dixon on 10/9/23.
//

import SwiftUI

struct ContentView2: View {
    var body: some View {
        Spacer()
        NumberInput()
            .padding()
        Spacer()
    }
}

fileprivate struct NumberInput: View {
    
    @State private var numberInput: String = ""
    
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
        NumberInput()
            .padding()
        Spacer()
    }
}
