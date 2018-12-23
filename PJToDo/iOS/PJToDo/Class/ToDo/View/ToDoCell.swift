//
//  ToDoCell.swift
//  PJToDo
//
//  Created by piaojin on 2018/12/22.
//  Copyright © 2018 piaojin. All rights reserved.
//

import UIKit

class ToDoCell: UITableViewCell {

    var titleLabel: UILabel = {
        let titleLabel = UILabel()
        titleLabel.translatesAutoresizingMaskIntoConstraints = false
        return titleLabel
    }()
    
    var contentLabel: UILabel = {
        let contentLabel = UILabel()
        contentLabel.translatesAutoresizingMaskIntoConstraints = false
        contentLabel.font = UIFont.systemFont(ofSize: 15)
        contentLabel.textColor = UIColor.colorWithRGB(red: 188, green: 188, blue: 188)
        return contentLabel
    }()
    
    var remindLabel: UILabel = {
        let remindLabel = UILabel()
        remindLabel.translatesAutoresizingMaskIntoConstraints = false
        remindLabel.font = UIFont.systemFont(ofSize: 15)
        remindLabel.textColor = UIColor.colorWithRGB(red: 75, green: 75, blue: 75)
        remindLabel.textAlignment = .right
        return remindLabel
    }()
    
    var typeLabel: UILabel = {
        let typeLabel = UILabel()
        typeLabel.translatesAutoresizingMaskIntoConstraints = false
        typeLabel.font = UIFont.systemFont(ofSize: 14)
        typeLabel.textColor = UIColor.colorWithRGB(red: 75, green: 75, blue: 75)
        return typeLabel
    }()
    
    var tagLabel: UILabel = {
        let tagLabel = UILabel()
        tagLabel.translatesAutoresizingMaskIntoConstraints = false
        tagLabel.font = UIFont.systemFont(ofSize: 14)
        tagLabel.textColor = UIColor.colorWithRGB(red: 75, green: 75, blue: 75)
        return tagLabel
    }()
    
    var priorityImageView: UIImageView = {
       let priorityImageView = UIImageView()
        priorityImageView.translatesAutoresizingMaskIntoConstraints = false
        return priorityImageView
    }()
    
    var item: PJ_ToDo? {
        didSet {
            if let toDo = item {
                self.titleLabel.text = toDo.title
                self.remindLabel.text = toDo.remindTime
                self.contentLabel.text = toDo.content
                self.typeLabel.text = toDo.toDoType.typeName
                self.tagLabel.text = toDo.toDoTag.tagName
                self.priorityImageView.image = UIImage(named: "priority_\(toDo.priority)")
            }
        }
    }
    
    override init(style: UITableViewCellStyle, reuseIdentifier: String?) {
        super.init(style: style, reuseIdentifier: reuseIdentifier)
        self.initView()
        
    }
    
    required init?(coder aDecoder: NSCoder) {
        super.init(coder: aDecoder)
    }
    
    private func initView() {
        self.contentView.addSubview(self.titleLabel)
        self.titleLabel.topAnchor.constraint(equalTo: self.contentView.topAnchor, constant: 10).isActive = true
        self.titleLabel.leadingAnchor.constraint(equalTo: self.contentView.leadingAnchor, constant: 15).isActive = true
        
        self.contentView.addSubview(self.remindLabel)
        self.remindLabel.trailingAnchor.constraint(equalTo: self.contentView.trailingAnchor, constant: -15).isActive = true
        self.remindLabel.centerYAnchor.constraint(equalTo: self.titleLabel.centerYAnchor).isActive = true
        
        let remindImageView = UIImageView(image: UIImage(named: "clock"))
        remindImageView.translatesAutoresizingMaskIntoConstraints = false
        self.contentView.addSubview(remindImageView)
        remindImageView.widthAnchor.constraint(equalToConstant: 13).isActive = true
        remindImageView.heightAnchor.constraint(equalToConstant: 13).isActive = true
        remindImageView.trailingAnchor.constraint(equalTo: self.remindLabel.leadingAnchor, constant: -3).isActive = true
        remindImageView.centerYAnchor.constraint(equalTo: self.remindLabel.centerYAnchor).isActive = true
        
        self.contentView.addSubview(self.contentLabel)
        self.contentLabel.topAnchor.constraint(equalTo: self.titleLabel.bottomAnchor, constant: 5).isActive = true
        self.contentLabel.leadingAnchor.constraint(equalTo: self.contentView.leadingAnchor, constant: 15).isActive = true
        
        let typeImageView = UIImageView(image: UIImage(named: "type_setting"))
        typeImageView.translatesAutoresizingMaskIntoConstraints = false
        self.contentView.addSubview(typeImageView)
        typeImageView.widthAnchor.constraint(equalToConstant: 13).isActive = true
        typeImageView.heightAnchor.constraint(equalToConstant: 13).isActive = true
        typeImageView.leadingAnchor.constraint(equalTo: self.contentView.leadingAnchor, constant: 15).isActive = true
        typeImageView.topAnchor.constraint(equalTo: self.contentLabel.bottomAnchor, constant: 5).isActive = true
        typeImageView.bottomAnchor.constraint(equalTo: self.contentView.bottomAnchor, constant: -10).isActive = true
        
        self.contentView.addSubview(self.typeLabel)
        self.typeLabel.leadingAnchor.constraint(equalTo: typeImageView.trailingAnchor, constant: 6).isActive = true
        self.typeLabel.centerYAnchor.constraint(equalTo: typeImageView.centerYAnchor).isActive = true
        
        let tagImageView = UIImageView(image: UIImage(named: "tag_setting"))
        tagImageView.translatesAutoresizingMaskIntoConstraints = false
        self.contentView.addSubview(tagImageView)
        tagImageView.widthAnchor.constraint(equalToConstant: 13).isActive = true
        tagImageView.heightAnchor.constraint(equalToConstant: 13).isActive = true
        tagImageView.leadingAnchor.constraint(equalTo: self.typeLabel.trailingAnchor, constant: 10).isActive = true
        tagImageView.centerYAnchor.constraint(equalTo: typeImageView.centerYAnchor).isActive = true
        
        self.contentView.addSubview(self.tagLabel)
        self.tagLabel.leadingAnchor.constraint(equalTo: tagImageView.trailingAnchor, constant: 6).isActive = true
        self.tagLabel.centerYAnchor.constraint(equalTo: tagImageView.centerYAnchor).isActive = true
        
        self.contentView.addSubview(priorityImageView)
        self.priorityImageView.widthAnchor.constraint(equalToConstant: 13).isActive = true
        self.priorityImageView.heightAnchor.constraint(equalToConstant: 13).isActive = true
        self.priorityImageView.centerYAnchor.constraint(equalTo: self.typeLabel.centerYAnchor).isActive = true
        self.priorityImageView.trailingAnchor.constraint(equalTo: self.contentView.trailingAnchor, constant: -15).isActive = true
    }
    
    override func awakeFromNib() {
        super.awakeFromNib()
        // Initialization code
    }

    override func setSelected(_ selected: Bool, animated: Bool) {
        super.setSelected(selected, animated: animated)

        // Configure the view for the selected state
    }

}
