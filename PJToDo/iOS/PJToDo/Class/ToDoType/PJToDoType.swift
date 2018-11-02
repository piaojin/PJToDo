//
//  PJToDoType.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/10/24.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

private enum PJToDoTypeMode {
    case insert
    case model
}

public class PJToDoType {
    private(set) var iToDoTypeInsert: OpaquePointer?
    
    private(set) var iToDoType: OpaquePointer?
    
    private var mode: PJToDoTypeMode = .model
    
    public var typeName: String {
        get {
            return String(cString: getToDoTypeTypeName(self.iToDoType))
        }
        
        set {
            setToDoTypeTypeName(self.iToDoType, newValue)
            if self.mode == .insert {
                setToDoTypeInsertTypeName(self.iToDoTypeInsert, newValue)
            }
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
    
    /*This constructor is used when inserting data.*/
    public init(typeName: String) {
        self.iToDoTypeInsert = createToDoTypeInsert(typeName)
        self.mode = .insert
    }
    
    /*This constructor is used by ToDoTypeController when getting data from db.*/
    public init(iToDoType: OpaquePointer?) {
        self.iToDoType = iToDoType;
    }
    
    deinit {
        /*Rust will free the iToDoType.*/
        if self.mode == .insert {
            if let tempIToDoTypeInsert = self.iToDoTypeInsert {
                free_rust_ToDoTypeInsert(tempIToDoTypeInsert)
            }
        } else {
//            if let tempIToDoType = self.iToDoType {
//                free_rust_ToDoType(tempIToDoType)
//            }
        }
    }
}
