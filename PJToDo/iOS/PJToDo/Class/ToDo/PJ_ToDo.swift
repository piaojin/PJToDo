//
//  PJ_ToDo.swift
//  PJ_ToDo
//
//  Created by Zoey Weng on 2018/11/7.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

public enum PJToDoState: Int {
    case completed
    case cancel
    case reToDo
    case determined
    case inProgress
}

public class PJ_ToDo {
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
                return String(cString: getToDoInsertTitle(self.iToDoInsert))
            } else {
                return String(cString: getToDoQueryTitle(self.iToDoQuery))
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
                return String(cString: getToDoInsertContent(self.iToDoInsert))
            } else {
                return String(cString: getToDoQueryContent(self.iToDoQuery))
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
                return String(cString: getToDoInsertDueTime(self.iToDoInsert))
            } else {
                return String(cString: getToDoQueryDueTime(self.iToDoQuery))
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
                return String(cString: getToDoInsertRemindTime(self.iToDoInsert))
            } else {
                return String(cString: getToDoQueryRemindTime(self.iToDoQuery))
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
                return String(cString: getToDoInsertCreateTime(self.iToDoInsert))
            } else {
                return String(cString: getToDoQueryCreateTime(self.iToDoQuery))
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
                return String(cString: getToDoInsertUpdateTime(self.iToDoInsert))
            } else {
                return String(cString: getToDoQueryUpdateTime(self.iToDoQuery))
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
    
    public var toDoTypeId: Int32 {
        get {
            return getToDoQuery_ToDoTypeId(self.iToDoQuery)
        }
        
        set {
            setToDoQuery_ToDoTypeId(self.iToDoQuery, newValue)
        }
    }
    
    public var toDoTagId: Int32 {
        get {
            return getToDoQuery_ToDoTagId(self.iToDoQuery)
        }
        
        set {
            setToDoQuery_ToDoTagId(self.iToDoQuery, newValue)
        }
    }
    
    public var state: PJToDoState {
        get {
            if let value = PJToDoState(rawValue: Int(getToDoQueryState(self.iToDoQuery))) {
                return value
            }
            return .determined
        }
        
        set {
            setToDoQueryState(self.iToDoQuery, Int32(newValue.rawValue))
        }
    }
    
    /*This constructor is used when inserting data.*/
    public init(mode: PJToDoMode = .model) {
        self.mode = mode
    }
    
    /*This constructor is used by ToDoTypeController when getting data from db.*/
    public init(iToDoQuery: OpaquePointer?, iToDoType: OpaquePointer?, iToDoTag: OpaquePointer?) {
        self.iToDoQuery = iToDoQuery;
        self.iToDoType = iToDoType;
        self.iToDoTag = iToDoTag;
    }
}
