//
//  PJUserInfoManager.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/11/21.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

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
    
    public static func saveUserInfo(userInfo: User) {
        PJUserInfoManager.shared._userInfo = userInfo
        PJCacheManager.saveCustomObject(customObject: userInfo, key: PJKeyCenter.UserInfo)
    }
    
    public static func removeUserInfo() {
        PJUserInfoManager.shared._userInfo = nil
        PJCacheManager.removeCustomObject(key: PJKeyCenter.UserInfo)
    }
}
