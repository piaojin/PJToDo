//
//  DetailViewController.swift
//  PJToDo
//
//  Created by piaojin on 2019/1/3.
//  Copyright © 2019 piaojin. All rights reserved.
//

import UIKit

enum DetailSectionType: Int {
    case titleSection
    case typeSection
    case timeSection
    case prioritySection
}

class DetailViewController: PJBaseViewController {
    private lazy var tableView: UITableView = {
        let tempTableView = UITableView(frame: .zero, style: .grouped)
        tempTableView.translatesAutoresizingMaskIntoConstraints = false
        tempTableView.backgroundColor = UIColor.colorWithRGB(red: 249, green: 249, blue: 249)
        tempTableView.estimatedRowHeight = 44.0
        tempTableView.rowHeight = UITableViewAutomaticDimension
        tempTableView.estimatedSectionHeaderHeight = UITableViewAutomaticDimension
        tempTableView.estimatedSectionFooterHeight = UITableViewAutomaticDimension
        tempTableView.tableHeaderView = UIView(frame: CGRect(x: 0, y: 0, width: 0, height: CGFloat.leastNormalMagnitude))
        tempTableView.tableFooterView = UIView()
        tempTableView.keyboardDismissMode = .onDrag
        tempTableView.sectionIndexBackgroundColor = UIColor.clear
        tempTableView.tableFooterView?.backgroundColor = .white
        tempTableView.cellLayoutMarginsFollowReadableWidth = false
        return tempTableView
    }()
    
    var toDo: PJ_ToDo = PJ_ToDo()
    
    var items: [[DetailItem]] = []
    
    static let DetailTextCellId = "DetailTextCellId"
    static let DetailValueCellId = "DetailValueCellId"
    static let DetailPriorityCellId = "DetailPriorityCellId"
    
    convenience init(toDo: PJ_ToDo) {
        self.init()
        self.toDo = toDo
    }
    
    override func viewDidLoad() {
        super.viewDidLoad()
        self.initView()
        self.initData()
    }
    
    private func initView() {
        self.title = "详情"
        self.view.addSubview(self.tableView)
        self.tableView.leadingAnchor.constraint(equalTo: self.view.leadingAnchor).isActive = true
        self.tableView.trailingAnchor.constraint(equalTo: self.view.trailingAnchor).isActive = true
        if #available(iOS 11.0, *) {
            self.tableView.topAnchor.constraint(equalTo: self.view.safeAreaLayoutGuide.topAnchor).isActive = true
            self.tableView.bottomAnchor.constraint(equalTo: self.view.safeAreaLayoutGuide.bottomAnchor).isActive = true
        } else {
            self.tableView.topAnchor.constraint(equalTo: self.topLayoutGuide.bottomAnchor).isActive = true
            self.tableView.bottomAnchor.constraint(equalTo: self.bottomLayoutGuide.bottomAnchor).isActive = true
        }
        
        if #available(iOS 11.0, *) {
            self.tableView.contentInsetAdjustmentBehavior = .never
        } else {
            self.automaticallyAdjustsScrollViewInsets = false
        }
        
        self.navigationItem.rightBarButtonItem = UIBarButtonItem(title: "Save", style: UIBarButtonItemStyle.done, target: self, action: #selector(saveAction))
    }
    
    private func initData() {
        
        let titleItem = DetailItem(imageName: "", title: "Title", detailText: self.toDo.title, type: .title)
        let contentItem = DetailItem(imageName: "", title: "Content", detailText: self.toDo.content, type: .content)
        self.items.append([titleItem, contentItem])
        
        let typeItem = DetailItem(imageName: "type_setting", title: "分类", detailText: self.toDo.toDoType.typeName, type: .type)
        let tagItem = DetailItem(imageName: "tag_setting", title: "标签", detailText: self.toDo.toDoTag.tagName, type: .tag)
        self.items.append([typeItem, tagItem])
        
        let dueTimeItem = DetailItem(imageName: "calendar", title: "到期日", detailText: self.toDo.dueTime, type: .dueTime)
        let remindTimeItem = DetailItem(imageName: "clock", title: "提醒日", detailText: self.toDo.remindTime, type: .remindTime)
        self.items.append([dueTimeItem, remindTimeItem])
        
        let imageName = "priority_\(self.toDo.priority)"
        let priorityTimeItem = DetailItem(imageName: imageName, title: "优先级", detailText: "\(self.toDo.priority)", type: .priority)
        self.items.append([priorityTimeItem])
        
        self.tableView.register(DetailTextCell.classForCoder(), forCellReuseIdentifier: DetailViewController.DetailTextCellId)
        self.tableView.register(DetailValueCell.classForCoder(), forCellReuseIdentifier: DetailViewController.DetailValueCellId)
        self.tableView.register(DetailPriorityCell.classForCoder(), forCellReuseIdentifier: DetailViewController.DetailPriorityCellId)
        
        self.tableView.delegate = self
        self.tableView.dataSource = self
    }
    
    @objc private func saveAction() {
        
    }
}

extension DetailViewController: UITableViewDelegate, UITableViewDataSource {
    func tableView(_ tableView: UITableView, cellForRowAt indexPath: IndexPath) -> UITableViewCell {
        var cellId = ""
        if let sectionType = DetailSectionType(rawValue: indexPath.section) {
            switch sectionType {
            case .titleSection:
                cellId = DetailViewController.DetailTextCellId
            case .typeSection:
                cellId = DetailViewController.DetailValueCellId
            case .timeSection:
                cellId = DetailViewController.DetailValueCellId
            case .prioritySection:
                cellId = DetailViewController.DetailPriorityCellId
            }
        }
        let cell = tableView.dequeueReusableCell(withIdentifier: cellId, for: indexPath)
        if let tempCell = cell as? DetailItemCell {
            let item: DetailItem = self.items[indexPath.section][indexPath.row]
            tempCell.item = item
        }
        return cell
    }
    
    func numberOfSections(in tableView: UITableView) -> Int {
        return self.items.count
    }
    
    func tableView(_ tableView: UITableView, numberOfRowsInSection section: Int) -> Int {
        return self.items[section].count
    }
    
    func tableView(_ tableView: UITableView, didSelectRowAt indexPath: IndexPath) {
        tableView.deselectRow(at: indexPath, animated: true)
    }
    
    func tableView(_ tableView: UITableView, viewForFooterInSection section: Int) -> UIView? {
        if let sectionType = DetailSectionType(rawValue: section) {
            if sectionType == .prioritySection {
                let footerView: UIView = DetailFooterView()
                return footerView
            }
        }
        return nil
    }
    
    func tableView(_ tableView: UITableView, heightForFooterInSection section: Int) -> CGFloat {
        if let sectionType = DetailSectionType(rawValue: section) {
            if sectionType == .prioritySection {
                return 64
            }
        }
        return CGFloat.leastNormalMagnitude
    }
}
