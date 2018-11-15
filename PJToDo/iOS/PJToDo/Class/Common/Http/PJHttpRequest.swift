//
//  PJHttpRequest.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/11/11.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import Foundation
import CocoaLumberjack

public struct PJHttpRequest {
    public static func login(name: String, passWord: String, responseBlock: ((_ data: User?, _ isSuccess : Bool) -> Void)?) {
        let httpRequestConfig = PJHttpRequestConfig(responseBlock: { (pointer, isSuccess) -> Void in
            if let tempPointer = pointer {
                do {
                    let jsonString = String(cString: tempPointer)
                    if let jsonData = jsonString.data(using: .utf8) {
                        let user = try JSONDecoder().decode(User.self, from: jsonData)
                        PJCacheManager.saveCustomObject(customObject: user, key: PJKeyCenter.userInfo)
                        responseBlock?(user, isSuccess)
                    } else {
                        responseBlock?(nil, isSuccess)
                    }
                } catch {
                    // 异常处理
                    DDLogWarn("parse user info error: \(error)")
                    responseBlock?(nil, isSuccess)
                }
                
                /*data to User and then free data pointer*/
                free_rust_object(UnsafeMutableRawPointer(mutating: tempPointer))
            } else {
                responseBlock?(nil, isSuccess)
            }
        })
        
        PJ_Login(httpRequestConfig.iDelegate, name, passWord)
    }
    
    public static func authorization(authorization: String, responseBlock: ((_ data: Authorization?, _ isSuccess : Bool) -> Void)?) {
        let httpRequestConfig = PJHttpRequestConfig(responseBlock: { (pointer, isSuccess) -> Void in
            if let tempPointer = pointer {
                do {
                    let jsonString = String(cString: tempPointer)
                    if let jsonData = jsonString.data(using: .utf8) {
                        let authorization = try JSONDecoder().decode(Authorization.self, from: jsonData)
                        PJCacheManager.saveCustomObject(customObject: authorization, key: PJKeyCenter.authorization)
                        responseBlock?(authorization, isSuccess)
                    } else {
                        responseBlock?(nil, isSuccess)
                    }
                } catch {
                    // 异常处理
                    DDLogWarn("parse authorization info error: \(error)")
                    responseBlock?(nil, isSuccess)
                }
                
                /*data to User and then free data pointer*/
                free_rust_object(UnsafeMutableRawPointer(mutating: tempPointer))
            } else {
                responseBlock?(nil, isSuccess)
            }
        })
        
        PJ_Authorizations(httpRequestConfig.iDelegate, authorization)
    }
}
