//
//  DetailFooterView.swift
//  PJToDo
//
//  Created by piaojin on 2019/1/3.
//  Copyright © 2019 piaojin. All rights reserved.
//

import UIKit

class DetailFooterView: UIView {

    lazy var deleteButton: UIButton = {
        let deleteButton = UIButton()
        deleteButton.translatesAutoresizingMaskIntoConstraints = false
        deleteButton.titleLabel?.font = UIFont.systemFont(ofSize: 16, weight: UIFont.Weight.regular)
        deleteButton.titleLabel?.textAlignment = .center
        deleteButton.setTitleColor(.red, for: .normal)
        deleteButton.setTitle("删除", for: .normal)
        deleteButton.backgroundColor = .white
        return deleteButton
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
        self.addSubview(self.deleteButton)
        self.deleteButton.leadingAnchor.constraint(equalTo: self.leadingAnchor).isActive = true
        self.deleteButton.trailingAnchor.constraint(equalTo: self.trailingAnchor).isActive = true
        self.deleteButton.topAnchor.constraint(equalTo: self.topAnchor, constant: 20).isActive = true
        self.deleteButton.bottomAnchor.constraint(equalTo: self.bottomAnchor).isActive = true
    }

}
