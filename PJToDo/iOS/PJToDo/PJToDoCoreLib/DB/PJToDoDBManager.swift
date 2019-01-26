//
//  PJToDoDBManager.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/10/6.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import Foundation
import SQLite3
import CocoaLumberjack

struct PJToDoDBManager {
    static public func initDB() {
        DDLogInfo("******start init DB******")
        var db: OpaquePointer? = nil
        let fm = FileManager()
        if fm.fileExists(atPath: PJToDoConst.DBPath, isDirectory: nil) {
            DDLogInfo("Database already exists..")
            DDLogInfo("******end init DB******")
            return
        }
        
        if sqlite3_open(PJToDoConst.DBPath, &db) == SQLITE_OK {
            DDLogInfo("Successfully opened connection to database at \(PJToDoConst.DBPath)")
        } else {
            DDLogError("Failed opened connection to database at \(PJToDoConst.DBPath)")
        }
        sqlite3_close(db)
        DDLogInfo("******end init DB******")
    }
}
