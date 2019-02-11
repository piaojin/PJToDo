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
        init_core_lib()
        init_database()
        init_tables()
        init_db_data_sql_file()
        test_pal_from_rust()
        DDLogInfo("******end init CoreLib******")
    }
}
