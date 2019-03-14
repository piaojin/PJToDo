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
        pj_create_folder(PJToDoConst.PJToDoData)
        pj_init_corelib()
        pj_test_pal_from_rust()
        DDLogInfo("******end init CoreLib******")
    }
    
    static public func initDBIfNeed() {
        pj_init_data_base()
        pj_init_tables()
    }
    
    static public func updateConnection() {
        pj_update_db_connection()
    }
}
