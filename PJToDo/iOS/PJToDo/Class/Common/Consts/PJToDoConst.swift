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
    @objc public static let PJDownLoadFolderPath = PJToDoConst.PJToDoData + "/DownLoad"
    @objc public static let DBName = "pj_to_db.db"
    @objc public static let DBGZipName = "pj_to_db.zip"
    @objc public static let DBUnCompressesName = "pj_to_db_uncompresses.db"
    @objc public static let DBToDoSQLFileName = "pj_to_db_todo_sql.txt"
    @objc public static let DBTypeSQLFileName = "pj_to_db_type_sql.txt"
    @objc public static let DBTagSQLFileName = "pj_to_db_tag_sql.txt"
    @objc public static let DBToDoSettingsSQLFileName = "pj_to_db_todo_settings_sql.txt"
    
    @objc public static let DBTypeSQLFilePath = PJToDoConst.PJToDoData + "/\(PJToDoConst.DBTypeSQLFileName)"
    @objc public static let DBTagSQLFilePath = PJToDoConst.PJToDoData + "/\(PJToDoConst.DBTagSQLFileName)"
    @objc public static let DBToDoSQLFilePath = PJToDoConst.PJToDoData + "/\(PJToDoConst.DBToDoSQLFileName)"
    @objc public static let DBToDoSettingsSQLFilePath = PJToDoConst.PJToDoData + "/\(PJToDoConst.DBToDoSettingsSQLFileName)"
    @objc public static let DBPath = PJToDoConst.PJToDoData + "/\(PJToDoConst.DBName)"
    @objc public static let DBGZipPath = PJToDoConst.PJToDoData + "/\(PJToDoConst.DBGZipName)"
    @objc public static let DBUnCompressesPath = PJToDoConst.PJToDoData + "/\(PJToDoConst.DBUnCompressesName)"
    
    @objc public static let PJDownLoadToDoZipFilePath = PJToDoConst.PJDownLoadFolderPath + "/\(PJToDoConst.DBGZipName)"
    @objc public static let PJDownLoadToDoUnZipFileFolderPath = PJToDoConst.PJToDoData + "/DownLoad/pj_to_db"
    @objc public static let PJUnZipGitHubToDoSQLFilePath = PJToDoConst.PJDownLoadToDoUnZipFileFolderPath + "/\(PJToDoConst.DBToDoSQLFileName)"
    @objc public static let PJUnZipGitHubTypeSQLFilePath = PJToDoConst.PJDownLoadToDoUnZipFileFolderPath + "/\(PJToDoConst.DBTypeSQLFileName)"
    @objc public static let PJUnZipGitHubTagSQLFilePath = PJToDoConst.PJDownLoadToDoUnZipFileFolderPath + "/\(PJToDoConst.DBTagSQLFileName)"
    @objc public static let PJUnZipGitHubSettingsSQLFilePath = PJToDoConst.PJDownLoadToDoUnZipFileFolderPath + "/\(PJToDoConst.DBToDoSettingsSQLFileName)"
}
