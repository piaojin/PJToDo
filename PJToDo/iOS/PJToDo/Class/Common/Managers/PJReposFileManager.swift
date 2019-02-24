//
//  PJReposFileManager.swift
//  PJToDo
//
//  Created by piaojin on 2019/1/26.
//  Copyright © 2019 piaojin. All rights reserved.
//

import Foundation
import CocoaLumberjack
import SSZipArchive
import SVProgressHUD

public struct PJReposFileManager {
    public static var shared = PJReposFileManager()
    
    public var hasSavedReposDBFileInLocal: Bool {
        if PJCacheManager.getCustomObject(type: ReposFile.self(), forKey: PJKeyCenter.ReposDBFileKey) != nil {
            return true
        }
        return false
    }
    
    public var hasCreateReposDBFileOnGitHub: Bool = false
    
    private var _reposFile: ReposFile?
    
    public var reposFile: ReposFile? {
        mutating get {
            guard let r = self._reposFile else {
                if let tempReposFile = PJCacheManager.getCustomObject(type: ReposFile.self(), forKey: PJKeyCenter.ReposDBFileKey) {
                    self._reposFile = tempReposFile
                    return tempReposFile
                }
                return nil
            }
            return r
        }
        
        set {
            PJReposFileManager.shared._reposFile = newValue
        }
    }
    
    public static func saveReposFile(reposFile: ReposFile) {
        PJReposFileManager.shared._reposFile = reposFile
        PJCacheManager.saveCustomObject(customObject: reposFile, key: PJKeyCenter.ReposDBFileKey)
    }
    
    public static func removeReposFile() {
        PJReposFileManager.shared._reposFile = nil
        PJReposFileManager.shared.hasCreateReposDBFileOnGitHub = false
        PJCacheManager.removeCustomObject(key: PJKeyCenter.ReposDBFileKey)
    }
    
    public static func createReposFile(completedHandle: ((Bool, ReposFile?, PJHttpError?) -> ())?) {
        DispatchQueue.global().async {
            SSZipArchive.createZipFile(atPath: PJToDoConst.DBGZipPath, withFilesAtPaths: [PJToDoConst.DBToDoSQLFilePath, PJToDoConst.DBTypeSQLFilePath, PJToDoConst.DBTagSQLFilePath, PJToDoConst.DBToDoSettingsSQLFilePath])
            
            do {
                let data = try Data(contentsOf: URL(fileURLWithPath: PJToDoConst.DBGZipPath))
                let base64DBContent = data.base64EncodedString()
                
                PJHttpRequest.createGitHubReposFile(requestUrl: PJHttpUrlConst.BaseReposFileUrl, path: PJHttpUrlConst.GitHubReposDBFilePath, message: "Create PJToDo DB file.", content: base64DBContent, sha: "", responseBlock: { (isSuccess, reposFile, error) in
                    PJReposFileManager.shared.hasCreateReposDBFileOnGitHub = isSuccess
                    if isSuccess {
                        if let tempReposFile = reposFile {
                            self.saveReposFile(reposFile: tempReposFile)
                            completedHandle?(isSuccess, tempReposFile, error)
                        } else {
                            completedHandle?(false, reposFile, error)
                        }
                    } else {
                        //Has created
                        if let errorCode = error?.errorCode, PJHttpReponseStatusCode(rawValue: errorCode) == PJHttpReponseStatusCode.HTTP_STATUS_UNPROCESSABLE_ENTITY {
                            PJReposFileManager.shared.hasCreateReposDBFileOnGitHub = true
                            DDLogInfo("Has create repos file yet!")
                            completedHandle?(true, reposFile, error)
                        } else {
                            completedHandle?(isSuccess, reposFile, error)
                        }
                    }
                })
            } catch {
                DDLogError("\(error)")
                completedHandle?(false, nil, PJHttpError(errorCode: 0, errorMessage: "❌Load db zip file error!❌"))
                return
            }
        }
    }
    
    public static func updateReposFile(completedHandle: ((Bool, ReposFile?, PJHttpError?) -> ())?) {
        DispatchQueue.global().async {
            SSZipArchive.createZipFile(atPath: PJToDoConst.DBGZipPath, withFilesAtPaths: [PJToDoConst.DBToDoSQLFilePath, PJToDoConst.DBTypeSQLFilePath, PJToDoConst.DBTagSQLFilePath, PJToDoConst.DBToDoSettingsSQLFilePath])
            
            guard let reposFile = PJReposFileManager.shared.reposFile else {
                completedHandle?(false, nil, PJHttpError(errorCode: 0, errorMessage: "❌Didn't init user data successfully!❌"))
                return
            }
            
            do {
                let data = try Data(contentsOf: URL(fileURLWithPath: PJToDoConst.DBGZipPath))
                let base64DBContent = data.base64EncodedString()
                
                PJHttpRequest.updateGitHubReposFile(requestUrl: PJHttpUrlConst.BaseReposFileUrl, path: PJHttpUrlConst.GitHubReposDBFilePath, message: "Update PJToDo DB file.", content: base64DBContent, sha: reposFile.content.sha, responseBlock: { (isSuccess, reposFile, error) in
                    if isSuccess {
                        if let tempReposFile = reposFile {
                            self.saveReposFile(reposFile: tempReposFile)
                            completedHandle?(isSuccess, tempReposFile, error)
                        } else {
                            completedHandle?(false, reposFile, error)
                        }
                    } else {
                        completedHandle?(isSuccess, reposFile, error)
                    }
                })
            } catch {
                DDLogError("\(error)")
                completedHandle?(false, nil, PJHttpError(errorCode: 0, errorMessage: "❌Load db zip file error!❌"))
                return
            }
        }
    }
    
    public static func deleteReposFile(completedHandle: ((Bool, ReposFile?, PJHttpError?) -> ())?) {
        guard PJReposFileManager.shared.hasSavedReposDBFileInLocal, PJReposFileManager.shared.hasCreateReposDBFileOnGitHub else {
            completedHandle?(false, nil, PJHttpError(errorCode: 0, errorMessage: "❌Didn't init user data successfully!❌"))
            return
        }
        
        guard let reposFile = PJReposFileManager.shared.reposFile else {
            completedHandle?(false, nil, PJHttpError(errorCode: 0, errorMessage: "❌Didn't init user data successfully!❌"))
            return
        }
        
        PJHttpRequest.deleteGitHubReposFile(requestUrl: PJHttpUrlConst.BaseReposFileUrl, path: PJHttpUrlConst.GitHubReposDBFilePath, message: "Delete PJToDo DB file.", content: "", sha: reposFile.content.sha, responseBlock: { (isSuccess, reposFile, error) in
            if isSuccess {
                self.removeReposFile()
            }
            completedHandle?(isSuccess, reposFile, error)
        })
    }
    
    public static func getReposFile(completedHandle: ((Bool, ReposFile?, PJHttpError?) -> ())?) {
        PJHttpRequest.getGitHubReposFile(requestUrl: PJHttpUrlConst.BaseReposFileUrl) { (isSuceesss, reposFileContent, error) in
            PJReposFileManager.shared.hasCreateReposDBFileOnGitHub = isSuceesss
            if isSuceesss {
                if let tempReposFileContent = reposFileContent {
                    let reposFile = ReposFile()
                    reposFile.content = tempReposFileContent
                    self.saveReposFile(reposFile: reposFile)
                    completedHandle?(isSuceesss, reposFile, error)
                } else {
                    completedHandle?(false, nil, error)
                }
            } else {
                //Didn't create repos file
                if let errorCode = error?.errorCode, PJHttpReponseStatusCode(rawValue: errorCode) == PJHttpReponseStatusCode.HTTP_STATUS_NOT_FOUND {
                    PJReposFileManager.shared.hasCreateReposDBFileOnGitHub = false
                    DDLogError("❌Haven't create repos yet!❌")
                }
                completedHandle?(isSuceesss, nil, error)
            }
        }
    }
    
    public static func initGitHubReposFile(completedHandle: ((Bool, ReposFile?, PJHttpError?) -> ())?) {
        if PJUserInfoManager.shared.isLogin {
            PJReposFileManager.createReposFile(completedHandle: completedHandle)
        }
    }
}
