//
//  ARCManager.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/12/8.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

open class ARCManager: NSObject {
    public static func retain(object: AnyObject) {
        _ = UnsafeMutableRawPointer(Unmanaged.passRetained(object).toOpaque())
    }
}
