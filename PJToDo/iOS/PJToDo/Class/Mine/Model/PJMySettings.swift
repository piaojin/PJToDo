//
//  PJMySettings.swift
//  PJMySettings
//
//  Created by Zoey Weng on 2018/11/10.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import Foundation

public class PJMySettings {
    private(set) var iToDoSettingsInsert: OpaquePointer?
    
    private(set) var iToDoSettings: OpaquePointer?
    
    private var mode: PJToDoMode = .model
    
    public var remindEmail: String {
        get {
            if self.mode == .insert {
                return String.create(cString: pj_get_todo_settings_insert_remind_email(self.iToDoSettingsInsert))
            } else {
                if let pointer = self.iToDoSettings {
                    return String.create(cString: pj_get_todo_settings_remind_email(pointer))
                }
                return ""
            }
        }
        
        set {
            if self.mode == .insert {
                pj_set_todo_settings_insert_remind_email(self.iToDoSettingsInsert, newValue)
            } else {
                pj_set_todo_settings_remind_email(self.iToDoSettings, newValue)
            }
        }
    }
    
    public var remindDays: Int32 {
        get {
            if self.mode == .insert {
                return pj_get_todo_settings_insert_remind_days(self.iToDoSettingsInsert)
            } else {
                if let pointer = self.iToDoSettings {
                    return pj_get_todo_settings_remind_days(pointer)
                }
                return -1
            }
        }
        
        set {
            if self.mode == .insert {
                pj_set_todo_settings_insert_remind_days(self.iToDoSettingsInsert, newValue)
            } else {
                pj_set_todo_settings_remind_days(self.iToDoSettings, newValue)
            }
        }
    }
    
    public var settingsId: Int32 {
        get {
            if let pointer = self.iToDoSettings {
                return pj_get_todo_settings_id(pointer)
            }
            return -1
        }
        
        set {
            pj_set_todo_settings_id(self.iToDoSettings, newValue)
        }
    }
    
    /*This constructor is used when inserting data.*/
    public init(remindEmail: String, remindDays: Int32) {
        self.iToDoSettingsInsert = pj_create_ToDoSettingsInsert(remindEmail, remindDays)
        self.mode = .insert
    }
    
    /*This constructor is used by ToDoTypeController when getting data from db.*/
    public init(iToDoSettings: OpaquePointer?) {
        self.iToDoSettings = iToDoSettings
    }
    
    public init(settingsId: Int32, remindEmail: String, remindDays: Int32) {
        self.iToDoSettings = pj_create_ToDoSettings(remindEmail, remindDays)
        self.settingsId = settingsId
    }
}

