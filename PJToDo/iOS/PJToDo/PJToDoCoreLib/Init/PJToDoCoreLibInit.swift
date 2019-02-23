//
//  PJToDoCoreLibInit.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/10/5.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit
import CocoaLumberjack
import SQLite3

public struct PJToDoCoreLibInit {
    static public func initRustCoreLib() {
        DDLogInfo("******start init CoreLib******")
        initFolder(PJToDoConst.PJToDoData)
        initCoreLib()
        initFolder(PJToDoConst.PJDownLoadFolderPath)
        test_pal_from_rust()
        DDLogInfo("******end init CoreLib******")
    }
    
    static public func initFolderIfNeed() {
        initFolder(PJToDoConst.PJToDoData)
        initFolder(PJToDoConst.PJDownLoadFolderPath)
    }
    
    static public func initDBIfNeed() {
        initDataBase()
        initTables()
    }
    
    static public func updateConnection() {
        updateDBConnection()
    }
}
