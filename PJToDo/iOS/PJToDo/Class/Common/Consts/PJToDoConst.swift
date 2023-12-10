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
    public static let githubClientID = "c8dc6ee8217584bc6544"
    public static let githubClientSecrets = "c3ef423d3acca131720c6fa684e39fb2b0ed7767"
    public static let githubAuthCallBack = "https://github.com/piaojin/pjtodo/authorization/callback_url"
}
