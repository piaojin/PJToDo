//
//  WelcomeViewController.swift
//  PJToDo
//
//  Created by piaojin on 2019/1/12.
//  Copyright © 2019 piaojin. All rights reserved.
//

import UIKit

class WelcomeViewController: PJBaseViewController {

    var logoImageView: UIImageView = {
        let logoImageView = UIImageView(image: UIImage(named: "dog"))
        logoImageView.translatesAutoresizingMaskIntoConstraints = false
        return logoImageView
    }()
    
    var loginButton: UIButton = {
        let loginButton = UIButton()
        loginButton.translatesAutoresizingMaskIntoConstraints = false
        loginButton.setTitle("登录", for: .normal)
        loginButton.setTitleColor(.white, for: .normal)
        loginButton.backgroundColor = UIColor.colorWithRGB(red: 0, green: 123, blue: 249)
        return loginButton
    }()
    
    override func viewDidLoad() {
        super.viewDidLoad()
        self.initView()
        self.initData()
    }
    
    private func initView() {
        self.title = "欢迎"
        self.view.backgroundColor = UIColor.colorWithRGB(red: 249, green: 249, blue: 249)
        
        let bgView = UIView()
        bgView.backgroundColor = UIColor.colorWithRGB(red: 249, green: 249, blue: 249)
        bgView.translatesAutoresizingMaskIntoConstraints = false
        
        bgView.addSubview(self.logoImageView)
        self.logoImageView.topAnchor.constraint(equalTo: bgView.topAnchor).isActive = true
        self.logoImageView.widthAnchor.constraint(equalToConstant: 100).isActive = true
        self.logoImageView.heightAnchor.constraint(equalToConstant: 100).isActive = true
        self.logoImageView.centerXAnchor.constraint(equalTo: bgView.centerXAnchor).isActive = true
        
        bgView.addSubview(self.loginButton)
        self.loginButton.leadingAnchor.constraint(equalTo: bgView.leadingAnchor, constant: 50).isActive = true
        self.loginButton.trailingAnchor.constraint(equalTo: bgView.trailingAnchor, constant: -50).isActive = true
        self.loginButton.topAnchor.constraint(equalTo: self.logoImageView.bottomAnchor, constant: 15).isActive = true
        self.loginButton.heightAnchor.constraint(equalToConstant: 40).isActive = true
        self.loginButton.bottomAnchor.constraint(equalTo: bgView.bottomAnchor).isActive = true
        self.loginButton.cornerRadius = 6
        self.loginButton.layer.masksToBounds = true
        
        self.view.addSubview(bgView)
        bgView.leadingAnchor.constraint(equalTo: self.view.leadingAnchor).isActive = true
        bgView.trailingAnchor.constraint(equalTo: self.view.trailingAnchor).isActive = true
        bgView.centerYAnchor.constraint(equalTo: self.view.centerYAnchor).isActive = true
    }
    
    private func initData() {
        self.loginButton.addTarget(self, action: #selector(loginAction), for: .touchUpInside)
    }
    
    @objc private func loginAction() {
//        if let account = PJUserInfoManager.shared.userInfo?.login {
//            if let sentiveStr = try? PJKeychainManager.readSensitiveString(withService: PJKeyCenter.KeychainUserInfoService, sensitiveKey: account) {
//                let base64Str = String.convertToBase64String(str: "\(account):\(sentiveStr)")
//                let basicToken = "Basic \(base64Str)"
//                PJHttpRequest.authorization(authorization: basicToken) { (authorization, isSuccess) in
//                    if (isSuccess) {
//                        try? PJKeychainManager.saveSensitiveString(withService: PJKeyCenter.KeychainAuthorizationService, sensitiveKey: PJKeyCenter.KeychainAuthorizationKey, sensitiveString: basicToken)
//                    }
//                }
//            }
//        }
        
        PJHttpRequest.createGitHubRepos { (isSuccess, resultStr, error) in
            if isSuccess {

            }
        }
        
//        let loginViewController = LoginViewController()
//        if self.navigationController != nil {
//            self.navigationController?.pushViewController(loginViewController, animated: true)
//        } else {
//            self.present(loginViewController, animated: true, completion: nil)
//        }
    }
}
