//
//  LoginViewController.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/11/21.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit
import SVProgressHUD

class LoginViewController: PJBaseViewController {

    var accountTextField: UITextField = {
        let accountTextField = UITextField()
        accountTextField.translatesAutoresizingMaskIntoConstraints = false
        accountTextField.placeholder = "Account"
        accountTextField.backgroundColor = .white
        return accountTextField
    }()
    
    var passwordTextField: UITextField = {
        let passwordTextField = UITextField()
        passwordTextField.translatesAutoresizingMaskIntoConstraints = false
        passwordTextField.placeholder = "PassWord"
        passwordTextField.backgroundColor = .white
        return passwordTextField
    }()
    
    var loginButton: UIButton = {
        let loginButton = UIButton()
        loginButton.translatesAutoresizingMaskIntoConstraints = false
        loginButton.setTitle("Login", for: .normal)
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
        self.title = "Login"
        self.view.backgroundColor = UIColor.colorWithRGB(red: 249, green: 249, blue: 249)
        
        let scrollView = UIScrollView()
        scrollView.translatesAutoresizingMaskIntoConstraints = false
        scrollView.isUserInteractionEnabled = true
        self.view.addSubview(scrollView)
        scrollView.leadingAnchor.constraint(equalTo: self.view.leadingAnchor).isActive = true
        scrollView.trailingAnchor.constraint(equalTo: self.view.trailingAnchor).isActive = true
        scrollView.topAnchor.constraint(equalTo: self.view.topAnchor).isActive = true
        scrollView.bottomAnchor.constraint(equalTo: self.view.bottomAnchor).isActive = true
        
        let bgView = UIView()
        bgView.backgroundColor = UIColor.colorWithRGB(red: 249, green: 249, blue: 249)
        bgView.translatesAutoresizingMaskIntoConstraints = false
        bgView.isUserInteractionEnabled = true
        scrollView.addSubview(bgView)
        bgView.topAnchor.constraint(equalTo: scrollView.topAnchor).isActive = true
        bgView.bottomAnchor.constraint(equalTo: scrollView.bottomAnchor).isActive = true
        bgView.leadingAnchor.constraint(equalTo: scrollView.leadingAnchor).isActive = true
        bgView.trailingAnchor.constraint(equalTo: scrollView.trailingAnchor).isActive = true
        
        let imageView = UIImageView(image: UIImage(named: "github"))
        bgView.addSubview(imageView)
        imageView.translatesAutoresizingMaskIntoConstraints = false
        imageView.widthAnchor.constraint(equalToConstant: 85).isActive = true
        imageView.heightAnchor.constraint(equalToConstant: 85).isActive = true
        imageView.topAnchor.constraint(equalTo: bgView.topAnchor, constant: 45).isActive = true
        imageView.centerXAnchor.constraint(equalTo: self.view.centerXAnchor).isActive = true

        let label = UILabel()
        bgView.addSubview(label)
        label.translatesAutoresizingMaskIntoConstraints = false
        label.text = "Login with GitHub"
        label.topAnchor.constraint(equalTo: imageView.bottomAnchor, constant: 10).isActive = true
        label.centerXAnchor.constraint(equalTo: self.view.centerXAnchor).isActive = true

        bgView.addSubview(self.accountTextField)
        self.accountTextField.heightAnchor.constraint(equalToConstant: 45).isActive = true
        self.accountTextField.centerXAnchor.constraint(equalTo: self.view.centerXAnchor).isActive = true
        self.accountTextField.topAnchor.constraint(equalTo: label.bottomAnchor, constant: 20).isActive = true
        self.accountTextField.leadingAnchor.constraint(equalTo: bgView.leadingAnchor, constant: 62).isActive = true
        self.accountTextField.trailingAnchor.constraint(equalTo: bgView.trailingAnchor, constant: -62).isActive = true
        self.accountTextField.cornerRadius = 6
        self.accountTextField.layer.masksToBounds = true
        
        bgView.addSubview(self.passwordTextField)
        self.passwordTextField.heightAnchor.constraint(equalToConstant: 45).isActive = true
        self.passwordTextField.centerXAnchor.constraint(equalTo: self.view.centerXAnchor).isActive = true
        self.passwordTextField.topAnchor.constraint(equalTo: self.accountTextField.bottomAnchor, constant: 20).isActive = true
        self.passwordTextField.leadingAnchor.constraint(equalTo: bgView.leadingAnchor, constant: 62).isActive = true
        self.passwordTextField.trailingAnchor.constraint(equalTo: bgView.trailingAnchor, constant: -62).isActive = true
        self.passwordTextField.cornerRadius = 6
        self.passwordTextField.layer.masksToBounds = true
        
        bgView.addSubview(self.loginButton)
        self.loginButton.heightAnchor.constraint(equalToConstant: 45).isActive = true
        self.loginButton.topAnchor.constraint(equalTo: self.passwordTextField.bottomAnchor, constant: 20).isActive = true
        self.loginButton.leadingAnchor.constraint(equalTo: bgView.leadingAnchor, constant: 62).isActive = true
        self.loginButton.trailingAnchor.constraint(equalTo: bgView.trailingAnchor, constant: -62).isActive = true
        self.loginButton.bottomAnchor.constraint(equalTo: bgView.bottomAnchor).isActive = true
        self.loginButton.cornerRadius = 6
        self.loginButton.layer.masksToBounds = true
    }
    
    private func initData() {
        self.loginButton.addTarget(self, action: #selector(loginAction), for: .touchUpInside)
    }
    
    @objc private func loginAction() {
        if let account = self.accountTextField.text, let passWord = self.passwordTextField.text {
            SVProgressHUD.show(withStatus: "Login...")
            PJHttpRequest.login(name: account, passWord: passWord, responseBlock: { (isSuccess, user, error) in
                DispatchQueue.main.async(execute: {
                    if isSuccess, let tempUser = user {
                        try? PJKeychainManager.saveSensitiveString(withService: PJKeyCenter.KeychainUserInfoService, sensitiveKey: account, sensitiveString: passWord)
                        
                        PJUserInfoManager.saveUserInfo(userInfo: tempUser)
                        
                        if PJReposManager.shared.hasSavedReposInLocal {
                            self.initRootViewController()
                            SVProgressHUD.dismiss()
                        } else {
                            PJReposManager.getRepos(completedHandle: { (isSuccess, repos, error) in
                                if isSuccess {
                                    DispatchQueue.main.async(execute: {
                                        self.initRootViewController()
                                        SVProgressHUD.dismiss()
                                    })
                                } else {
                                    //Didn't create repos
                                    if let errorCode = error?.errorCode, PJHttpReponseStatusCode(rawValue: errorCode) == PJHttpReponseStatusCode.HTTP_STATUS_NOT_FOUND {
                                        PJReposManager.createRepos(completedHandle: { (isSuccess, repos, error) in
                                            if isSuccess {
                                                DispatchQueue.main.async(execute: {
                                                    self.initRootViewController()
                                                    SVProgressHUD.dismiss()
                                                })
                                            } else {
                                                DispatchQueue.main.async(execute: {
                                                    SVProgressHUD.showError(withStatus: "Login error when init data!")
                                                })
                                            }
                                        })
                                    } else {
                                        SVProgressHUD.showError(withStatus: "Login fail!")
                                    }
                                }
                            })
                        }
                    } else {
                        SVProgressHUD.showError(withStatus: "Login fail!")
                    }
                })
            })
        } else {
            SVProgressHUD.showError(withStatus: "account and password both can't be nil!")
        }
    }
    
    private func createRepos(completeHandle: ((Bool, Repos?, PJHttpError?) -> ())?) {
        PJHttpRequest.getGitHubRepos(reposUrl: PJHttpUrlConst.GetReposUrl) { (isSuccess, repos, error) in
            //Has created repos
            if isSuccess {
                if let tempRepos = repos {
                    PJReposManager.saveRepos(repos: tempRepos)
                }
            } else {
                //Didn't create repos
                if let errorCode = error?.errorCode, PJHttpReponseStatusCode(rawValue: errorCode) == PJHttpReponseStatusCode.HTTP_STATUS_NOT_FOUND {
                    
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
}
