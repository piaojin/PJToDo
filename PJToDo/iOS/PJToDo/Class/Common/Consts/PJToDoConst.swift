//
//  PJToDoConst.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/10/6.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

@objc class PJToDoConst: NSObject {
    @objc public static let dbName = "pj_to_db.db"
    @objc public static let dbGZipName = "pj_to_db.gzip"
    @objc public static let dbPath = PJCacheManager.shared.documnetPath + "/\(PJToDoConst.dbName)";
    @objc public static let dbGZipPath = PJCacheManager.shared.documnetPath + "/\(PJToDoConst.dbGZipName)";
}
