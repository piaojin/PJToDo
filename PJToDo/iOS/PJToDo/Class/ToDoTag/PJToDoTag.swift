//
//  PJToDoTag.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/11/3.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

public class PJToDoTag {
    private(set) var iToDoTagInsert: OpaquePointer?
    
    private(set) var iToDoTag: OpaquePointer?
    
    private var mode: PJToDoMode = .model
    
    public var tagName: String {
        get {
            if self.mode == .insert {
                return String(cString: getToDoTagInsertName(self.iToDoTagInsert))
            } else {
                return String(cString: getToDoTagName(self.iToDoTag))
            }
        }
        
        set {
            if self.mode == .insert {
                setToDoTagInsertName(self.iToDoTagInsert, newValue)
            } else {
                setToDoTagName(self.iToDoTag, newValue)
            }
        }
    }
    
    public var tagId: Int32 {
        get {
            return getToDoTagId(self.iToDoTag)
        }
        
        set {
            setToDoTagId(self.iToDoTag, newValue)
        }
    }
    
    /*This constructor is used when inserting data.*/
    public init(insertTagName: String) {
        self.iToDoTagInsert = createToDoTagInsert(insertTagName)
        self.mode = .insert
    }
    
    /*This constructor is used by ToDoTagController when getting data from db.*/
    public init(iToDoTag: OpaquePointer?) {
        self.iToDoTag = iToDoTag;
    }
    
    public init(tagId: Int32, tagName: String) {
        self.iToDoTag = createToDoTag(tagName)
        self.tagId = tagId;
    }
}