//
//  TextViewController.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/12/8.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit
import SVProgressHUD

enum TextType {
    case type
    case tag
}

class TextViewController: PJBaseViewController {

    private lazy var tableView: UITableView = {
        let tempTableView = UITableView(frame: .zero, style: .plain)
        tempTableView.translatesAutoresizingMaskIntoConstraints = false
        tempTableView.backgroundColor = UIColor.colorWithRGB(red: 249, green: 249, blue: 249)
        tempTableView.estimatedRowHeight = 44.0
        tempTableView.rowHeight = UITableViewAutomaticDimension
        tempTableView.tableFooterView = UIView()
        tempTableView.keyboardDismissMode = .onDrag
        tempTableView.sectionIndexBackgroundColor = UIColor.clear
        tempTableView.tableFooterView?.backgroundColor = .white
        tempTableView.cellLayoutMarginsFollowReadableWidth = false
        return tempTableView
    }()
    
    lazy var typeController: ToDoTypeController = {
        let controller = ToDoTypeController(delegate: self)
        return controller
    }()
    
    lazy var tagController: ToDoTagController = {
        let controller = ToDoTagController(delegate: self)
        return controller
    }()
    
    var textType: TextType = .type
    
    static let TextCellId = "TextCellId"
    
    convenience init(textType: TextType) {
        self.init()
        self.textType = textType
    }
    
    override func viewDidLoad() {
        super.viewDidLoad()
        self.initView()
        self.initData()
    }
    
    private func initView() {
        self.title = self.textType == .type ? "Type" : "Tag"
        self.view.addSubview(self.tableView)
        self.tableView.leadingAnchor.constraint(equalTo: self.view.leadingAnchor).isActive = true
        self.tableView.trailingAnchor.constraint(equalTo: self.view.trailingAnchor).isActive = true
        if #available(iOS 11.0, *) {
            self.tableView.topAnchor.constraint(equalTo: self.view.safeAreaLayoutGuide.topAnchor).isActive = true
        } else {
            self.tableView.topAnchor.constraint(equalTo: self.topLayoutGuide.bottomAnchor).isActive = true
        }
        self.tableView.bottomAnchor.constraint(equalTo: self.view.bottomAnchor).isActive = true
        
        if #available(iOS 11.0, *) {
            self.tableView.contentInsetAdjustmentBehavior = .never
        } else {
            self.automaticallyAdjustsScrollViewInsets = false
        }
        
        self.tableView.register(TextCell.classForCoder(), forCellReuseIdentifier: TextViewController.TextCellId)
        self.tableView.delegate = self
        self.tableView.dataSource = self
        
        let addBarButton = UIBarButtonItem(title: "Add", style: UIBarButtonItemStyle.plain, target: self, action: #selector(addAction))
        self.navigationItem.rightBarButtonItem = addBarButton
    }
    
    private func initData() {
        if self.textType == .type {
            self.typeController.fetchData()
        } else {
            self.tagController.fetchData()
        }
    }
    
    @objc private func addAction() {
        let addAlert = UIAlertController(title: self.textType == .type ? "Add Type" : "Add Tag", message: "", preferredStyle: UIAlertControllerStyle.alert)
        addAlert.addTextField { (textField) in
            
        }
        
        let cancelAction = UIAlertAction(title: "Cancel", style: UIAlertActionStyle.cancel) { (action) in
            
        }
        
        let okAction = UIAlertAction(title: "OK", style: .default) { (action) in
            let text = addAlert.textFields?.first?.text ?? "no name"
            if self.textType == .type {
                self.typeController.insert(toDoType: PJToDoType(typeName: text))
            } else {
                self.tagController.insert(toDoTag: PJToDoTag(insertTagName: text))
            }
        }
        
        addAlert.addAction(cancelAction)
        addAlert.addAction(okAction)
        self.present(addAlert, animated: true, completion: nil)
    }
    
    private func getCount() -> Int {
        if self.textType == .type {
            return Int(self.typeController.getCount())
        } else {
            return Int(self.tagController.getCount())
        }
    }
    
    private func item(at index: Int) -> TextItem {
        if self.textType == .type {
            let type = self.typeController.toDoTypeAt(index: Int32(index))
            let item = TextItem(id: Int(type.typeId), text: type.typeName)
            return item
        } else {
            let tag = self.tagController.toDoTagAt(index: Int32(index))
            let item = TextItem(id: Int(tag.tagId), text: tag.tagName)
            return item
        }
    }
}

extension TextViewController: UITableViewDelegate, UITableViewDataSource {
    
    func tableView(_ tableView: UITableView, numberOfRowsInSection section: Int) -> Int {
        return self.getCount()
    }
    
    func tableView(_ tableView: UITableView, cellForRowAt indexPath: IndexPath) -> UITableViewCell {
        let cell = tableView.dequeueReusableCell(withIdentifier: TextViewController.TextCellId)
        if let tempCell = cell as? TextCell {
            tempCell.textItem = self.item(at: indexPath.row)
            return tempCell
        }
        return UITableViewCell()
    }
    
    func tableView(_ tableView: UITableView, didSelectRowAt indexPath: IndexPath) {
        tableView.deselectRow(at: indexPath, animated: true)
    }
}

extension TextViewController: ToDoTypeDelegate {
    func fetchTypeDataResult(isSuccess: Bool) {
        DispatchQueue.main.async {
            if isSuccess {
                if self.getCount() == 0 {
                    self.showEmpty(show: true)
                } else {
                    self.showEmpty(show: false)
                    self.tableView.reloadData()
                }
            } else {
                SVProgressHUD.showError(withStatus: "Fetch Type Data Error!")
            }
        }
    }
    
    func findTypeByNameResult(toDoType: PJToDoType?, isSuccess: Bool) {
        if isSuccess {
            print("typeName: \(String(describing: toDoType?.typeName))")
        }
    }
    
    func findTypeByIdResult(toDoType: PJToDoType?, isSuccess: Bool) {
        if isSuccess {
            print("typeName: \(String(describing: toDoType?.typeName))")
        }
    }
    
    func updateTypeResult(isSuccess: Bool) {
        if isSuccess {
            
        }
    }
    
    func deleteTypeResult(isSuccess: Bool) {
        if isSuccess {
            
        }
    }
    
    func insertTypeResult(isSuccess: Bool) {
        if isSuccess {
            self.typeController.fetchData()
        } else {
            SVProgressHUD.showError(withStatus: "Add Type Error!")
        }
    }
}

extension TextViewController: ToDoTagDelegate {
    func findTagByIdResult(toDoTag: PJToDoTag?, isSuccess: Bool) {
        
    }
    
    func findTagByNameResult(toDoTag: PJToDoTag?, isSuccess: Bool) {
        
    }
    
    func fetchTagDataResult(isSuccess: Bool) {
        DispatchQueue.main.async {
            if isSuccess {
                if self.getCount() == 0 {
                    self.showEmpty(show: true)
                } else {
                    self.showEmpty(show: false)
                    self.tableView.reloadData()
                }
            } else {
                SVProgressHUD.showError(withStatus: "Fetch Tag Data Error!")
            }
        }
    }
    
    func findTagByNameResult(toDoTag: PJToDoTag, isSuccess: Bool) {
        if isSuccess {
            
        }
    }
    
    func findTagByIdResult(toDoTag: PJToDoTag, isSuccess: Bool) {
        if isSuccess {
            
        }
    }
    
    func updateTagResult(isSuccess: Bool) {
        if isSuccess {
            
        }
    }
    
    func deleteTagResult(isSuccess: Bool) {
        if isSuccess {
            
        }
    }
    
    func insertTagResult(isSuccess: Bool) {
        if isSuccess {
            self.tagController.fetchData()
        } else {
            SVProgressHUD.showError(withStatus: "Add Tag Error!")
        }
    }
}

