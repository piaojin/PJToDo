//
//  PJToDoType.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/10/24.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

public class PJToDoType {
    public var iToDoTypeInsert: OpaquePointer?
    
    public var iToDoType: OpaquePointer?
    
    public var typeName: String {
        get {
            return String(cString: getToDoTypeTypeName(self.iToDoType))
        }
        
        set {
            setToDoTypeTypeName(self.iToDoType, newValue)
        }
    }
    
    public var typeId: Int32 {
        get {
            return getToDoTypeTypeId(self.iToDoType)
        }
        
        set {
            setToDoTypeTypeId(self.iToDoType, newValue)
        }
    }
    
    public init(typeName: String) {
        self.iToDoTypeInsert = createToDoTypeInsert(typeName)
        self.iToDoType = createToDoType(typeName)
    }
    
    public init(iToDoType: OpaquePointer?) {
        self.iToDoType = iToDoType;
    }
}
