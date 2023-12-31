//
//  iosApp.swift
//  ios
//
//  Created by Emily Dixon on 7/4/23.
//

import SwiftUI

@main
struct iosApp: App {
    
    @StateObject var primeSieveModel = PrimeSieveModel()
    
    var body: some Scene {
        WindowGroup {
            ContentView()
                .environmentObject(primeSieveModel)
        }
    }
}
