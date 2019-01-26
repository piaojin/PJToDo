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
        SSZipArchive.createZipFile(atPath: PJToDoConst.DBGZipPath, withFilesAtPaths: [PJToDoConst.DBPath])
        SSZipArchive.unzipFile(atPath: PJToDoConst.DBGZipPath, toDestination: PJCacheManager.shared.documnetPath, overwrite: true, password: nil, progressHandler: nil) { (path, isSuccess, error) in
            if isSuccess {
                guard let dbData = FileManager.default.contents(atPath: path) else {
                    completedHandle?(false, nil, PJHttpError(errorCode: 0, errorMessage: "❌Load db zip file error!❌"))
                    return
                }
                
                guard let dbContent = String(data: dbData, encoding: String.Encoding.utf8) else {
                    completedHandle?(false, nil, PJHttpError(errorCode: 0, errorMessage: "❌Load db zip file content error!❌"))
                    return
                }
                
                guard let base64DBContent = ConvertStrToBase64Str(dbContent) else {
                    completedHandle?(false, nil, PJHttpError(errorCode: 0, errorMessage: "❌Convert db zip file to base64 error!❌"))
                    return
                }
                
                PJHttpRequest.createGitHubReposFile(requestUrl: PJHttpUrlConst.BaseReposFileUrl, path: PJHttpUrlConst.GitHubReposDBFilePath, message: "Create PJToDo DB file.", content: String.create(cString: base64DBContent), sha: "", responseBlock: { (isSuccess, reposFile, error) in
                    PJReposFileManager.shared.hasCreateReposDBFileOnGitHub = isSuccess
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
            } else {
                completedHandle?(false, nil, PJHttpError(errorCode: 0, errorMessage: error.debugDescription))
            }
        }
    }
    
    public static func updateReposFile(completedHandle: ((Bool, ReposFile?, PJHttpError?) -> ())?) {
        SSZipArchive.createZipFile(atPath: PJToDoConst.DBGZipPath, withFilesAtPaths: [PJToDoConst.DBPath])
        SSZipArchive.unzipFile(atPath: PJToDoConst.DBGZipPath, toDestination: PJCacheManager.shared.documnetPath, overwrite: true, password: nil, progressHandler: nil) { (path, isSuccess, error) in
            if isSuccess {
                guard PJReposFileManager.shared.hasSavedReposDBFileInLocal, PJReposFileManager.shared.hasCreateReposDBFileOnGitHub else {
//                    SVProgressHUD.showError(withStatus: "Didn't init user data successfully, please logout and login try again!")
                    completedHandle?(false, nil, PJHttpError(errorCode: 0, errorMessage: "❌Didn't init user data successfully!❌"))
                    return
                }
                
                guard let reposFile = PJReposFileManager.shared.reposFile else {
                    completedHandle?(false, nil, PJHttpError(errorCode: 0, errorMessage: "❌Didn't init user data successfully!❌"))
                    return
                }
                
                guard let dbData = FileManager.default.contents(atPath: path) else {
//                    SVProgressHUD.showError(withStatus: "An unexpected mistake has occurred, please try again!")
                    completedHandle?(false, nil, PJHttpError(errorCode: 0, errorMessage: "❌Load db zip file error!❌"))
                    return
                }
                
                guard let dbContent = String(data: dbData, encoding: String.Encoding.utf8) else {
                    completedHandle?(false, nil, PJHttpError(errorCode: 0, errorMessage: "❌Load db zip file content error!❌"))
                    return
                }
                
                guard let base64DBContent = ConvertStrToBase64Str(dbContent) else {
                    completedHandle?(false, nil, PJHttpError(errorCode: 0, errorMessage: "❌Convert db zip file to base64 error!❌"))
                    return
                }
                
                PJHttpRequest.updateGitHubReposFile(requestUrl: PJHttpUrlConst.BaseReposFileUrl, path: PJHttpUrlConst.GitHubReposDBFilePath, message: "Update PJToDo DB file.", content: String.create(cString: base64DBContent), sha: reposFile.content.sha, responseBlock: { (isSuccess, reposFile, error) in
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
            } else {
                completedHandle?(false, nil, PJHttpError(errorCode: 0, errorMessage: error.debugDescription))
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
        PJHttpRequest.getGitHubReposFile(requestUrl: PJHttpUrlConst.BaseReposFileUrl) { (isSuceesss, reposFile, error) in
            if isSuceesss {
                PJReposFileManager.shared.hasCreateReposDBFileOnGitHub = true
                if let tempReposFile = reposFile {
                    self.saveReposFile(reposFile: tempReposFile)
                    completedHandle?(isSuceesss, reposFile, error)
                } else {
                    completedHandle?(false, reposFile, error)
                }
            } else {
                //Didn't create repos file
                if let errorCode = error?.errorCode, PJHttpReponseStatusCode(rawValue: errorCode) == PJHttpReponseStatusCode.HTTP_STATUS_NOT_FOUND {
                    PJReposFileManager.shared.hasCreateReposDBFileOnGitHub = false
                    DDLogError("❌Haven't create repos yet!❌")
                }
                completedHandle?(isSuceesss, reposFile, error)
            }
        }
    }
}
