//
//  MineViewController.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/11/23.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit
import SVProgressHUD

class MineViewController: PJBaseViewController {

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
    
    var items: [[MineItem]] = []
    
    static let MineCellId = "MineCellId"
    
    lazy var mineController: MineController = {
        let mineController = MineController(delegate: self)
        return mineController
    }()
    
    var mySettings: PJMySettings?
    
    override func viewDidLoad() {
        super.viewDidLoad()
        self.initView()
        self.initData()
    }

    override func didReceiveMemoryWarning() {
        super.didReceiveMemoryWarning()
        // Dispose of any resources that can be recreated.
    }
    
    private func initView() {
        self.title = "Mine"
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
    }
    
    private func initData() {
        let typeItem = MineItem(imageName: "type_setting", title: "分类", detailText: "", type: .type)
        let tagItem = MineItem(imageName: "tag_setting", title: "标签", detailText: "", type: .tag)
        self.items.append([typeItem, tagItem])
        
        let emailItem = MineItem(imageName: "mailbox", title: "邮箱", detailText: "", type: .email)
        let remindItem = MineItem(imageName: "calendar", title: "提醒天数", detailText: "", type: .remindDays)
        self.items.append([emailItem, remindItem])
        
        let blogItem = MineItem(imageName: "blog", title: "博客", detailText: "", type: .blog)
        let aboutItem = MineItem(imageName: "about", title: "关于", detailText: "", type: .about)
        self.items.append([blogItem, aboutItem])
        
        self.tableView.register(MineCell.classForCoder(), forCellReuseIdentifier: MineViewController.MineCellId)
        self.tableView.delegate = self
        self.tableView.dataSource = self
        
        self.mineController.fetchData()
    }
}

extension MineViewController: UITableViewDelegate, UITableViewDataSource {
    
    func numberOfSections(in tableView: UITableView) -> Int {
        return self.items.count
    }
    
    func tableView(_ tableView: UITableView, numberOfRowsInSection section: Int) -> Int {
        return self.items[section].count
    }
    
    func tableView(_ tableView: UITableView, cellForRowAt indexPath: IndexPath) -> UITableViewCell {
        let cell = tableView.dequeueReusableCell(withIdentifier: MineViewController.MineCellId)
        if let tempCell = cell as? MineCell {
            tempCell.item = self.items[indexPath.section][indexPath.row]
            return tempCell
        }
        return UITableViewCell()
    }
    
    func tableView(_ tableView: UITableView, viewForHeaderInSection section: Int) -> UIView? {
        if section == 0 {
            let headerView = MineHeaderView()
            return headerView
        }
        return nil
    }
    
    func tableView(_ tableView: UITableView, heightForHeaderInSection section: Int) -> CGFloat {
        if section == 0 {
            return UITableViewAutomaticDimension
        }
        return CGFloat.leastNormalMagnitude
    }
    
    func tableView(_ tableView: UITableView, viewForFooterInSection section: Int) -> UIView? {
        var footerView: UIView = UIView()
        if section == self.items.count - 1 {
            footerView = MineFooterView()
            return footerView
        }
        footerView.backgroundColor = UIColor.colorWithRGB(red: 249, green: 249, blue: 249)
        return footerView
    }
    
    func tableView(_ tableView: UITableView, heightForFooterInSection section: Int) -> CGFloat {
        if section == self.items.count - 1 {
            return 64
        }
        return 10
    }
    
    func tableView(_ tableView: UITableView, didSelectRowAt indexPath: IndexPath) {
        tableView.deselectRow(at: indexPath, animated: true)
        let item = self.items[indexPath.section][indexPath.row]
        switch item.type {
        case .type:
            let textViewController = TextViewController(textType: .type)
            textViewController.hidesBottomBarWhenPushed = true
            self.navigationController?.pushViewController(textViewController, animated: true)
        case .tag:
            let textViewController = TextViewController(textType: .tag)
            textViewController.hidesBottomBarWhenPushed = true
            self.navigationController?.pushViewController(textViewController, animated: true)
        case .email, .remindDays:
            self.editEmailOrRemindDays(item: item)
        case .blog:
            break
        case .about:
            break
        }
    }
    
    private func editEmailOrRemindDays(item: MineItem) {
        let editAlert = UIAlertController(title: "Set Text", message: "", preferredStyle: UIAlertControllerStyle.alert)
        editAlert.addTextField { (textField) in
            if item.type == .remindDays {
                textField.keyboardType = .numberPad
            } else {
                textField.keyboardType = .emailAddress
            }
        }
        
        let cancelAction = UIAlertAction(title: "Cancel", style: UIAlertActionStyle.cancel) { (action) in
            
        }
        
        let okAction = UIAlertAction(title: "OK", style: .default) { (action) in
            let text = editAlert.textFields?.first?.text ?? ""
            let id = self.mySettings?.settingsId ?? -1
            if id < 0{
                if item.type == .email {
                    let settings = PJMySettings(remindEmail: text, remindDays: 0)
                    self.mineController.insert(toDoSettings: settings)
                } else {
                    let settings = PJMySettings(remindEmail: "", remindDays: Int32(text) ?? 0)
                    self.mineController.insert(toDoSettings: settings)
                }
            } else {
                if let tempSettings = self.mySettings {
                    if item.type == .email {
                        let settings = PJMySettings(settingsId: tempSettings.settingsId, remindEmail: text, remindDays: tempSettings.remindDays)
                        self.mineController.update(toDoSettings: settings)
                    } else {
                        let settings = PJMySettings(settingsId: tempSettings.settingsId, remindEmail: tempSettings.remindEmail, remindDays: Int32(text) ?? 0)
                        self.mineController.update(toDoSettings: settings)
                    }
                } else {
                    SVProgressHUD.showError(withStatus: "Update email or remind days error!")
                }
            }
        }
        
        editAlert.addAction(cancelAction)
        editAlert.addAction(okAction)
        self.present(editAlert, animated: true, completion: nil)
    }
}

extension MineViewController: ToDoSettingsDelegate {
    func insertSettingsResult(isSuccess: Bool) {
        if isSuccess {
            self.mineController.fetchData()
        }
    }
    
    func deleteSettingsResult(isSuccess: Bool) {
        
    }
    
    func updateSettingsResult(isSuccess: Bool) {
        if isSuccess {
            self.mineController.fetchData()
        }
    }
    
    func fetchSettingsDataResult(mySettings: PJMySettings?, isSuccess: Bool) {
        if isSuccess, let settings = mySettings {
            self.mySettings = mySettings
            let tempItems = self.items[1]
            for item in tempItems {
                if item.type == .email {
                    item.detailText = settings.remindEmail
                } else {
                    item.detailText = "\(settings.remindDays)"
                }
            }
            DispatchQueue.main.async {
                self.tableView.reloadSections(IndexSet(integer: 1), with: .none)
            }
        }
    }
}
