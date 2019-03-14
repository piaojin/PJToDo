//
//  PJToDoTag.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/11/3.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

public class PJToDoTag: NSObject {
    private(set) var iToDoTagInsert: OpaquePointer?
    
    private(set) var iToDoTag: OpaquePointer?
    
    private var mode: PJToDoMode = .model
    
    public var tagName: String {
        get {
            if self.mode == .insert {
                return String.create(cString: pj_get_todo_tag_insert_name(self.iToDoTagInsert))
            } else {
                return String.create(cString: pj_get_todo_tag_name(self.iToDoTag))
            }
        }
        
        set {
            if self.mode == .insert {
                pj_set_todo_tag_insert_name(self.iToDoTagInsert, newValue)
            } else {
                pj_set_todo_tag_name(self.iToDoTag, newValue)
            }
        }
    }
    
    public var tagId: Int32 {
        get {
            return pj_get_todo_tag_id(self.iToDoTag)
        }
        
        set {
            pj_set_todo_tag_id(self.iToDoTag, newValue)
        }
    }
    
    /*This constructor is used when inserting data.*/
    public init(insertTagName: String) {
        super.init()
        self.iToDoTagInsert = pj_create_ToDoTagInsert(insertTagName)
        self.mode = .insert
    }
    
    /*This constructor is used by ToDoTagController when getting data from db.*/
    public init(iToDoTag: OpaquePointer?) {
        super.init()
        self.iToDoTag = iToDoTag;
    }
    
    public init(tagId: Int32, tagName: String) {
        super.init()
        self.iToDoTag = pj_create_ToDoTag(tagName)
        self.tagId = tagId;
    }
}
