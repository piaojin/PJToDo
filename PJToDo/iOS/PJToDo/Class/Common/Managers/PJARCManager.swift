//
//  ARCManager.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/12/8.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import Foundation

open class PJARCManager: NSObject {
    public static func retain(object: AnyObject) -> UnsafeMutableRawPointer {
        let ownedPointer = UnsafeMutableRawPointer(Unmanaged.passRetained(object).toOpaque())
        return ownedPointer
    }
}
