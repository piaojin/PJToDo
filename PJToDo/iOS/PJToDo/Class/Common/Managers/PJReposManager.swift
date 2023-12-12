//
//  PJReposManager.swift
//  PJToDo
//
//  Created by piaojin on 2019/1/26.
//  Copyright © 2019 piaojin. All rights reserved.
//

import Foundation
import CocoaLumberjack
import SSZipArchive
import SVProgressHUD

public struct PJReposManager {
    public static var shared = PJReposManager()
    
    public var hasSavedReposInLocal: Bool {
        if PJCacheManager.getCustomObject(type: Repos.self(), forKey: PJKeyCenter.ReposKey) != nil {
            return true
        }
        return false
    }
    
    public var hasCreateReposOnGitHub: Bool = false
    
    private var _repos: Repos?
    
    public var repos: Repos? {
        mutating get {
            guard let r = self._repos else {
                if let tempRepos = PJCacheManager.getCustomObject(type: Repos.self(), forKey: PJKeyCenter.ReposKey) {
                    self._repos = tempRepos
                    return tempRepos
                }
                return nil
            }
            return r
        }
        
        set {
            PJReposManager.shared._repos = newValue
        }
    }
    
    public static func saveRepos(repos: Repos) {
        PJReposManager.shared._repos = repos
        PJCacheManager.saveCustomObject(customObject: repos, key: PJKeyCenter.ReposKey)
    }
    
    public static func removeRepos() {
        PJReposManager.shared._repos = nil
        PJReposManager.shared.hasCreateReposOnGitHub = false
        PJCacheManager.removeCustomObject(key: PJKeyCenter.ReposKey)
    }
    
    public static func getRepos(completedHandle: ((Bool, Repos?, PJHttpError?) -> ())?) {
        PJHttpRequest.getGitHubRepos(reposUrl: PJHttpUrlConst.GetReposUrl) { (isSuccess, repos, error) in
            //Has created repos
            if isSuccess {
                PJReposManager.shared.hasCreateReposOnGitHub = true
                if let tempRepos = repos {
                    self.saveRepos(repos: tempRepos)
                    completedHandle?(isSuccess, repos, error)
                } else {
                    completedHandle?(false, repos, error)
                }
            } else {
                //Didn't create repos
                if let errorCode = error?.errorCode, PJHttpReponseStatusCode(rawValue: errorCode) == PJHttpReponseStatusCode.HTTP_STATUS_NOT_FOUND {
                    PJReposManager.shared.hasCreateReposOnGitHub = false
                    DDLogError("❌❌❌❌❌❌Haven't create repos yet!❌❌❌❌❌❌")
                }
                completedHandle?(isSuccess, repos, error)
            }
        }
    }
    
    public static func createRepos(completedHandle: ((Bool, Repos?, PJHttpError?) -> ())?) {
        PJHttpRequest.createGitHubRepos { (isSuccess, repos, error) in
            //Create repos success
            PJReposManager.shared.hasCreateReposOnGitHub = isSuccess
            if isSuccess {
                if let tempRepos = repos {
                    self.saveRepos(repos: tempRepos)
                }
                completedHandle?(isSuccess, repos, error)
                DDLogInfo("💯Create repos success!💯")
            } else {
                //Has created
                if let errorCode = error?.errorCode, PJHttpReponseStatusCode(rawValue: errorCode) == PJHttpReponseStatusCode.HTTP_STATUS_UNPROCESSABLE_ENTITY {
                    PJReposManager.shared.hasCreateReposOnGitHub = true
                    DDLogInfo("Has create repos!")
                    completedHandle?(true, repos, error)
                } else {
                    //Create repos error
                    PJReposManager.shared.hasCreateReposOnGitHub = false
                    DDLogError("❌❌❌❌❌❌Create repos error!❌❌❌❌❌❌")
                    completedHandle?(isSuccess, repos, error)
                }
            }
        }
    }
    
    public static func deleteRepos(completedHandle: ((Bool, Repos?, PJHttpError?) -> ())?) {
        PJHttpRequest.deleteGitHubRepos(reposUrl: PJHttpUrlConst.DeleteReposUrl) { (isSuccess, repos, error) in
            if isSuccess {
                PJReposManager.shared.hasCreateReposOnGitHub = false
                self.removeRepos()
            }
            completedHandle?(isSuccess, repos, error)
        }
    }
    
    public static func initGitHubRepos(completedHandle: ((Bool, Repos?, PJHttpError?) -> ())?) {
        if PJUserInfoManager.shared.isLogin {
            PJReposManager.createRepos(completedHandle: completedHandle)
        }
    }
}
