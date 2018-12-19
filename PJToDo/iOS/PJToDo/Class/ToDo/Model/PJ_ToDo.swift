//
//  PJ_ToDo.swift
//  PJ_ToDo
//
//  Created by Zoey Weng on 2018/11/7.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

public enum PJToDoState: Int {
    case determined
    case inProgress
    case completed
    case overdue
}

public class PJ_ToDo: NSObject {
    private(set) var iToDoInsert: OpaquePointer?
    
    private(set) var iToDoQuery: OpaquePointer?
    
    private(set) var iToDoType: OpaquePointer?
    
    private(set) var iToDoTag: OpaquePointer?
    
    public var toDoType: PJToDoType?
    
    public var toDoTag: PJToDoTag?
    
    private var mode: PJToDoMode = .model
    
    public var toDoId: Int32 {
        get {
            return getToDoQueryId(self.iToDoQuery)
        }
        
        set {
            setToDoQueryId(self.iToDoQuery, newValue)
        }
    }
    
    public var title: String {
        get {
            if self.mode == .insert {
                return String.create(cString: getToDoInsertTitle(self.iToDoInsert))
            } else {
                return String.create(cString: getToDoQueryTitle(self.iToDoQuery))
            }
        }
        
        set {
            if self.mode == .insert {
                setToDoInsertTitle(self.iToDoInsert, newValue)
            } else {
                setToDoQueryTitle(self.iToDoQuery, newValue)
            }
        }
    }
    
    public var content: String {
        get {
            if self.mode == .insert {
                return String.create(cString: getToDoInsertContent(self.iToDoInsert))
            } else {
                return String.create(cString: getToDoQueryContent(self.iToDoQuery))
            }
        }
        
        set {
            if self.mode == .insert {
                setToDoInsertContent(self.iToDoInsert, newValue)
            } else {
                setToDoQueryContent(self.iToDoQuery, newValue)
            }
        }
    }
    
    public var dueTime: String {
        get {
            if self.mode == .insert {
                return String.create(cString: getToDoInsertDueTime(self.iToDoInsert))
            } else {
                return String.create(cString: getToDoQueryDueTime(self.iToDoQuery))
            }
        }
        
        set {
            if self.mode == .insert {
                setToDoInsertDueTime(self.iToDoInsert, newValue)
            } else {
                setToDoQueryDueTime(self.iToDoQuery, newValue)
            }
        }
    }
    
    public var remindTime: String {
        get {
            if self.mode == .insert {
                return String.create(cString: getToDoInsertRemindTime(self.iToDoInsert))
            } else {
                return String.create(cString: getToDoQueryRemindTime(self.iToDoQuery))
            }
        }
        
        set {
            if self.mode == .insert {
                setToDoInsertRemindTime(self.iToDoInsert, newValue)
            } else {
                setToDoQueryRemindTime(self.iToDoQuery, newValue)
            }
        }
    }
    
    public var createTime: String {
        get {
            if self.mode == .insert {
                return String.create(cString: getToDoInsertCreateTime(self.iToDoInsert))
            } else {
                return String.create(cString: getToDoQueryCreateTime(self.iToDoQuery))
            }
        }
        
        set {
            if self.mode == .insert {
                setToDoInsertCreateTime(self.iToDoInsert, newValue)
            } else {
                setToDoQueryCreateTime(self.iToDoQuery, newValue)
            }
        }
    }
    
    public var updateTime: String {
        get {
            if self.mode == .insert {
                return String.create(cString: getToDoInsertUpdateTime(self.iToDoInsert))
            } else {
                return String.create(cString: getToDoQueryUpdateTime(self.iToDoQuery))
            }
        }
        
        set {
            if self.mode == .insert {
                setToDoInsertUpdateTime(self.iToDoInsert, newValue)
            } else {
                setToDoQueryUpdateTime(self.iToDoQuery, newValue)
            }
        }
    }
    
    public var priority: Int32 {
        get {
            if self.mode == .insert {
                return getToDoInsert_ToDoPriority(self.iToDoInsert)
            } else {
                return getToDoQuery_ToDoPriority(self.iToDoQuery)
            }
        }
        
        set {
            if self.mode == .insert {
                setToDoInsert_ToDoPriority(self.iToDoInsert, newValue)
            } else {
                setToDoQuery_ToDoPriority(self.iToDoQuery, newValue)
            }
        }
    }
    
    public var toDoTypeId: Int32 {
        get {
            if self.mode == .insert {
                return getToDoInsert_ToDoTypeId(self.iToDoInsert)
            } else {
                return getToDoQuery_ToDoTypeId(self.iToDoQuery)
            }
        }
        
        set {
            if self.mode == .insert {
                setToDoInsert_ToDoTypeId(self.iToDoInsert, newValue)
            } else {
                setToDoQuery_ToDoTypeId(self.iToDoQuery, newValue)
            }
        }
    }
    
    public var toDoTagId: Int32 {
        get {
            if self.mode == .insert {
                return getToDoInsert_ToDoTagId(self.iToDoInsert)
            } else {
                return getToDoQuery_ToDoTagId(self.iToDoQuery)
            }
        }
        
        set {
            if self.mode == .insert {
                setToDoInsert_ToDoTagId(self.iToDoInsert, newValue)
            } else {
                setToDoQuery_ToDoTagId(self.iToDoQuery, newValue)
            }
        }
    }
    
    public var state: PJToDoState {
        get {
            if self.mode == .insert {
                if let value = PJToDoState(rawValue: Int(getToDoInsertState(self.iToDoInsert))) {
                    return value
                }
            } else {
                if let value = PJToDoState(rawValue: Int(getToDoQueryState(self.iToDoQuery))) {
                    return value
                }
            }
            return .determined
        }
        
        set {
            if self.mode == .insert {
                setToDoInsertState(self.iToDoInsert, Int32(newValue.rawValue))
            } else {
                setToDoQueryState(self.iToDoQuery, Int32(newValue.rawValue))
            }
        }
    }
    
    /*This constructor is used when inserting data.*/
    public init(mode: PJToDoMode = .model) {
        self.mode = mode
        if mode == .insert {
            self.iToDoInsert = createToDoInsert()
        } else {
            self.iToDoQuery = createToDoQuery()
        }
    }
    
    /*This constructor is used by ToDoTypeController when getting data from db.*/
    public init(iToDoQuery: OpaquePointer?, iToDoType: OpaquePointer?, iToDoTag: OpaquePointer?) {
        self.iToDoQuery = iToDoQuery
        self.iToDoType = iToDoType
        self.iToDoTag = iToDoTag
    }
}
