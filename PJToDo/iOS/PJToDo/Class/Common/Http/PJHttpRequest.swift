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
                PJHttpRequest.handleResponse(pointer, statusCode, isSuccess, actionName: "login", responseBlock: responseBlock)
            })
        })
        
        PJ_Login(httpRequestConfig.iDelegate, name, passWord)
    }
    
    public static func authorization(authorization: String, responseBlock: ((_ isSuccess : Bool, _ data: Authorizations?, _ error: PJHttpError?) -> Void)?) {
        let httpRequestConfig = PJHttpRequestConfig(responseBlock: { (pointer, statusCode, isSuccess) -> Void in
            DispatchQueue.main.async(execute: {
                PJHttpRequest.handleResponse(pointer, statusCode, isSuccess, actionName: "authorization", responseBlock: responseBlock)
            })
        })
        
        PJ_Authorizations(httpRequestConfig.iDelegate, authorization)
    }
    
    public static func requestUserInfo(responseBlock: ((_ isSuccess : Bool, _ data: User?, _ error: PJHttpError?) -> Void)?) {
        let httpRequestConfig = PJHttpRequestConfig(responseBlock: { (pointer, statusCode, isSuccess) -> Void in
            DispatchQueue.main.async(execute: {
                PJHttpRequest.handleResponse(pointer, statusCode, isSuccess, actionName: "requestUserInfo", responseBlock: responseBlock)
            })
        })
        
        PJ_RequestUserInfo(httpRequestConfig.iDelegate)
    }
    
    //MARK: Repos
    
    public static func createGitHubRepos(responseBlock: ((_ isSuccess : Bool, _ data: Repos?, _ error: PJHttpError?) -> Void)?) {
        let httpRequestConfig = self.createHttpRequestConfig(actionName: "createGitHubRepos", responseBlock: responseBlock)
        PJ_CreateRepos(httpRequestConfig.iDelegate)
    }
    
    public static func getGitHubRepos(reposUrl: String, responseBlock: ((_ isSuccess : Bool, _ data: Repos?, _ error: PJHttpError?) -> Void)?) {
        let httpRequestConfig = self.createHttpRequestConfig(actionName: "getGitHubRepos", responseBlock: responseBlock)
        PJ_GetRepos(httpRequestConfig.iDelegate, reposUrl)
    }
    
    public static func deleteGitHubRepos(reposUrl: String, responseBlock: ((_ isSuccess : Bool, _ data: Repos?, _ error: PJHttpError?) -> Void)?) {
        let httpRequestConfig = self.createHttpRequestConfig(actionName: "deleteGitHubRepos", responseBlock: responseBlock)
        PJ_DeleteRepos(httpRequestConfig.iDelegate, reposUrl)
    }
    
    //MARK: File
    
    public static func createGitHubReposFile(requestUrl: String, path: String, message: String, content: String, sha: String, responseBlock: ((_ isSuccess : Bool, _ data: ReposFile?, _ error: PJHttpError?) -> Void)?) {
        let httpRequestConfig = self.createHttpRequestConfig(actionName: "createGitHubFile", responseBlock: responseBlock)
        PJ_CreateReposFile(httpRequestConfig.iDelegate, requestUrl, path, message, content, sha)
    }
    
    public static func updateGitHubReposFile(requestUrl: String, path: String, message: String, content: String, sha: String, responseBlock: ((_ isSuccess : Bool, _ data: ReposFile?, _ error: PJHttpError?) -> Void)?) {
        let httpRequestConfig = self.createHttpRequestConfig(actionName: "updateGitHubFile", responseBlock: responseBlock)
        PJ_UpdateReposFile(httpRequestConfig.iDelegate, requestUrl, path, message, content, sha)
    }
    
    public static func deleteGitHubReposFile(requestUrl: String, path: String, message: String, content: String, sha: String, responseBlock: ((_ isSuccess : Bool, _ data: ReposFile?, _ error: PJHttpError?) -> Void)?) {
        let httpRequestConfig = self.createHttpRequestConfig(actionName: "deleteGitHubFile", responseBlock: responseBlock)
        PJ_DeleteReposFile(httpRequestConfig.iDelegate, requestUrl, path, message, content, sha)
    }
    
    public static func getGitHubReposFile(requestUrl: String, responseBlock: ((_ isSuccess : Bool, _ data: ReposFileContent?, _ error: PJHttpError?) -> Void)?) {
        let httpRequestConfig = self.createHttpRequestConfig(actionName: "getGitHubReposFile", responseBlock: responseBlock)
        PJ_GetReposFile(httpRequestConfig.iDelegate, requestUrl)
    }
    
    private static func createHttpRequestConfig<T: Codable>(actionName: String, responseBlock: ((_ isSuccess : Bool, _ data: T?, _ error: PJHttpError?) -> Void)?) -> PJHttpRequestConfig {
        let httpRequestConfig = PJHttpRequestConfig(responseBlock: { (pointer, statusCode, isSuccess) -> Void in
            DispatchQueue.main.async(execute: {
                PJHttpRequest.handleResponse(pointer, statusCode, isSuccess, actionName: actionName, responseBlock: responseBlock)
            })
        })
        return httpRequestConfig
    }
    
    public static func handleResponse<T: Codable>(_ dataPointer: UnsafeMutablePointer<Int8>?, _ statusCode: Int, _ isSuccess : Bool, actionName: String, responseBlock: ((_ isSuccess : Bool, _ data: T?, _ error: PJHttpError?) -> Void)?) {
        if let tempPointer = dataPointer {
            let jsonString = String.create(cString: tempPointer)
            if isSuccess {
                do {
                    if let jsonData = jsonString.data(using: .utf8) {
                        let model = try JSONDecoder().decode(T.self, from: jsonData)
                        responseBlock?(isSuccess, model, nil)
                    } else {
                        responseBlock?(isSuccess, nil, PJHttpError(errorCode: statusCode, errorMessage: "❌\(actionName) -> jsonString to data error!!!❌"))
                    }
                } catch {
                    // 异常处理
                    DDLogWarn("❌\(actionName) parse user info error: \(error)❌")
                    responseBlock?(isSuccess, nil, PJHttpError(errorCode: statusCode, errorMessage: "❌\(actionName) -> parse user info error: \(error)❌"))
                }
            } else {
                let httpError = self.createPJHttpError(jsonString: jsonString)
                httpError?.errorCode = statusCode
                responseBlock?(isSuccess, nil, httpError)
            }
        } else {
            responseBlock?(isSuccess, nil, PJHttpError(errorCode: statusCode, errorMessage: "❌\(actionName) -> pointer is null!!!❌"))
        }
    }
}
