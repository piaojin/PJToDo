//
//  PJToDoCoreLibInit.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/10/5.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit
import CocoaLumberjack

public struct PJToDoCoreLibInit {
    static public func initRustCoreLib() {
        DDLogInfo("******start init CoreLib******")
        init_hello_piaojin()
//        PJToDoDBManager.initDB()
        init_database();
        init_tables();
        test_pal_from_rust()
        DDLogInfo("******end init CoreLib******")
    }
}
