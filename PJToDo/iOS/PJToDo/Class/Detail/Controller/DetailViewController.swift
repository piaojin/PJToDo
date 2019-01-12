//
//  DetailViewController.swift
//  PJToDo
//
//  Created by piaojin on 2019/1/3.
//  Copyright © 2019 piaojin. All rights reserved.
//

import UIKit
import SVProgressHUD
import PJPresentation

enum DetailSectionType: Int {
    case titleSection
    case typeSection
    case timeSection
    case prioritySection
}

protocol DetailViewControllerDelegate {
    func didUpdateToDo(detailViewController: DetailViewController, isSuccess: Bool)
    func didDeleteToDo(detailViewController: DetailViewController, isSuccess: Bool)
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
    
    var currentSelectDateType: DetailItemType = .remindTime
    
    static let DetailTextCellId = "DetailTextCellId"
    static let DetailValueCellId = "DetailValueCellId"
    static let DetailPriorityCellId = "DetailPriorityCellId"
    
    var delegate: DetailViewControllerDelegate?
    
    lazy var toDoController: ToDoController = {
        let controller = ToDoController(delegate: self)
        return controller
    }()
    
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
        
        let typeItem = DetailItem(imageName: "type_setting", title: "分类", detailText: self.toDo.toDoType.typeName, type: .type, value: Int(self.toDo.toDoTypeId))
        let tagItem = DetailItem(imageName: "tag_setting", title: "标签", detailText: self.toDo.toDoTag.tagName, type: .tag, value: Int(self.toDo.toDoTagId))
        self.items.append([typeItem, tagItem])
        
        let dueTimeItem = DetailItem(imageName: "calendar", title: "到期日", detailText: self.toDo.dueTime, type: .dueTime)
        let remindTimeItem = DetailItem(imageName: "clock", title: "提醒日", detailText: self.toDo.remindTime, type: .remindTime)
        self.items.append([dueTimeItem, remindTimeItem])
        
        let imageName = "priority_\(self.toDo.priority)"
        let priorityTimeItem = DetailItem(imageName: imageName, title: "优先级", detailText: "\(self.toDo.priority)", type: .priority, value: Int(self.toDo.priority))
        self.items.append([priorityTimeItem])
        
        self.tableView.register(DetailTextCell.classForCoder(), forCellReuseIdentifier: DetailViewController.DetailTextCellId)
        self.tableView.register(DetailValueCell.classForCoder(), forCellReuseIdentifier: DetailViewController.DetailValueCellId)
        self.tableView.register(DetailPriorityCell.classForCoder(), forCellReuseIdentifier: DetailViewController.DetailPriorityCellId)
        
        self.tableView.delegate = self
        self.tableView.dataSource = self
    }
    
    @objc private func saveAction() {
        self.updateToDoModel()
        self.toDoController.update(toDo: self.toDo)
    }
    
    private func updateToDoModel() {
        for sectionItems in self.items {
            for item in sectionItems {
                self.saveValueToModel(item: item)
            }
        }
    }
    
    private func saveValueToModel(item: DetailItem) {
        switch item.type {
            case .title:
                self.toDo.title = item.detailText
            case .content:
                self.toDo.content = item.detailText
            case .type:
                self.toDo.toDoTypeId = Int32(item.value)
            case .tag:
                self.toDo.toDoTagId = Int32(item.value)
            case .remindTime:
                self.toDo.remindTime = item.detailText
            case .dueTime:
                self.toDo.dueTime = item.detailText
            case .priority:
                self.toDo.priority = Int32(item.value)
        }
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
        let item: DetailItem = self.items[indexPath.section][indexPath.row]
        switch item.type {
            case .type:
                let textViewController = TextViewController(textType: .type)
                textViewController.didSelectItemBlock = { (_, textItem) in
                    item.detailText = textItem.text
                    item.value = textItem.id
                    tableView.reloadRows(at: [indexPath], with: .automatic)
                }
                self.navigationController?.pushViewController(textViewController, animated: true)
            case .tag:
                let textViewController = TextViewController(textType: .tag)
                textViewController.didSelectItemBlock = { (_, textItem) in
                    item.detailText = textItem.text
                    item.value = textItem.id
                    tableView.reloadRows(at: [indexPath], with: .automatic)
                }
                self.navigationController?.pushViewController(textViewController, animated: true)
            case .remindTime:
                self.showSelectDate(defaultDateString: item.detailText) { (dateString) in
                    item.detailText = dateString
                    tableView.reloadRows(at: [indexPath], with: .automatic)
                }
            case .dueTime:
                self.showSelectDate(defaultDateString: item.detailText) { (dateString) in
                    item.detailText = dateString
                    tableView.reloadRows(at: [indexPath], with: .automatic)
                }
            default:
                break
        }
    }
    
    private func showSelectDate(defaultDateString: String, didSelectBlock: @escaping (_ dateString: String) -> ()) {
        let dateSelectView = DateSelectView()
        dateSelectView.setDate(defaultDateString, animated: true)
        
        var options = PJPresentationOptions()
        options.dismissDirection = .topToBottom
        let presentationViewController = PJPresentationControllerManager.presentView(contentView: dateSelectView, presentationViewControllerHeight: 250, presentationOptions: options)
        presentationViewController.dismissClosure = {
            didSelectBlock(dateSelectView.dateString)
        }
        
        dateSelectView.doneBlock = { (dateString) in
            PJPresentationControllerManager.dismiss(presentationViewController: presentationViewController, animated: true, completion: {
                didSelectBlock(dateSelectView.dateString)
            })
        }
    }
    
    func tableView(_ tableView: UITableView, viewForFooterInSection section: Int) -> UIView? {
        if let sectionType = DetailSectionType(rawValue: section) {
            if sectionType == .prioritySection {
                let footerView: DetailFooterView = DetailFooterView()
                footerView.deleteToDoBlock = {
                    self.toDoController.delete(toDoId: Int(self.toDo.toDoId))
                }
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

extension DetailViewController: ToDoDelegate {
    func fetchToDoDataResult(isSuccess: Bool) {
        
    }
    
    func updateToDoResult(isSuccess: Bool) {
        DispatchQueue.main.async {
            if isSuccess {
                self.delegate?.didUpdateToDo(detailViewController: self, isSuccess: isSuccess)
                SVProgressHUD.showSuccess(withStatus: "更新成功!")
                self.navigationController?.popViewController(animated: true)
            } else {
                SVProgressHUD.showError(withStatus: "更新失败!")
            }
        }
    }
    
    func deleteToDoResult(isSuccess: Bool) {
        DispatchQueue.main.async {
            if isSuccess {
                self.delegate?.didDeleteToDo(detailViewController: self, isSuccess: isSuccess)
                SVProgressHUD.showSuccess(withStatus: "删除成功!")
                self.navigationController?.popViewController(animated: true)
            } else {
                SVProgressHUD.showError(withStatus: "删除失败!")
            }
        }
    }
}
