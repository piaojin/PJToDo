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
        initDownLoadFolder(PJToDoConst.PJToDoData)
        initCoreLib()
        initDownLoadFolder(PJToDoConst.PJDownLoadFolderPath)
        test_pal_from_rust()
        DDLogInfo("******end init CoreLib******")
    }
}
