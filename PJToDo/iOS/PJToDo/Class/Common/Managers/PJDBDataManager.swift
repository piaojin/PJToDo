//
//  PJDBDataManager.swift
//  PJToDo
//
//  Created by piaojin on 2019/2/22.
//  Copyright © 2019 piaojin. All rights reserved.
//

import SVProgressHUD
import SSZipArchive
import UIKit

class PJDBDataManager: NSObject {
    public static var shared = PJDBDataManager()
    
    private lazy var toDoSyncGitHubDataController = ToDoSyncGitHubDataController(delegate: self)
    
    // TODO: sync github data to sqlite
    // Sync Type -> Tag -> Settings -> ToDo
    public func syncGitHubDataToDB() {
        DispatchQueue.global().async {
            removeFolder(PJToDoConst.PJDownLoadToDoUnZipFileFolderPath, true)
            SSZipArchive.unzipFile(atPath: PJToDoConst.PJDownLoadToDoZipFilePath, toDestination: PJToDoConst.PJDownLoadToDoUnZipFileFolderPath, progressHandler: nil, completionHandler: { (path, isSuccess, error) in
                if isSuccess {
                    self.toDoSyncGitHubDataController.syncGitHubTypeDataResult(filePath: PJToDoConst.PJUnZipGitHubTypeSQLFilePath)
                }
            })
        }
    }
    
    public func showSyncGitHubDataErrorAlert() {
        DispatchQueue.main.async {
            SVProgressHUD.showError(withStatus: "Sync Data failure, please logout and login try again!")
        }
    }
}

extension PJDBDataManager: ToDoSyncGitHubDataDelegate {
    func syncGitHubToDoDataResult(isSuccess: Bool) {
        DispatchQueue.main.async {
            if isSuccess {
                NotificationCenter.default.post(name: NSNotification.Name.init(PJKeyCenter.InsertToDoNotification), object: nil)
            } else {
                self.showSyncGitHubDataErrorAlert()
            }
        }
    }
    
    func syncGitHubTypeDataResult(isSuccess: Bool) {
        if isSuccess {
            toDoSyncGitHubDataController.syncGitHubTagDataResult(filePath: PJToDoConst.PJUnZipGitHubTagSQLFilePath)
        } else {
            self.showSyncGitHubDataErrorAlert()
        }
    }
    
    func syncGitHubTagDataResult(isSuccess: Bool) {
        if isSuccess {
            toDoSyncGitHubDataController.syncGitHubSettingsDataResult(filePath: PJToDoConst.PJUnZipGitHubSettingsSQLFilePath)
        } else {
            self.showSyncGitHubDataErrorAlert()
        }
    }
    
    func syncGitHubSettingsDataResult(isSuccess: Bool) {
        if isSuccess {
            toDoSyncGitHubDataController.syncGitHubToDoDataResult(filePath: PJToDoConst.PJUnZipGitHubToDoSQLFilePath)
        } else {
            self.showSyncGitHubDataErrorAlert()
        }
    }
}
