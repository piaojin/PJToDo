//
//  PJInputBox.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/12/5.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

enum ActionType {
    case remind
    case type
    case tag
    case priority
    case send
}

final class PJInputBoxButton: UIButton {
    var actionType: ActionType = .remind
}

typealias PJInputBoxActionClosure = (UIView, ActionType) -> Void

class PJInputBox: UIView {

    lazy var textField: UITextField = {
        let textField = UITextField()
        textField.translatesAutoresizingMaskIntoConstraints = false
        textField.placeholder = "比如,今晚打老虎"
        return textField
    }()

    lazy var remindTimeButton: PJInputBoxButton = {
        let remindTimeButton = PJInputBoxButton()
        remindTimeButton.translatesAutoresizingMaskIntoConstraints = false
        remindTimeButton.setImage(UIImage(named: "calendar"), for: .normal)
        remindTimeButton.actionType = .remind
        return remindTimeButton
    }()
    
    lazy var typeButton: PJInputBoxButton = {
        let typeButton = PJInputBoxButton()
        typeButton.translatesAutoresizingMaskIntoConstraints = false
        typeButton.setImage(UIImage(named: "type_setting"), for: .normal)
        typeButton.actionType = .type
        return typeButton
    }()
    
    lazy var tagButton: PJInputBoxButton = {
        let tagButton = PJInputBoxButton()
        tagButton.translatesAutoresizingMaskIntoConstraints = false
        tagButton.setImage(UIImage(named: "tag_setting"), for: .normal)
        tagButton.actionType = .tag
        return tagButton
    }()
    
    lazy var priorityButton: PJInputBoxButton = {
        let priorityButton = PJInputBoxButton()
        priorityButton.translatesAutoresizingMaskIntoConstraints = false
        priorityButton.setImage(UIImage(named: "priority_5"), for: .normal)
        priorityButton.actionType = .priority
        return priorityButton
    }()
    
    lazy var sendButton: PJInputBoxButton = {
        let sendButton = PJInputBoxButton()
        sendButton.translatesAutoresizingMaskIntoConstraints = false
        sendButton.setImage(UIImage(named: "send"), for: .normal)
        sendButton.imageView?.contentMode = .scaleToFill
        sendButton.backgroundColor = UIColor.colorWithRGB(red: 0, green: 123, blue: 249)
        sendButton.actionType = .send
        return sendButton
    }()
    
    var inputBoxActionClosure: PJInputBoxActionClosure?
    
    init() {
        super.init(frame: .zero)
        self.initView()
        self.initData()
    }
    
    required init?(coder aDecoder: NSCoder) {
        super.init(coder: aDecoder)
    }
    
    private func initView() {
        self.backgroundColor = .white
        self.addSubview(self.textField)
        self.textField.leadingAnchor.constraint(equalTo: self.leadingAnchor, constant: 15).isActive = true
        self.textField.topAnchor.constraint(equalTo: self.topAnchor, constant: 10).isActive = true
        self.textField.trailingAnchor.constraint(equalTo: self.trailingAnchor, constant: -15).isActive = true
        self.textField.heightAnchor.constraint(equalToConstant: 30).isActive = true
        
        self.addSubview(self.remindTimeButton)
        self.remindTimeButton.widthAnchor.constraint(equalToConstant: 17).isActive = true
        self.remindTimeButton.heightAnchor.constraint(equalToConstant: 17).isActive = true
        self.remindTimeButton.leadingAnchor.constraint(equalTo: self.leadingAnchor, constant: 15).isActive = true
        self.remindTimeButton.topAnchor.constraint(equalTo: self.textField.bottomAnchor, constant: 15).isActive = true
        self.remindTimeButton.bottomAnchor.constraint(equalTo: self.bottomAnchor, constant: -10).isActive = true
        
        self.addSubview(self.typeButton)
        self.typeButton.widthAnchor.constraint(equalToConstant: 17).isActive = true
        self.typeButton.heightAnchor.constraint(equalToConstant: 17).isActive = true
        self.typeButton.leadingAnchor.constraint(equalTo: self.remindTimeButton.trailingAnchor, constant: 28).isActive = true
        self.typeButton.centerYAnchor.constraint(equalTo: self.remindTimeButton.centerYAnchor).isActive = true
        
        self.addSubview(self.tagButton)
        self.tagButton.widthAnchor.constraint(equalToConstant: 17).isActive = true
        self.tagButton.heightAnchor.constraint(equalToConstant: 17).isActive = true
        self.tagButton.leadingAnchor.constraint(equalTo: self.typeButton.trailingAnchor, constant: 28).isActive = true
        self.tagButton.centerYAnchor.constraint(equalTo: self.typeButton.centerYAnchor).isActive = true
        
        self.addSubview(self.priorityButton)
        self.priorityButton.widthAnchor.constraint(equalToConstant: 17).isActive = true
        self.priorityButton.heightAnchor.constraint(equalToConstant: 17).isActive = true
        self.priorityButton.leadingAnchor.constraint(equalTo: self.tagButton.trailingAnchor, constant: 28).isActive = true
        self.priorityButton.centerYAnchor.constraint(equalTo: self.tagButton.centerYAnchor).isActive = true
        
        self.addSubview(self.sendButton)
        self.sendButton.widthAnchor.constraint(equalToConstant: 28).isActive = true
        self.sendButton.heightAnchor.constraint(equalToConstant: 28).isActive = true
        self.sendButton.trailingAnchor.constraint(equalTo: self.trailingAnchor, constant: -15).isActive = true
        self.sendButton.centerYAnchor.constraint(equalTo: self.priorityButton.centerYAnchor).isActive = true
        self.sendButton.layer.cornerRadius = 14
        self.sendButton.layer.masksToBounds = true
    }
    
    private func initData() {
        self.remindTimeButton.addTarget(self, action: #selector(clickAction(sender:)), for: .touchUpInside)
        self.typeButton.addTarget(self, action: #selector(clickAction(sender:)), for: .touchUpInside)
        self.tagButton.addTarget(self, action: #selector(clickAction(sender:)), for: .touchUpInside)
        self.priorityButton.addTarget(self, action: #selector(clickAction(sender:)), for: .touchUpInside)
        self.sendButton.addTarget(self, action: #selector(clickAction(sender:)), for: .touchUpInside)
    }
    
    @objc private func clickAction(sender: PJInputBoxButton) {
        self.inputBoxActionClosure?(sender, sender.actionType)
    }
}
