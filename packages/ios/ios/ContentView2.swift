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
        NumberInput(numberInput: nil)
            .padding()
        Spacer()
    }
}

fileprivate struct NumberInput: View {
    
    @State private var numberInput: String
    
    private let inputFormatter: Formatter = {
        let formatter = NumberFormatter()
        formatter.usesGroupingSeparator = true
        formatter.groupingSeparator = ","
        
        return formatter
    }()
    
    var body: some View {
        HStack {
            // todo - format this
            TextField(
                value: $numberInput,
                formatter: inputFormatter,
                prompt: Text("Positive Integer")
            ) {
                Text("Number input")
            }
            Button(action: {
                // todo - calculate
            }, label: {
                Text("Go!")
            })
        }
        .padding()
        .background {
            RoundedRectangle(cornerRadius: 4.0)
                .strokeBorder(style: StrokeStyle(lineWidth: 1.0))
        }
    }
    
    init(numberInput: String?) {
        self.numberInput = numberInput ?? ""
    }
}

#Preview("Screen Content") {
    ContentView2()
}

#Preview("Number Input") {
    VStack(alignment: .leading) {
        Spacer()
        NumberInput(numberInput: "3000")
            .padding()
        Spacer()
    }
}
