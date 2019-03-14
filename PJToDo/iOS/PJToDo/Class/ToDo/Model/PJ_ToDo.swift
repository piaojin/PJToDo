//
//  PJ_ToDo.swift
//  PJ_ToDo
//
//  Created by Zoey Weng on 2018/11/7.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

public enum PJToDoState: Int {
    case inProgress
    case unDetermined
    case completed
    case overdue
}

public class PJ_ToDo: NSObject {
    private(set) var iToDoInsert: OpaquePointer?
    
    private(set) var iToDoQuery: OpaquePointer?
    
    public var toDoType: PJToDoType = PJToDoType(typeId: -1, typeName: "")
    
    public var toDoTag: PJToDoTag = PJToDoTag(tagId: -1, tagName: "")
    
    private var mode: PJToDoMode = .model
    
    public var toDoId: Int32 {
        get {
            return pj_get_todo_query_id(self.iToDoQuery)
        }
        
        set {
            pj_set_todo_query_id(self.iToDoQuery, newValue)
        }
    }
    
    public var title: String {
        get {
            if self.mode == .insert {
                return String.create(cString: pj_get_todo_insert_title(self.iToDoInsert))
            } else {
                return String.create(cString: pj_get_todo_query_title(self.iToDoQuery))
            }
        }
        
        set {
            if self.mode == .insert {
                pj_set_todo_insert_title(self.iToDoInsert, newValue)
            } else {
                pj_set_todo_query_title(self.iToDoQuery, newValue)
            }
        }
    }
    
    public var content: String {
        get {
            if self.mode == .insert {
                return String.create(cString: pj_get_todo_insert_content(self.iToDoInsert))
            } else {
                return String.create(cString: pj_get_todo_query_content(self.iToDoQuery))
            }
        }
        
        set {
            if self.mode == .insert {
                pj_set_todo_insert_content(self.iToDoInsert, newValue)
            } else {
                pj_set_todo_query_content(self.iToDoQuery, newValue)
            }
        }
    }
    
    public var dueTime: String {
        get {
            if self.mode == .insert {
                return String.create(cString: pj_get_todo_insert_duetime(self.iToDoInsert))
            } else {
                return String.create(cString: pj_get_todo_query_duetime(self.iToDoQuery))
            }
        }
        
        set {
            if self.mode == .insert {
                pj_set_todo_insert_duetime(self.iToDoInsert, newValue)
            } else {
                pj_set_todo_query_duetime(self.iToDoQuery, newValue)
            }
        }
    }
    
    public var remindTime: String {
        get {
            if self.mode == .insert {
                return String.create(cString: pj_get_todo_insert_remind_time(self.iToDoInsert))
            } else {
                return String.create(cString: pj_get_todo_query_remind_time(self.iToDoQuery))
            }
        }
        
        set {
            if self.mode == .insert {
                pj_set_todo_insert_remind_time(self.iToDoInsert, newValue)
            } else {
                pj_set_todo_query_remind_time(self.iToDoQuery, newValue)
            }
        }
    }
    
    public var createTime: String {
        get {
            if self.mode == .insert {
                return String.create(cString: pj_get_todo_insert_create_time(self.iToDoInsert))
            } else {
                return String.create(cString: pj_get_todo_query_create_time(self.iToDoQuery))
            }
        }
        
        set {
            if self.mode == .insert {
                pj_set_todo_insert_create_time(self.iToDoInsert, newValue)
            } else {
                pj_set_todo_query_create_time(self.iToDoQuery, newValue)
            }
        }
    }
    
    public var updateTime: String {
        get {
            if self.mode == .insert {
                return String.create(cString: pj_get_todo_insert_update_time(self.iToDoInsert))
            } else {
                return String.create(cString: pj_get_todo_query_update_time(self.iToDoQuery))
            }
        }
        
        set {
            if self.mode == .insert {
                pj_set_todo_insert_update_time(self.iToDoInsert, newValue)
            } else {
                pj_set_todo_query_update_time(self.iToDoQuery, newValue)
            }
        }
    }
    
    public var priority: Int32 {
        get {
            if self.mode == .insert {
                return pj_get_todo_insert_todo_priority(self.iToDoInsert)
            } else {
                return pj_get_todo_query_todo_priority(self.iToDoQuery)
            }
        }
        
        set {
            if self.mode == .insert {
                pj_set_todo_insert_todo_priority(self.iToDoInsert, newValue)
            } else {
                pj_set_todo_query_todo_priority(self.iToDoQuery, newValue)
            }
        }
    }
    
    public var toDoTypeId: Int32 {
        get {
            if self.mode == .insert {
                return pj_get_todo_insert_todo_type_id(self.iToDoInsert)
            } else {
                return pj_get_todo_query_todo_type_id(self.iToDoQuery)
            }
        }
        
        set {
            if self.mode == .insert {
                pj_set_todo_insert_todo_type_id(self.iToDoInsert, newValue)
            } else {
                pj_set_todo_query_todo_type_id(self.iToDoQuery, newValue)
            }
        }
    }
    
    public var toDoTagId: Int32 {
        get {
            if self.mode == .insert {
                return pj_get_todo_insert_todo_tag_id(self.iToDoInsert)
            } else {
                return pj_get_todo_query_todo_tag_id(self.iToDoQuery)
            }
        }
        
        set {
            if self.mode == .insert {
                pj_set_todo_insert_todo_tag_id(self.iToDoInsert, newValue)
            } else {
                pj_set_todo_query_todo_tag_id(self.iToDoQuery, newValue)
            }
        }
    }
    
    public var state: PJToDoState {
        get {
            if self.mode == .insert {
                if let value = PJToDoState(rawValue: Int(pj_get_todo_insert_state(self.iToDoInsert))) {
                    return value
                }
            } else {
                if let value = PJToDoState(rawValue: Int(pj_get_todo_query_state(self.iToDoQuery))) {
                    return value
                }
            }
            return .unDetermined
        }
        
        set {
            if self.mode == .insert {
                pj_set_todo_insert_state(self.iToDoInsert, Int32(newValue.rawValue))
            } else {
                pj_set_todo_query_state(self.iToDoQuery, Int32(newValue.rawValue))
            }
        }
    }
    
    /*This constructor is used when inserting data.*/
    public init(mode: PJToDoMode = .model) {
        self.mode = mode
        if mode == .insert {
            self.iToDoInsert = pj_create_ToDoInsert()
        } else {
            self.iToDoQuery = pj_create_ToDoQuery()
        }
    }
    
    /*This constructor is used by ToDoTypeController when getting data from db.*/
    public init(iToDoQuery: OpaquePointer?, iToDoType: OpaquePointer?, iToDoTag: OpaquePointer?) {
        self.iToDoQuery = iToDoQuery
        self.toDoType = PJToDoType(iToDoType: iToDoType)
        self.toDoTag = PJToDoTag(iToDoTag: iToDoTag)
    }
}
