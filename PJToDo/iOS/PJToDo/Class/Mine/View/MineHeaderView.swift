//
//  MineHeaderView.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/12/6.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

class MineHeaderView: UIView {

    lazy var avatarImageView: UIImageView = {
       let avatarImageView = UIImageView()
        avatarImageView.translatesAutoresizingMaskIntoConstraints = false
        avatarImageView.backgroundColor = UIColor.colorWithRGB(red: 216, green: 216, blue: 216)
        return avatarImageView
    }()
    
    lazy var nickNameLabel: UILabel = {
        let nickNameLabel = UILabel()
        nickNameLabel.translatesAutoresizingMaskIntoConstraints = false
        nickNameLabel.font = UIFont.systemFont(ofSize: 20, weight: UIFont.Weight.regular)
        nickNameLabel.textAlignment = .center
        nickNameLabel.text = "飘金"
        return nickNameLabel
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
        self.addSubview(self.avatarImageView)
        self.avatarImageView.heightAnchor.constraint(equalToConstant: 64).isActive = true
        self.avatarImageView.widthAnchor.constraint(equalToConstant: 64).isActive = true
        self.avatarImageView.topAnchor.constraint(equalTo: self.topAnchor, constant: 17).isActive = true
        self.avatarImageView.centerXAnchor.constraint(equalTo: self.centerXAnchor).isActive = true
        self.avatarImageView.layer.cornerRadius = 32
        self.avatarImageView.layer.masksToBounds = true
        
        self.addSubview(self.nickNameLabel)
        self.nickNameLabel.leadingAnchor.constraint(equalTo: self.leadingAnchor).isActive = true
        self.nickNameLabel.trailingAnchor.constraint(equalTo: self.trailingAnchor).isActive = true
        self.nickNameLabel.topAnchor.constraint(equalTo: self.avatarImageView.bottomAnchor, constant: 5).isActive = true
        self.nickNameLabel.bottomAnchor.constraint(equalTo: self.bottomAnchor, constant: -5).isActive = true
    }
}
