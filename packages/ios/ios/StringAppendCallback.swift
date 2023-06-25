//
//  StringAppendCallback.swift
//  ios
//
//  Created by Emily Dixon on 9/5/23.
//

import Foundation

// NOTES:
// The Swift Bridge requires this class to be public because its bridged types are public
// The Rust side matches its param names with the labels of swift funtions, not the names

public class StringAppendCallback {
    let action: (RustString) -> String
    
    func invoke(from x: RustString) -> String {
        return action(x)
    }
    
    init(action: @escaping (RustString) -> String) {
        self.action = action
    }
}
