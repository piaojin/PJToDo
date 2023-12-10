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
        PJUserInfoManager.loginViaAccessToken()
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
