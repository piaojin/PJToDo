//
//  MineFooterView.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/12/6.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

class MineFooterView: UIView {
    
    lazy var loginOutLabel: UILabel = {
        let loginOutLabel = UILabel()
        loginOutLabel.translatesAutoresizingMaskIntoConstraints = false
        loginOutLabel.font = UIFont.systemFont(ofSize: 16, weight: UIFont.Weight.regular)
        loginOutLabel.textAlignment = .center
        loginOutLabel.textColor = .red
        loginOutLabel.text = "退出登录"
        loginOutLabel.backgroundColor = .white
        return loginOutLabel
    }()
    
    override init(frame: CGRect) {
        super.init(frame: .zero)
        self.initView()
    }
    
    required init?(coder aDecoder: NSCoder) {
        super.init(coder: aDecoder)
    }
    
    convenience init() {
        self.init(frame: .zero)
    }
    
    private func initView() {
        self.addSubview(self.loginOutLabel)
        self.loginOutLabel.leadingAnchor.constraint(equalTo: self.leadingAnchor).isActive = true
        self.loginOutLabel.trailingAnchor.constraint(equalTo: self.trailingAnchor).isActive = true
        self.loginOutLabel.topAnchor.constraint(equalTo: self.topAnchor, constant: 20).isActive = true
        self.loginOutLabel.bottomAnchor.constraint(equalTo: self.bottomAnchor).isActive = true
    }
}
