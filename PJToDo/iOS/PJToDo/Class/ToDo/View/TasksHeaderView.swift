//
//  TasksHeaderView.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/12/2.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

class TasksHeaderView: UIView {
    
    var titleLabel: UILabel = {
        let titleLabel = UILabel()
        titleLabel.translatesAutoresizingMaskIntoConstraints = false
        titleLabel.font = UIFont.systemFont(ofSize: 18, weight: .bold)
        return titleLabel
    }()
    
    var title: String = "" {
        didSet {
            self.titleLabel.text = title
        }
    }
    
    init() {
        super.init(frame: .zero)
        self.initView()
    }
    
    required init?(coder aDecoder: NSCoder) {
        super.init(coder: aDecoder)
    }
    
    private func initView() {
        self.backgroundColor = UIColor.colorWithRGB(red: 242, green: 242, blue: 242)
        self.addSubview(self.titleLabel)
        self.titleLabel.topAnchor.constraint(equalTo: self.topAnchor, constant: 10).isActive = true
        self.titleLabel.leadingAnchor.constraint(equalTo: self.leadingAnchor, constant: 15).isActive = true
        self.titleLabel.bottomAnchor.constraint(equalTo: self.bottomAnchor, constant: -10).isActive = true
        self.titleLabel.trailingAnchor.constraint(equalTo: self.trailingAnchor, constant: -15).isActive = true
    }
    
    func setTitleColor(color: UIColor) {
        self.titleLabel.textColor = color
    }
}
