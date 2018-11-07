//
//  PJToDoType.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/10/24.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

public class PJToDoType {
    private(set) var iToDoTypeInsert: OpaquePointer?
    
    private(set) var iToDoType: OpaquePointer?
    
    private var mode: PJToDoMode = .model
    
    public var typeName: String {
        get {
            if self.mode == .insert {
                return String(cString: getToDoTypeInsertName(self.iToDoTypeInsert))
            } else {
                return String(cString: getToDoTypeName(self.iToDoType))
            }
        }
        
        set {
            if self.mode == .insert {
                setToDoTypeInsertName(self.iToDoTypeInsert, newValue)
            } else {
                setToDoTypeName(self.iToDoType, newValue)
            }
        }
    }
    
    public var typeId: Int32 {
        get {
            return getToDoTypeId(self.iToDoType)
        }
        
        set {
            setToDoTypeId(self.iToDoType, newValue)
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
    
    public init(typeId: Int32, typeName: String) {
        self.iToDoType = createToDoType(typeName)
        self.typeId = typeId;
    }
}
