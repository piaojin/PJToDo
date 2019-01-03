//
//  DetailTextCell.swift
//  PJToDo
//
//  Created by piaojin on 2019/1/2.
//  Copyright © 2019 piaojin. All rights reserved.
//

import UIKit

class DetailTextCell: DetailItemCell {
    
    var textField: UITextField = {
        let textField = UITextField()
        textField.translatesAutoresizingMaskIntoConstraints = false
        textField.borderStyle = .none
        return textField
    }()
    
    override var item: DetailItem {
        didSet {
            self.textLabel?.text = item.type == .title ? "Title:" : "Content:"
            self.textField.text = item.detailText
        }
    }
    
    required override public init(style: UITableViewCellStyle, reuseIdentifier: String?) {
        super.init(style: style, reuseIdentifier: reuseIdentifier)
        self.initView()
    }

    required public init?(coder aDecoder: NSCoder) {
        super.init(coder: aDecoder)
    }
    
    private func initView() {
        self.contentView.addSubview(self.textField)
        self.textField.heightAnchor.constraint(equalToConstant: 40).isActive = true
        self.textField.topAnchor.constraint(equalTo: self.contentView.topAnchor).isActive = true
        self.textField.leadingAnchor.constraint(equalTo: self.contentView.leadingAnchor, constant: 65).isActive = true
        self.textField.bottomAnchor.constraint(equalTo: self.contentView.bottomAnchor).isActive = true
        self.textField.trailingAnchor.constraint(equalTo: self.contentView.trailingAnchor, constant: -5).isActive = true
    }
}
