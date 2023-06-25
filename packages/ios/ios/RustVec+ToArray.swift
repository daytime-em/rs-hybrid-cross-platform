//
//  RustVec+Array.swift
//  ios
//
//  Created by Emily Dixon on 10/10/23.
//

import Foundation

extension RustVec<UInt64> {
    func copyToSwiftArray() -> [UInt64] {
        var outputArr: [UInt64] = []
        for t in self {
            outputArr.append(t)
        }
        return outputArr
    }
}
