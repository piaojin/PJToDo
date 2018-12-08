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
                return String.create(cString: getToDoSettingsInsertRemindEmail(self.iToDoSettingsInsert))
            } else {
                if let pointer = self.iToDoSettings {
                    return String.create(cString: getToDoSettingsRemindEmail(pointer))
                }
                return ""
            }
        }
        
        set {
            if self.mode == .insert {
                setToDoSettingsInsertRemindEmail(self.iToDoSettingsInsert, newValue)
            } else {
                setToDoSettingsRemindEmail(self.iToDoSettings, newValue)
            }
        }
    }
    
    public var remindDays: Int32 {
        get {
            if self.mode == .insert {
                return getToDoSettingsInsertRemindDays(self.iToDoSettingsInsert)
            } else {
                if let pointer = self.iToDoSettings {
                    return getToDoSettingsRemindDays(pointer)
                }
                return -1
            }
        }
        
        set {
            if self.mode == .insert {
                setToDoSettingsInsertRemindDays(self.iToDoSettingsInsert, newValue)
            } else {
                setToDoSettingsRemindDays(self.iToDoSettings, newValue)
            }
        }
    }
    
    public var settingsId: Int32 {
        get {
            if let pointer = self.iToDoSettings {
                return getToDoSettingsId(pointer)
            }
            return -1
        }
        
        set {
            setToDoSettingsId(self.iToDoSettings, newValue)
        }
    }
    
    /*This constructor is used when inserting data.*/
    public init(remindEmail: String, remindDays: Int32) {
        self.iToDoSettingsInsert = createToDoSettingsInsert(remindEmail, remindDays)
        self.mode = .insert
    }
    
    /*This constructor is used by ToDoTypeController when getting data from db.*/
    public init(iToDoSettings: OpaquePointer?) {
        self.iToDoSettings = iToDoSettings
    }
    
    public init(settingsId: Int32, remindEmail: String, remindDays: Int32) {
        self.iToDoSettings = createToDoSettings(remindEmail, remindDays)
        self.settingsId = settingsId
    }
}

