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
        
        pj_login(httpRequestConfig.iDelegate, name, passWord)
    }
    
    public static func authorization(authorization: String, responseBlock: ((_ isSuccess : Bool, _ data: Authorizations?, _ error: PJHttpError?) -> Void)?) {
        let httpRequestConfig = PJHttpRequestConfig(responseBlock: { (pointer, statusCode, isSuccess) -> Void in
            DispatchQueue.main.async(execute: {
                PJHttpRequest.handleResponse(pointer, statusCode, isSuccess, actionName: "authorization", responseBlock: responseBlock)
            })
        })
        
        pj_authorizations(httpRequestConfig.iDelegate, authorization)
    }
    
    public static func requestUserInfo(responseBlock: ((_ isSuccess : Bool, _ data: User?, _ error: PJHttpError?) -> Void)?) {
        let httpRequestConfig = PJHttpRequestConfig(responseBlock: { (pointer, statusCode, isSuccess) -> Void in
            DispatchQueue.main.async(execute: {
                PJHttpRequest.handleResponse(pointer, statusCode, isSuccess, actionName: "requestUserInfo", responseBlock: responseBlock)
            })
        })
        
        pj_request_user_info(httpRequestConfig.iDelegate)
    }
    
    //MARK: Repos
    
    public static func createGitHubRepos(responseBlock: ((_ isSuccess : Bool, _ data: Repos?, _ error: PJHttpError?) -> Void)?) {
        let httpRequestConfig = self.createHttpRequestConfig(actionName: "createGitHubRepos", responseBlock: responseBlock)
        pj_create_repos(httpRequestConfig.iDelegate)
    }
    
    public static func getGitHubRepos(reposUrl: String, responseBlock: ((_ isSuccess : Bool, _ data: Repos?, _ error: PJHttpError?) -> Void)?) {
        let httpRequestConfig = self.createHttpRequestConfig(actionName: "getGitHubRepos", responseBlock: responseBlock)
        pj_get_repos(httpRequestConfig.iDelegate, reposUrl)
    }
    
    public static func deleteGitHubRepos(reposUrl: String, responseBlock: ((_ isSuccess : Bool, _ data: Repos?, _ error: PJHttpError?) -> Void)?) {
        let httpRequestConfig = self.createHttpRequestConfig(actionName: "deleteGitHubRepos", responseBlock: responseBlock)
        pj_delete_repos(httpRequestConfig.iDelegate, reposUrl)
    }
    
    //MARK: File
    
    public static func createGitHubReposFile(requestUrl: String, path: String, message: String, content: String, sha: String, responseBlock: ((_ isSuccess : Bool, _ data: ReposFile?, _ error: PJHttpError?) -> Void)?) {
        let httpRequestConfig = self.createHttpRequestConfig(actionName: "createGitHubFile", responseBlock: responseBlock)
        pj_create_repos_file(httpRequestConfig.iDelegate, requestUrl, path, message, content, sha)
    }
    
    public static func updateGitHubReposFile(requestUrl: String, path: String, message: String, content: String, sha: String, responseBlock: ((_ isSuccess : Bool, _ data: ReposFile?, _ error: PJHttpError?) -> Void)?) {
        let httpRequestConfig = self.createHttpRequestConfig(actionName: "updateGitHubFile", responseBlock: responseBlock)
        pj_update_repos_file(httpRequestConfig.iDelegate, requestUrl, path, message, content, sha)
    }
    
    public static func deleteGitHubReposFile(requestUrl: String, path: String, message: String, content: String, sha: String, responseBlock: ((_ isSuccess : Bool, _ data: ReposFile?, _ error: PJHttpError?) -> Void)?) {
        let httpRequestConfig = self.createHttpRequestConfig(actionName: "deleteGitHubFile", responseBlock: responseBlock)
        pj_delete_repos_file(httpRequestConfig.iDelegate, requestUrl, path, message, content, sha)
    }
    
    public static func getGitHubReposFile(requestUrl: String, responseBlock: ((_ isSuccess : Bool, _ data: ReposFileContent?, _ error: PJHttpError?) -> Void)?) {
        let httpRequestConfig = self.createHttpRequestConfig(actionName: "getGitHubReposFile", responseBlock: responseBlock)
        pj_get_repos_file(httpRequestConfig.iDelegate, requestUrl)
    }
    
    public static func downloadFile(requestUrl: String, savePath: String, responseBlock: ((_ isSuccess : Bool, _ data: String?, _ error: PJHttpError?) -> Void)?) {
        let httpRequestConfig = PJHttpRequestConfig(responseBlock: { (pointer, statusCode, isSuccess) -> Void in
            if isSuccess {
                responseBlock?(isSuccess, "", nil)
            } else {
                if let tempPointer = pointer {
                    let errorString = String.create(cString: tempPointer)
                    let httpError = PJHttpError(errorCode: statusCode, errorMessage: errorString)
                    responseBlock?(isSuccess, errorString, httpError)
                } else {
                    let httpError = PJHttpError(errorCode: statusCode, errorMessage: "download file unknow error")
                    responseBlock?(isSuccess, "download file unknow error", httpError)
                }
            }
        })
        pj_download_file(httpRequestConfig.iDelegate, requestUrl, savePath)
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
