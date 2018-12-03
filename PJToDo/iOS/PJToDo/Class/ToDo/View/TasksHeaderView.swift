//
//  TasksHeaderView.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/12/2.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

class TasksHeaderView: UITableViewHeaderFooterView {
    static let TasksHeaderViewId = "TasksHeaderView"
//    let titleLabel: UILabel = UILabel()
//
//    override init(frame: CGRect) {
//        super.init(frame: frame)
//        self.initView()
//    }
//
//    required init?(coder aDecoder: NSCoder) {
//        super.init(coder: aDecoder)
//    }
    
    static func initWith(tableView: UITableView) -> TasksHeaderView {
        if let headerView = tableView.dequeueReusableHeaderFooterView(withIdentifier: TasksHeaderView.TasksHeaderViewId), let header = headerView as? TasksHeaderView {
            return header
        } else {
            let headerView = TasksHeaderView(reuseIdentifier: TasksHeaderView.TasksHeaderViewId)
            return headerView
        }
    }
    
//    private func initView() {
//        self.addSubview(titleLabel)
//        titleLabel.translatesAutoresizingMaskIntoConstraints = false
//        titleLabel.leadingAnchor.constraint(equalTo: self.leadingAnchor, constant: 15).isActive = true
//        titleLabel.trailingAnchor.constraint(equalTo: self.trailingAnchor, constant: -15).isActive = true
//        titleLabel.topAnchor.constraint(equalTo: self.topAnchor, constant: 10).isActive = true
//        titleLabel.bottomAnchor.constraint(equalTo: self.bottomAnchor, constant: -10).isActive = true
//    }
    
    func setTitle(title: String?) {
        self.textLabel?.text = title
    }
    
    func setTitleColor(color: UIColor) {
        self.textLabel?.textColor = color
    }
}
