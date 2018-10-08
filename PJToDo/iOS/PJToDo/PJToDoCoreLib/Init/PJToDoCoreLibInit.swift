//
//  PJToDoCoreLibInit.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/10/5.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit
import SQLite3

public struct PJToDoCoreLibInit {
    static public func initRustCoreLib() {
        init_hello_piaojin()
        print("Database already exists..")
    }
    
    static public func initDB() {
        
    }
    
    static public func createDB() {
        var db: OpaquePointer? = nil
        let fm = FileManager()
        if fm.fileExists(atPath: "dbPath", isDirectory: nil) {
            print("Database already exists..")
            return
        }
        
        if sqlite3_open("dbPath", &db) == SQLITE_OK {
            print("Successfully opened connection to database at dbPath")
        } else {
            print("Failed opened connection to database at dbPath")
        }
        
        sqlite3_close(db)
    }
}
