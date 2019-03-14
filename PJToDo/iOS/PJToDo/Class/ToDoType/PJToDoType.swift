//
//  PJToDoType.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/10/24.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

public class PJToDoType: NSObject {
    private(set) var iToDoTypeInsert: OpaquePointer?
    
    private(set) var iToDoType: OpaquePointer?
    
    private var mode: PJToDoMode = .model
    
    public var typeName: String {
        get {
            if self.mode == .insert {
                return String.create(cString: pj_get_todo_type_insert_name(self.iToDoTypeInsert))
            } else {
                return String.create(cString: pj_get_todo_type_name(self.iToDoType))
            }
        }
        
        set {
            if self.mode == .insert {
                pj_set_todo_type_insert_name(self.iToDoTypeInsert, newValue)
            } else {
                pj_set_todo_type_name(self.iToDoType, newValue)
            }
        }
    }
    
    public var typeId: Int32 {
        get {
            return pj_get_todo_type_id(self.iToDoType)
        }
        
        set {
            pj_set_todo_type_id(self.iToDoType, newValue)
        }
    }
    
    /*This constructor is used when inserting data.*/
    public init(typeName: String) {
        super.init()
        self.iToDoTypeInsert = pj_create_ToDoTypeInsert(typeName)
        self.mode = .insert
    }
    
    /*This constructor is used by ToDoTypeController when getting data from db.*/
    public init(iToDoType: OpaquePointer?) {
        super.init()
        self.iToDoType = iToDoType;
    }
    
    public init(typeId: Int32, typeName: String) {
        super.init()
        self.iToDoType = pj_create_ToDoType(typeName)
        self.typeId = typeId;
    }
}
