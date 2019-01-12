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
    
    //MARK: User
    
    public static func login(name: String, passWord: String, responseBlock: ((_ data: User?, _ isSuccess : Bool) -> Void)?) {
        let httpRequestConfig = PJHttpRequestConfig(responseBlock: { (pointer, isSuccess) -> Void in
            DispatchQueue.main.async(execute: {
                if let tempPointer = pointer {
                    do {
                        let jsonString = String.create(cString: tempPointer)
                        if let jsonData = jsonString.data(using: .utf8) {
                            let user = try JSONDecoder().decode(User.self, from: jsonData)
                            PJCacheManager.saveCustomObject(customObject: user, key: PJKeyCenter.UserInfo)
                            responseBlock?(user, isSuccess)
                        } else {
                            responseBlock?(nil, isSuccess)
                        }
                    } catch {
                        // 异常处理
                        DDLogWarn("parse user info error: \(error)")
                        responseBlock?(nil, isSuccess)
                    }
                } else {
                    responseBlock?(nil, isSuccess)
                }
            })
        })
        
        PJ_Login(httpRequestConfig.iDelegate, name, passWord)
    }
    
    public static func authorization(authorization: String, responseBlock: ((_ data: Authorization?, _ isSuccess : Bool) -> Void)?) {
        let httpRequestConfig = PJHttpRequestConfig(responseBlock: { (pointer, isSuccess) -> Void in
            if let tempPointer = pointer {
                do {
                    let jsonString = String.create(cString: tempPointer)
                    if let jsonData = jsonString.data(using: .utf8) {
                        let authorization = try JSONDecoder().decode(Authorization.self, from: jsonData)
                        PJCacheManager.saveCustomObject(customObject: authorization, key: PJKeyCenter.Authorization)
                        responseBlock?(authorization, isSuccess)
                    } else {
                        responseBlock?(nil, isSuccess)
                    }
                } catch {
                    // 异常处理
                    DDLogWarn("parse authorization info error: \(error)")
                    responseBlock?(nil, isSuccess)
                }
            } else {
                responseBlock?(nil, isSuccess)
            }
        })
        
        PJ_Authorizations(httpRequestConfig.iDelegate, authorization)
    }
    
    public static func requestUserInfo(responseBlock: ((_ data: User?, _ isSuccess : Bool) -> Void)?) {
        let httpRequestConfig = PJHttpRequestConfig(responseBlock: { (pointer, isSuccess) -> Void in
            DispatchQueue.main.async(execute: {
                if let tempPointer = pointer {
                    do {
                        let jsonString = String.create(cString: tempPointer)
                        if let jsonData = jsonString.data(using: .utf8) {
                            let user = try JSONDecoder().decode(User.self, from: jsonData)
                            PJCacheManager.saveCustomObject(customObject: user, key: PJKeyCenter.UserInfo)
                            responseBlock?(user, isSuccess)
                        } else {
                            responseBlock?(nil, isSuccess)
                        }
                    } catch {
                        // 异常处理
                        DDLogWarn("parse user info error: \(error)")
                        responseBlock?(nil, isSuccess)
                    }
                } else {
                    responseBlock?(nil, isSuccess)
                }
            })
        })
        
        PJ_RequestUserInfo(httpRequestConfig.iDelegate)
    }
    
    //MARK: Repos
    
    public static func createGitHubRepos(responseBlock: ((_ resultString: String?, _ isSuccess : Bool) -> Void)?) {
        let httpRequestConfig = self.createHttpRequestConfig(responseBlock: responseBlock)
        PJ_CreateRepos(httpRequestConfig.iDelegate)
    }
    
    public static func getGitHubRepos(reposUrl: String, responseBlock: ((_ resultString: String?, _ isSuccess : Bool) -> Void)?) {
        let httpRequestConfig = self.createHttpRequestConfig(responseBlock: responseBlock)
        PJ_GetRepos(httpRequestConfig.iDelegate, reposUrl)
    }
    
    public static func deleteGitHubRepos(reposUrl: String, responseBlock: ((_ resultString: String?, _ isSuccess : Bool) -> Void)?) {
        let httpRequestConfig = self.createHttpRequestConfig(responseBlock: responseBlock)
        PJ_DeleteRepos(httpRequestConfig.iDelegate, reposUrl)
    }
    
    //MARK: File
    
    public static func createGitHubFile(path: String, message: String, content: String, sha: String, responseBlock: ((_ resultString: String?, _ isSuccess : Bool) -> Void)?) {
        let httpRequestConfig = self.createHttpRequestConfig(responseBlock: responseBlock)
        PJ_CreateFile(httpRequestConfig.iDelegate, path, message, content, sha)
    }
    
    public static func updateGitHubFile(path: String, message: String, content: String, sha: String, responseBlock: ((_ resultString: String?, _ isSuccess : Bool) -> Void)?) {
        let httpRequestConfig = self.createHttpRequestConfig(responseBlock: responseBlock)
        PJ_UpdateFile(httpRequestConfig.iDelegate, path, message, content, sha)
    }
    
    public static func deleteGitHubFile(path: String, message: String, content: String, sha: String, responseBlock: ((_ resultString: String?, _ isSuccess : Bool) -> Void)?) {
        let httpRequestConfig = self.createHttpRequestConfig(responseBlock: responseBlock)
        PJ_DeleteFile(httpRequestConfig.iDelegate, path, message, content, sha)
    }
    
    private static func createHttpRequestConfig(responseBlock: ((_ resultString: String?, _ isSuccess : Bool) -> Void)?) -> PJHttpRequestConfig {
        let httpRequestConfig = PJHttpRequestConfig(responseBlock: { (pointer, isSuccess) -> Void in
            DispatchQueue.main.async(execute: {
                if let tempPointer = pointer {
                    let jsonString = String.create(cString: tempPointer)
                    responseBlock?(jsonString, isSuccess)
                } else {
                    responseBlock?(nil, isSuccess)
                }
            })
        })
        return httpRequestConfig
    }
}
