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
    
    static func initWith(tableView: UITableView) -> TasksHeaderView {
        if let headerView = tableView.dequeueReusableHeaderFooterView(withIdentifier: TasksHeaderView.TasksHeaderViewId), let header = headerView as? TasksHeaderView {
            return header
        } else {
            let headerView = TasksHeaderView(reuseIdentifier: TasksHeaderView.TasksHeaderViewId)
            return headerView
        }
    }
    
    func setTitle(title: String?) {
        self.textLabel?.text = title
    }
    
    func setTitleColor(color: UIColor) {
        self.textLabel?.textColor = color
    }
}
