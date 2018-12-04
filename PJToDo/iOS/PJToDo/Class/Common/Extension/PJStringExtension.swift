//
//  PJStringExtension.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/12/4.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import Foundation

public extension String {
    public static func create(cString: UnsafeMutablePointer<Int8>) -> String {
        //get string pointer from rust and use the pointer to create a Swift String and then free the rust string
        let str = String(cString: cString)
        free_rust_string(cString)
        return str
    }
}
