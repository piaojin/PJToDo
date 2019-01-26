//
//  PJToDoConst.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/10/6.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

@objc class PJToDoConst: NSObject {
    @objc public static let DBName = "pj_to_db.db"
    @objc public static let DBGZipName = "pj_to_db.zip"
    @objc public static let DBUnCompressesName = "pj_to_db_uncompresses.db"
    @objc public static let DBPath = PJCacheManager.shared.documnetPath + "/\(PJToDoConst.DBName)";
    @objc public static let DBGZipPath = PJCacheManager.shared.documnetPath + "/\(PJToDoConst.DBGZipName)";
    @objc public static let DBUnCompressesPath = PJCacheManager.shared.documnetPath + "/\(PJToDoConst.DBUnCompressesName)";
    @objc public static let PJToDoWebDataBase = "PJToDoWebDataBase"
}
