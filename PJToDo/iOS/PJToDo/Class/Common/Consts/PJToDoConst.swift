//
//  PJToDoConst.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/10/6.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

@objc class PJToDoConst: NSObject {
    @objc public static let PJToDoData = PJCacheManager.shared.documnetPath + "/PJToDoData"
    @objc public static let PJToDoWebDataBase = "PJToDoWebDataBase"
    @objc public static let DBName = "pj_to_db.db"
    @objc public static let DBPath = PJToDoConst.PJToDoData + "/\(PJToDoConst.DBName)"
}
