//
//  PJUserInfoManager.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/11/21.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit
import SVProgressHUD

@objc public class PJUserInfoManagerOC: NSObject {
    @objc static var userAccount: String? {
        return PJUserInfoManager.shared.userInfo?.login
    }
    
    @objc static var isLogin: Bool {
        return PJUserInfoManager.shared.isLogin
    }
}

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
    
    /// The access token will be set into request heads
    public static func loginViaAccessToken(_ loginCompletedHandle: ((Bool) -> Void)? = nil) {
        SVProgressHUD.show(withStatus: "Login...")
        PJHttpRequest.loginViaAccessToken { isSuccess, user, error in
            DispatchQueue.main.async {
                if isSuccess, let tempUser = user {
                    PJUserInfoManager.saveUserInfo(userInfo: tempUser)
                    self.initRootViewController()
                    SVProgressHUD.show(withStatus: "Sync Data...")
                    PJReposManager.initGitHubRepos { isSuccess, _, _ in
                        if isSuccess {
                            PJReposFileManager.initGitHubReposFile(completedHandle: { (isSuccess, _, _) in
                                PJReposFileManager.getReposFile(completedHandle: { (isSuccess, reposFile, error) in
                                    if isSuccess, let tempReposFile = reposFile {
                                        //download github data file
                                        self.downloadDBFromGitHub(reposFile: tempReposFile, completedHandle: loginCompletedHandle)
                                    } else {
                                        SVProgressHUD.dismiss()
                                        loginCompletedHandle?(false)
                                        DispatchQueue.main.async(execute: {
                                            let alert = UIAlertController(title: "Sync Failure", message: "Sync Data failure, please logout and login try again!", preferredStyle: .alert)
                                            
                                            let okAction = UIAlertAction(title: "OK", style: .default, handler: { (_) in
                                                PJUserInfoManager.logOut()
                                            })
                                            
                                            alert.addAction(okAction)
                                            
                                            RootController.shared.modalViewController().present(alert, animated: true, completion: nil)
                                        })
                                    }
                                })
                            })
                        } else {
                            self.showSyncError()
                            loginCompletedHandle?(false)
                        }
                    }
                } else {
                    if let nav = RootController.shared.modalViewController().navigationController {
                        nav.popViewController(animated: true)
                    } else {
                        RootController.shared.modalViewController().dismiss(animated: true)
                    }
                    SVProgressHUD.showError(withStatus: "Login Failure!")
                    loginCompletedHandle?(false)
                }
            }
        }
    }
    
    private static func initRootViewController() {
        if let window = UIApplication.shared.delegate?.window {
            window?.rootViewController = PJTabBarViewController()
            window?.makeKeyAndVisible()
        }
    }
    
    public static func downloadDBFromGitHub(reposFile: ReposFile, completedHandle: ((Bool) -> Void)? = nil) {
        PJHttpRequest.downloadFile(requestUrl: reposFile.content.download_url, savePath: PJToDoConst.DBPath) { (isSuccess, errorString, error) in
            if isSuccess {
                DispatchQueue.main.async(execute: {
                    pj_update_db_connection()
                    DispatchQueue.main.asyncAfter(deadline: DispatchTime.now() + 1, execute: {
                        NotificationCenter.default.post(name: NSNotification.Name.init(PJKeyCenter.InsertToDoNotification), object: nil)
                        SVProgressHUD.dismiss()
                        completedHandle?(true)
                    })
                })
            } else {
                self.showSyncError()
                completedHandle?(false)
            }
        }
    }
    
    public static func showSyncError() {
        DispatchQueue.main.async(execute: {
            SVProgressHUD.showError(withStatus: "Sync Failure!")
        })
    }
    
    public static func logOut() {
        try? PJKeychainManager.deleteItem(withService: PJKeyCenter.KeychainAuthorizationService, sensitiveKey: PJKeyCenter.KeychainAccessTokenKey)
        PJUserInfoManager.removeUserInfo()
        PJReposManager.removeRepos()
        pj_logout()
        pj_remove_folder(PJToDoConst.PJToDoData, true)
        PJToDoCoreLibInit.initRustCoreLib()
        if let window = UIApplication.shared.delegate?.window {
            window?.rootViewController = UINavigationController(rootViewController: WelcomeViewController())
            window?.makeKeyAndVisible()
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
