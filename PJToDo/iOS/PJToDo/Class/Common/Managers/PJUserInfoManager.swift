//
//  PJUserInfoManager.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/11/21.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

@objc public class PJUserInfoManagerOC: NSObject {
    @objc static var userAccount: String? {
        return PJUserInfoManager.shared.userInfo?.login
    }
    
    @objc static var isLogin: Bool {
        return PJUserInfoManager.shared.isLogin
    }
}

public struct PJUserInfoManager {
    
    public static var shared = PJUserInfoManager()
    
    var isLogin: Bool {
        if PJCacheManager.getCustomObject(type: User.self(), forKey: PJKeyCenter.UserInfo) != nil {
            return true
        }
        return false
    }
    
    private var _userInfo: User?
    
    var userInfo: User? {
        mutating get {
            guard let u = self._userInfo else {
            if let user = PJCacheManager.getCustomObject(type: User.self(), forKey: PJKeyCenter.UserInfo) {
                self._userInfo = user
                return user
            }
                return nil
            }
            return u
        }
        
        set {
            PJUserInfoManager.shared._userInfo = newValue
        }
    }
    
    public static func logOut() {
        if let account = PJUserInfoManager.shared.userInfo?.login {
            try? PJKeychainManager.deleteItem(withService: PJKeyCenter.KeychainUserInfoService, sensitiveKey: account)
        }
        PJUserInfoManager.removeUserInfo()
        PJReposManager.removeRepos()
        PJ_LogOut()
        if let window = UIApplication.shared.delegate?.window {
            window?.rootViewController = UINavigationController(rootViewController: WelcomeViewController())
            window?.makeKeyAndVisible()
        }
    }
    
    public static func saveUserInfo(userInfo: User) {
        PJUserInfoManager.shared._userInfo = userInfo
        PJCacheManager.saveCustomObject(customObject: userInfo, key: PJKeyCenter.UserInfo)
    }
    
    public static func removeUserInfo() {
        PJUserInfoManager.shared._userInfo = nil
        PJCacheManager.removeCustomObject(key: PJKeyCenter.UserInfo)
    }
}
