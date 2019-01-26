//
//  MineFooterView.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/12/6.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

class MineFooterView: UIView {
    
    var logOutClosure: (() -> ())?
    
    lazy var loginOutButton: UIButton = {
        let loginOutButton = UIButton()
        loginOutButton.translatesAutoresizingMaskIntoConstraints = false
        loginOutButton.titleLabel?.font = UIFont.systemFont(ofSize: 16, weight: UIFont.Weight.regular)
        loginOutButton.titleLabel?.textAlignment = .center
        loginOutButton.setTitleColor(.red, for: .normal)
        loginOutButton.setTitle("LogOut", for: .normal)
        loginOutButton.backgroundColor = .white
        return loginOutButton
    }()
    
    override init(frame: CGRect) {
        super.init(frame: .zero)
        self.initView()
        self.initData()
    }
    
    required init?(coder aDecoder: NSCoder) {
        super.init(coder: aDecoder)
    }
    
    convenience init() {
        self.init(frame: .zero)
    }
    
    private func initView() {
        self.addSubview(self.loginOutButton)
        self.loginOutButton.leadingAnchor.constraint(equalTo: self.leadingAnchor).isActive = true
        self.loginOutButton.trailingAnchor.constraint(equalTo: self.trailingAnchor).isActive = true
        self.loginOutButton.topAnchor.constraint(equalTo: self.topAnchor, constant: 20).isActive = true
        self.loginOutButton.bottomAnchor.constraint(equalTo: self.bottomAnchor).isActive = true
    }
    
    private func initData() {
        self.loginOutButton.addTarget(self, action: #selector(loginOutAction), for: .touchUpInside)
    }
    
    @objc private func loginOutAction() {
        self.logOutClosure?()
    }
}
