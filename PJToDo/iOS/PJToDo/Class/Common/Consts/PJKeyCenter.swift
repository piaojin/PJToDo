//
//  PJKeyCenter.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/11/15.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import Foundation

@objc public class PJKeyCenterOC: NSObject {
    @objc static var KeychainUserInfoService: String {
        return PJKeyCenter.KeychainUserInfoService
    }
    
    @objc static var KeychainAuthorizationService: String {
        return PJKeyCenter.KeychainAuthorizationService
    }
    
    @objc static var KeychainAccessTokenKey: String {
        return PJKeyCenter.KeychainAccessTokenKey
    }
}

public struct PJKeyCenter {
    public static let UserInfo: String = "UserInfo"
    public static let Authorization: String = "Authorization"
    public static let InsertToDoNotification = "InsertToDoNotification"
    public static let KeychainUserInfoService = "UserInfoService"
    public static let KeychainAuthorizationService = "AuthorizationService"
    public static let KeychainAuthorizationKey = "PJToDoAuthorization"
    public static let KeychainAccessTokenKey = "KeychainAccessTokenKey"
    public static let KeychainUserPassWordKey: String = "UserPassWord"
    public static let ReposKey: String = "ReposKey"
    public static let ReposDBFileKey: String = "ReposDBFileKey"
    public static let ShouldUpdateDBToGitHubKey: String = "ShouldUpdateDBToGitHubKey"
}
