//
//  LoginViewController.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/11/21.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit
import SVProgressHUD
import WebKit

class LoginViewController: PJBaseViewController {
    private let wkWebView: WKWebView = {
        let wkWebView = WKWebView()
        wkWebView.translatesAutoresizingMaskIntoConstraints = false
        return wkWebView
    }()
    
    override func viewDidLoad() {
        super.viewDidLoad()
        self.initView()
        self.initData()
    }
    
    private func initView() {
        self.title = "Login"
        self.view.backgroundColor = UIColor.colorWithRGB(red: 249, green: 249, blue: 249)
        view.addSubview(wkWebView)
        NSLayoutConstraint.activate([
            wkWebView.topAnchor.constraint(equalTo: view.topAnchor),
            wkWebView.bottomAnchor.constraint(equalTo: view.bottomAnchor),
            wkWebView.leadingAnchor.constraint(equalTo: view.leadingAnchor),
            wkWebView.trailingAnchor.constraint(equalTo: view.trailingAnchor)
        ])
    }
    
    private func initData() {
        wkWebView.navigationDelegate = self
        guard let url = URL(string: "https://github.com/login/oauth/authorize?client_id=\(PJToDoConst.githubClientID)&redirect_uri=\(PJToDoConst.githubAuthCallBack)&scope=repo%20admin:org%20admin:public_key%20admin:repo_hook%20admin:org_hook%20gist%20notifications%20user%20delete_repo%20write:discussion%20admin:gpg_key") else { return }

        wkWebView.load(URLRequest(url: url))
    }
    
    private func loginAction() {
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
                                        self.downloadDBFromGitHub(reposFile: tempReposFile)
                                    } else {
                                        SVProgressHUD.dismiss()
                                        DispatchQueue.main.async(execute: {
                                            let alert = UIAlertController(title: "Sync Failure", message: "Sync Data failure, please logout and login try again!", preferredStyle: .alert)
                                            
                                            let okAction = UIAlertAction(title: "OK", style: .default, handler: { (_) in
                                                PJUserInfoManager.logOut()
                                            })
                                            
                                            alert.addAction(okAction)
                                            
                                            self.present(alert, animated: true, completion: nil)
                                        })
                                    }
                                })
                            })
                        } else {
                            self.showSyncError()
                        }
                    }
                } else {
                    self.navigationController?.popViewController(animated: true)
                    SVProgressHUD.showError(withStatus: "Login Failure!")
                }
            }
        }
    }
    
    private func initRootViewController() {
        if let window = UIApplication.shared.delegate?.window {
            window?.rootViewController = PJTabBarViewController()
            window?.makeKeyAndVisible()
        }
    }
    
    private func downloadDBFromGitHub(reposFile: ReposFile) {
        PJHttpRequest.downloadFile(requestUrl: reposFile.content.download_url, savePath: PJToDoConst.DBPath) { (isSuccess, errorString, error) in
            if isSuccess {
                DispatchQueue.main.async(execute: {
                    pj_update_db_connection()
                    DispatchQueue.main.asyncAfter(deadline: DispatchTime.now() + 1, execute: {
                        NotificationCenter.default.post(name: NSNotification.Name.init(PJKeyCenter.InsertToDoNotification), object: nil)
                        SVProgressHUD.dismiss()
                    })
                })
            } else {
                self.showSyncError()
            }
        }
    }
    
    private func showSyncError() {
        DispatchQueue.main.async(execute: {
            SVProgressHUD.showError(withStatus: "Sync Failure!")
        })
    }
}

extension LoginViewController: WKNavigationDelegate {
    func webView(_ webView: WKWebView, decidePolicyFor navigationResponse: WKNavigationResponse, decisionHandler: @escaping (WKNavigationResponsePolicy) -> Void) {
        guard let path = webView.url?.path, PJToDoConst.githubAuthCallBack.contains(path), let url = webView.url else {
            decisionHandler(.allow)
            return
        }
        
        SVProgressHUD.show(withStatus: "Login...")
        
        if let queryItems = URLComponents(url: url, resolvingAgainstBaseURL: false)?.queryItems {
            var githubCode: String?
            for item in queryItems {
                print("\(item.name): \(item.value ?? "")")
                if item.name == "code" {
                    githubCode = item.value ?? ""
                    break
                }
            }
            
            if let githubCode = githubCode {
                try? PJKeychainManager.deleteItem(withService: PJKeyCenter.KeychainAuthorizationService, sensitiveKey: PJKeyCenter.KeychainAccessTokenKey)
                PJHttpRequest.accessToken(code: githubCode, clientID: PJToDoConst.githubClientID, clientSecret: PJToDoConst.githubClientSecrets) { isSuccess, data, error in
                    if isSuccess, let accessToken = data {
                        try? PJKeychainManager.saveSensitiveString(withService: PJKeyCenter.KeychainAuthorizationService, sensitiveKey: PJKeyCenter.KeychainAccessTokenKey, sensitiveString: accessToken.access_token)
                        self.loginAction()
                    } else {
                        SVProgressHUD.showError(withStatus: "Login Failure!")
                    }
                }
            }
        }
        decisionHandler(.cancel)
    }
}
