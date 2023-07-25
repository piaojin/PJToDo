//
//  SearchToDoCell.swift
//  PJToDo
//
//  Created by piaojin on 2019/1/5.
//  Copyright © 2019 piaojin. All rights reserved.
//

import UIKit

class SearchToDoCell: ToDoCell {

    var stateLabel: UILabel = {
        let stateLabel = UILabel()
        stateLabel.translatesAutoresizingMaskIntoConstraints = false
        stateLabel.font = UIFont.systemFont(ofSize: 14)
        stateLabel.textColor = UIColor.colorWithRGB(red: 75, green: 75, blue: 75)
        return stateLabel
    }()
    
    override var item: PJ_ToDo? {
        didSet {
            if let toDo = item {
                self.titleLabel.text = toDo.title
                self.remindLabel.text = toDo.remindTime
                self.contentLabel.text = toDo.content
                self.typeLabel.text = toDo.toDoType.typeName
                self.tagLabel.text = toDo.toDoTag.tagName
                self.priorityImageView.image = UIImage(named: "priority_\(toDo.priority)")
                self.updateStateLabel(item: toDo)
            }
        }
    }
    
    override init(style: UITableViewCell.CellStyle, reuseIdentifier: String?) {
        super.init(style: style, reuseIdentifier: reuseIdentifier)
        self.initView()
        
    }
    
    required init?(coder aDecoder: NSCoder) {
        super.init(coder: aDecoder)
    }
    
    private func initView() {
        self.contentView.addSubview(self.stateLabel)
        self.stateLabel.trailingAnchor.constraint(equalTo: self.priorityImageView.leadingAnchor, constant: -5).isActive = true
        self.stateLabel.centerYAnchor.constraint(equalTo: self.priorityImageView.centerYAnchor).isActive = true
    }
    
    private func updateStateLabel(item: PJ_ToDo) {
        var stateText: String = ""
        switch item.state {
            case .inProgress:
                stateText = "进行中"
            case .unDetermined:
                stateText = "待定"
            case .overdue:
                stateText = "过期"
            case .completed:
                stateText = "完成"
        }
        self.stateLabel.text = stateText
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
