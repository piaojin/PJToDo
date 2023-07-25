//
//  DetailTextCell.swift
//  PJToDo
//
//  Created by piaojin on 2019/1/2.
//  Copyright © 2019 piaojin. All rights reserved.
//

import UIKit

class DetailTextCell: DetailItemCell, UITextFieldDelegate {
    
    var titleLabel: UILabel = {
        let titleLabel = UILabel()
        titleLabel.translatesAutoresizingMaskIntoConstraints = false
        titleLabel.font = UIFont.systemFont(ofSize: 17, weight: .bold)
        return titleLabel
    }()
    
    var textField: UITextField = {
        let textField = UITextField()
        textField.translatesAutoresizingMaskIntoConstraints = false
        textField.borderStyle = .none
        return textField
    }()
    
    override var item: DetailItem {
        didSet {
            self.titleLabel.text = item.type == .title ? "Title" : "Content"
            self.textField.text = item.detailText
        }
    }
    
    required override public init(style: UITableViewCell.CellStyle, reuseIdentifier: String?) {
        super.init(style: style, reuseIdentifier: reuseIdentifier)
        self.initView()
        self.initData()
    }

    required public init?(coder aDecoder: NSCoder) {
        super.init(coder: aDecoder)
    }
    
    private func initView() {
        self.contentView.addSubview(self.titleLabel)
        self.titleLabel.leadingAnchor.constraint(equalTo: self.contentView.leadingAnchor, constant: 15).isActive = true
        self.titleLabel.trailingAnchor.constraint(equalTo: self.contentView.trailingAnchor, constant: -15).isActive = true
        self.titleLabel.topAnchor.constraint(equalTo: self.contentView.topAnchor, constant: 10).isActive = true
        
        self.contentView.addSubview(self.textField)
        self.textField.topAnchor.constraint(equalTo: self.titleLabel.bottomAnchor, constant: 5).isActive = true
        self.textField.leadingAnchor.constraint(equalTo: self.contentView.leadingAnchor, constant: 15).isActive = true
        self.textField.bottomAnchor.constraint(equalTo: self.contentView.bottomAnchor, constant: -10).isActive = true
        self.textField.trailingAnchor.constraint(equalTo: self.contentView.trailingAnchor, constant: -15).isActive = true
    }
    
    private func initData() {
        self.textField.addTarget(self, action: #selector(textFieldValueChanged(sender:)), for: .editingChanged)
//        self.textField.delegate = self
    }
    
    @objc private func textFieldValueChanged(sender: UIView) {
        self.item.detailText = self.textField.text ?? ""
    }
    
//    func textFieldDidBeginEditing(_ textField: UITextField) {
//        self.item.detailText = textField.text ?? ""
//    }
}
