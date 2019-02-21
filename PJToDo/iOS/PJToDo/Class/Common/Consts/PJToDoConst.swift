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
    @objc public static let DBToDoSQLFileName = "pj_to_db_todo_sql.txt"
    @objc public static let DBTypeSQLFileName = "pj_to_db_type_sql.txt"
    @objc public static let DBTagSQLFileName = "pj_to_db_tag_sql.txt"
    @objc public static let DBToDoSettingsSQLFileName = "pj_to_db_todo_settings_sql.txt"
    @objc public static let DBTypeSQLFilePath = PJCacheManager.shared.documnetPath + "/\(PJToDoConst.DBTypeSQLFileName)"
    @objc public static let DBTagSQLFilePath = PJCacheManager.shared.documnetPath + "/\(PJToDoConst.DBTagSQLFileName)"
    @objc public static let DBToDoSQLFilePath = PJCacheManager.shared.documnetPath + "/\(PJToDoConst.DBToDoSQLFileName)"
    @objc public static let DBToDoSettingsSQLFilePath = PJCacheManager.shared.documnetPath + "/\(PJToDoConst.DBToDoSettingsSQLFileName)"
    @objc public static let DBPath = PJCacheManager.shared.documnetPath + "/\(PJToDoConst.DBName)";
    @objc public static let DBGZipPath = PJCacheManager.shared.documnetPath + "/\(PJToDoConst.DBGZipName)";
    @objc public static let DBUnCompressesPath = PJCacheManager.shared.documnetPath + "/\(PJToDoConst.DBUnCompressesName)";
    @objc public static let PJToDoWebDataBase = "PJToDoWebDataBase"
    @objc public static let PJDownLoadFolderPath = PJCacheManager.shared.documnetPath + "/Download/"
    @objc public static let PJDownLoadToDoZipFilePath = PJToDoConst.PJDownLoadFolderPath + "\(PJToDoConst.DBGZipName)"
}
