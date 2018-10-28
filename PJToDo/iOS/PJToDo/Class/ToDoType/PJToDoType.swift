//
//  PJToDoType.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/10/24.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

class PJToDoType {
    public var iToDoTypeInsert: OpaquePointer?
    
    public var iToDoType: OpaquePointer?
    
    public var typeName: String = ""
    
    public var typeId: Int = -1
    
    init(typeName: String) {
        self.iToDoTypeInsert = createToDoTypeInsert(typeName)
        self.iToDoType = createToDoType(typeName)
    }
}
