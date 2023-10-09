//
//  ContentView.swift
//  ios
//
//  Created by Emily Dixon on 7/4/23.
//

import SwiftUI

struct ContentView: View {
    var body: some View {
        VStack {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundColor(.accentColor)
            Text(primes())
            Text(appendedText())
        }
        .padding()
    }
    
    func appendedText() -> String {
        let startingWith = "String with";
        let cb = StringAppendCallback { rsStr in
            return "\(rsStr.toString()) appended callback text"
        }
        return doACallback(startingWith: startingWith, cb).toString()
    }
    
    func primes() -> String {
        let finder = SimplePrimeFinder()
        let result = finder.findPrimesWithFastSieve(upTo: 2000)
        
//        for prime in result.getPrimes() {
//            print("prime \(prime)")
//        }
        
        return String(result.primeCount())
    }
    
    func something() {
        printViaPrivateDelegate()
        didIAppear()
        printViaRedeclareMethod()
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
