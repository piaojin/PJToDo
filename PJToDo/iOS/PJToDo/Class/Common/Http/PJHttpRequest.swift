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
    
    public static func createPJHttpError(jsonString: String) -> PJHttpError? {
        var httpError: PJHttpError?
        if let jsonData = jsonString.data(using: .utf8) {
            do {
                httpError = try JSONDecoder().decode(PJHttpError.self, from: jsonData)
            } catch {
                DDLogWarn("❌createPJHttpError -> parse PJHttpError error: \(error)❌")
            }
        } else {
            httpError = PJHttpError()
            httpError?.message = "❌createPJHttpError -> jsonString to data error!!!❌"
        }
        return httpError
    }
    
    //MARK: User
    
    public static func login(name: String, passWord: String, responseBlock: ((_ isSuccess : Bool, _ data: User?, _ error: PJHttpError?) -> Void)?) {
        let httpRequestConfig = PJHttpRequestConfig(responseBlock: { (pointer, statusCode, isSuccess) -> Void in
            DispatchQueue.main.async(execute: {
                var httpError: PJHttpError?
                if let tempPointer = pointer {
                    let jsonString = String.create(cString: tempPointer)
                    if isSuccess {
                        do {
                            if let jsonData = jsonString.data(using: .utf8) {
                                let user = try JSONDecoder().decode(User.self, from: jsonData)
                                PJCacheManager.saveCustomObject(customObject: user, key: PJKeyCenter.UserInfo)
                                responseBlock?(isSuccess, user, nil)
                            } else {
                                httpError = PJHttpError()
                                httpError?.message = "❌login -> jsonString to data error!!!❌"
                                responseBlock?(isSuccess, nil, httpError)
                            }
                        } catch {
                            // 异常处理
                            DDLogWarn("parse user info error: \(error)")
                            httpError = PJHttpError()
                            httpError?.message = "❌login -> parse user info error: \(error)❌"
                            responseBlock?(isSuccess, nil, httpError)
                        }
                    } else {
                        httpError = self.createPJHttpError(jsonString: jsonString)
                        responseBlock?(isSuccess, nil, httpError)
                    }
                } else {
                    httpError = PJHttpError()
                    httpError?.message = "❌login -> pointer is null!!!❌"
                    responseBlock?(isSuccess, nil, httpError)
                }
            })
        })
        
        PJ_Login(httpRequestConfig.iDelegate, name, passWord)
    }
    
    public static func authorization(authorization: String, responseBlock: ((_ isSuccess : Bool, _ data: Authorizations?, _ error: PJHttpError?) -> Void)?) {
        let httpRequestConfig = PJHttpRequestConfig(responseBlock: { (pointer, statusCode, isSuccess) -> Void in
            var httpError: PJHttpError?
            if let tempPointer = pointer {
                let jsonString = String.create(cString: tempPointer)
                if isSuccess {
                    do {
                        if let jsonData = jsonString.data(using: .utf8) {
                            let authorization = try JSONDecoder().decode(Authorizations.self, from: jsonData)
                            PJCacheManager.saveCustomObject(customObject: authorization, key: PJKeyCenter.Authorization)
                            responseBlock?(isSuccess, authorization, nil)
                        } else {
                            httpError = PJHttpError()
                            httpError?.message = "❌authorization -> jsonString to data error!!!❌"
                            responseBlock?(isSuccess, nil, httpError)
                        }
                    } catch {
                        // 异常处理
                        DDLogWarn("parse authorization info error: \(error)")
                        httpError = PJHttpError()
                        httpError?.message = "❌authorization -> parse user info error: \(error)❌"
                        responseBlock?(isSuccess, nil, httpError)
                    }
                } else {
                    httpError = self.createPJHttpError(jsonString: jsonString)
                    responseBlock?(isSuccess, nil, httpError)
                }
            } else {
                httpError = PJHttpError()
                httpError?.message = "❌authorization -> pointer is null!!!❌"
                responseBlock?(isSuccess, nil, httpError)
            }
        })
        
        PJ_Authorizations(httpRequestConfig.iDelegate, authorization)
    }
    
    public static func requestUserInfo(responseBlock: ((_ isSuccess : Bool, _ data: User?, _ error: PJHttpError?) -> Void)?) {
        let httpRequestConfig = PJHttpRequestConfig(responseBlock: { (pointer, statusCode, isSuccess) -> Void in
            DispatchQueue.main.async(execute: {
                var httpError: PJHttpError?
                if let tempPointer = pointer {
                    let jsonString = String.create(cString: tempPointer)
                    if isSuccess {
                        do {
                            if let jsonData = jsonString.data(using: .utf8) {
                                let user = try JSONDecoder().decode(User.self, from: jsonData)
                                PJCacheManager.saveCustomObject(customObject: user, key: PJKeyCenter.UserInfo)
                                responseBlock?(isSuccess, user, nil)
                            } else {
                                httpError = PJHttpError()
                                httpError?.message = "❌login -> jsonString to data error!!!❌"
                                responseBlock?(isSuccess, nil, httpError)
                            }
                        } catch {
                            // 异常处理
                            DDLogWarn("parse user info error: \(error)")
                            httpError = PJHttpError()
                            httpError?.message = "❌login -> parse user info error: \(error)❌"
                            responseBlock?(isSuccess, nil, httpError)
                        }
                    } else {
                        httpError = self.createPJHttpError(jsonString: jsonString)
                        responseBlock?(isSuccess, nil, httpError)
                    }
                } else {
                    httpError = PJHttpError()
                    httpError?.message = "❌login -> pointer is null!!!❌"
                    responseBlock?(isSuccess, nil, httpError)
                }
            })
        })
        
        PJ_RequestUserInfo(httpRequestConfig.iDelegate)
    }
    
    //MARK: Repos
    
    public static func createGitHubRepos(responseBlock: ((_ isSuccess : Bool, _ resultString: String?, _ error: PJHttpError?) -> Void)?) {
        let httpRequestConfig = self.createHttpRequestConfig(responseBlock: responseBlock)
        PJ_CreateRepos(httpRequestConfig.iDelegate)
    }
    
    public static func getGitHubRepos(reposUrl: String, responseBlock: ((_ isSuccess : Bool, _ resultString: String?, _ error: PJHttpError?) -> Void)?) {
        let httpRequestConfig = self.createHttpRequestConfig(responseBlock: responseBlock)
        PJ_GetRepos(httpRequestConfig.iDelegate, reposUrl)
    }
    
    public static func deleteGitHubRepos(reposUrl: String, responseBlock: ((_ isSuccess : Bool, _ resultString: String?, _ error: PJHttpError?) -> Void)?) {
        let httpRequestConfig = self.createHttpRequestConfig(responseBlock: responseBlock)
        PJ_DeleteRepos(httpRequestConfig.iDelegate, reposUrl)
    }
    
    //MARK: File
    
    public static func createGitHubFile(path: String, message: String, content: String, sha: String, responseBlock: ((_ isSuccess : Bool, _ resultString: String?, _ error: PJHttpError?) -> Void)?) {
        let httpRequestConfig = self.createHttpRequestConfig(responseBlock: responseBlock)
        PJ_CreateFile(httpRequestConfig.iDelegate, path, message, content, sha)
    }
    
    public static func updateGitHubFile(path: String, message: String, content: String, sha: String, responseBlock: ((_ isSuccess : Bool, _ resultString: String?, _ error: PJHttpError?) -> Void)?) {
        let httpRequestConfig = self.createHttpRequestConfig(responseBlock: responseBlock)
        PJ_UpdateFile(httpRequestConfig.iDelegate, path, message, content, sha)
    }
    
    public static func deleteGitHubFile(path: String, message: String, content: String, sha: String, responseBlock: ((_ isSuccess : Bool, _ resultString: String?, _ error: PJHttpError?) -> Void)?) {
        let httpRequestConfig = self.createHttpRequestConfig(responseBlock: responseBlock)
        PJ_DeleteFile(httpRequestConfig.iDelegate, path, message, content, sha)
    }
    
    private static func createHttpRequestConfig(responseBlock: ((_ isSuccess : Bool, _ resultString: String?, _ error: PJHttpError?) -> Void)?) -> PJHttpRequestConfig {
        let httpRequestConfig = PJHttpRequestConfig(responseBlock: { (pointer, statusCode, isSuccess) -> Void in
            DispatchQueue.main.async(execute: {
                var httpError: PJHttpError?
                if let tempPointer = pointer {
                    let jsonString = String.create(cString: tempPointer)
                    if isSuccess {
                        responseBlock?(isSuccess, jsonString, nil)
                    } else {
                        httpError = self.createPJHttpError(jsonString: jsonString)
                        responseBlock?(isSuccess, jsonString, httpError)
                    }
                } else {
                    httpError = PJHttpError()
                    httpError?.message = "❌createHttpRequestConfig -> pointer is null!!!❌"
                    responseBlock?(isSuccess, nil, httpError)
                }
            })
        })
        return httpRequestConfig
    }
}
