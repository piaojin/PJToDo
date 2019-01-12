//
//  AddToDoViewController.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/11/30.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit
import SVProgressHUD

class AddToDoViewController: PJBaseViewController {
    
    lazy var inputBox: PJInputBox = {
        let inputBox = PJInputBox()
        inputBox.translatesAutoresizingMaskIntoConstraints = false
        return inputBox
    }()
    
    lazy var selectComposeTypeView: SelectComposeTypeView = {
        let selectComposeTypeView = SelectComposeTypeView()
        selectComposeTypeView.translatesAutoresizingMaskIntoConstraints = false
        return selectComposeTypeView
    }()
    
    lazy var selectedComposeView: SelectedComposeView = {
        let selectedComposeView = SelectedComposeView()
        selectedComposeView.translatesAutoresizingMaskIntoConstraints = false
        return selectedComposeView
    }()
    
    lazy var datePicker: UIDatePicker = {
        let datePicker = UIDatePicker()
        datePicker.translatesAutoresizingMaskIntoConstraints = false
        return datePicker
    }()
    
    lazy var dateFormatter: DateFormatter = {
        let dateFormatter = DateFormatter()
        dateFormatter.dateFormat = "yyyy-MM-dd h:m"
        return dateFormatter
    }()
    
    lazy var doneView: UIView = {
        let doneView = UIView()
        doneView.isUserInteractionEnabled = true
        doneView.frame = CGRect(x: 0, y: 0, width: 0, height: 40)
        
        let doneButton = UIButton()
        doneButton.setTitle("Done", for: .normal)
        doneButton.setTitleColor(UIColor.colorWithRGB(red: 0, green: 123, blue: 249), for: .normal)
        doneButton.translatesAutoresizingMaskIntoConstraints = false
        doneView.addSubview(doneButton)
        doneButton.topAnchor.constraint(equalTo: doneView.topAnchor).isActive = true
        doneButton.bottomAnchor.constraint(equalTo: doneView.bottomAnchor).isActive = true
        doneButton.trailingAnchor.constraint(equalTo: doneView.trailingAnchor, constant: -5).isActive = true
        doneButton.widthAnchor.constraint(equalToConstant: 60).isActive = true
        doneButton.addTarget(self, action: #selector(doneAction), for: .touchUpInside)
        return doneView
    }()
    
    var selectComposeTypeViewHeightConstraint: NSLayoutConstraint?
    var selectedComposeViewHeightConstraint: NSLayoutConstraint?
    var bottomAnchorLayoutConstraint: NSLayoutConstraint?
    
    static let selectComposeTypeViewHeight: CGFloat = 130
    static let selectedComposeViewHeight: CGFloat = 60
    
    lazy var toDoController: ToDoController = {
        let controller = ToDoController(delegate: self)
        return controller
    }()
    
    lazy var typeController: ToDoTypeController = {
        let controller = ToDoTypeController(delegate: self)
        return controller
    }()
    
    lazy var tagController: ToDoTagController = {
        let controller = ToDoTagController(delegate: self)
        return controller
    }()
    
    var priorityItems: [(Int, String)] = []
    
    override func viewDidLoad() {
        super.viewDidLoad()
        self.initView()
        self.initData()
    }
    
    private func initView() {
        self.title = "New ToDo"
        self.view.backgroundColor = UIColor.colorWithRGB(red: 220, green: 220, blue: 220)
        self.navigationItem.leftBarButtonItem = UIBarButtonItem(title: "Close", style: UIBarButtonItemStyle.done, target: self, action: #selector(closeAction))
        
        self.view.addSubview(self.inputBox)
        self.inputBox.leadingAnchor.constraint(equalTo: self.view.leadingAnchor).isActive = true
        self.inputBox.trailingAnchor.constraint(equalTo: self.view.trailingAnchor).isActive = true
        if #available(iOS 11.0, *) {
            self.bottomAnchorLayoutConstraint = self.inputBox.bottomAnchor.constraint(equalTo: self.view.safeAreaLayoutGuide.bottomAnchor)
            self.bottomAnchorLayoutConstraint?.isActive = true
        } else {
            self.bottomAnchorLayoutConstraint = self.inputBox.bottomAnchor.constraint(equalTo: self.bottomLayoutGuide.bottomAnchor)
            self.bottomAnchorLayoutConstraint?.isActive = true
        }
        
        self.view.addSubview(self.selectedComposeView)
        self.selectedComposeView.leadingAnchor.constraint(equalTo: self.view.leadingAnchor).isActive = true
        self.selectedComposeView.trailingAnchor.constraint(equalTo: self.view.trailingAnchor).isActive = true
        self.selectedComposeView.bottomAnchor.constraint(equalTo: self.inputBox.topAnchor).isActive = true
        self.selectedComposeViewHeightConstraint = self.selectedComposeView.heightAnchor.constraint(equalToConstant: 0)
        self.selectedComposeViewHeightConstraint?.isActive = true
        self.selectedComposeViewHeightConstraint?.constant = 0
        
        self.view.addSubview(self.selectComposeTypeView)
        self.selectComposeTypeView.leadingAnchor.constraint(equalTo: self.view.leadingAnchor).isActive = true
        self.selectComposeTypeView.trailingAnchor.constraint(equalTo: self.view.trailingAnchor).isActive = true
        self.selectComposeTypeView.bottomAnchor.constraint(equalTo: self.selectedComposeView.topAnchor).isActive = true
        self.selectComposeTypeViewHeightConstraint = self.selectComposeTypeView.heightAnchor.constraint(equalToConstant: 0)
        self.selectComposeTypeViewHeightConstraint?.isActive = true
        self.selectComposeTypeViewHeightConstraint?.constant = 0
    }
    
    private func initData() {
        NotificationCenter.default.addObserver(self, selector: #selector(self.keyboardWillShow(note:)), name: NSNotification.Name.UIKeyboardWillShow, object: nil)
        NotificationCenter.default.addObserver(self, selector: #selector(self.keyboardDidHidden(note:)), name: NSNotification.Name.UIKeyboardDidHide, object: nil)
        
        self.selectComposeTypeView.delegate = self
        self.inputBox.textField.becomeFirstResponder()
        
        self.inputBox.inputBoxActionClosure = { [weak self] (box, type) in
            switch type {
                case ActionType.type:
                    self?.showSelectComposeTypeView(show: true)
                    self?.typeController.fetchData()
                case ActionType.tag:
                    self?.showSelectComposeTypeView(show: true)
                    self?.tagController.fetchData()
                case ActionType.remind:
                    self?.showDatePicker(show: true)
                case ActionType.priority:
                    self?.showSelectComposeTypeView(show: true)
                    self?.handlePriorityAction()
                case ActionType.send:
                    self?.addToDo()
            }
        }
        
        priorityItems = [(0, "priority_0"), (1, "priority_1"), (2, "priority_2"), (3, "priority_3"), (4, "priority_4"), (5, "priority_5")]
        
        self.datePicker.addTarget(self, action: #selector(dateChangeAction(datePicker:)), for: .valueChanged)
        
        self.selectedComposeView.deleteBlock = { [weak self] (_, _) in
            self?.showSelectedComposeView(show: self?.selectedComposeView.composeItems.count != 0)
        }
    }
    
    func handlePriorityAction() {
        var items: [ComposeTypeItem] = []
        for (id, title) in priorityItems {
            let item = ComposeTypeItem(id: id, title: title, imageNamed: "white_priority", composeType: .priority)
            items.append(item)
        }
        self.selectComposeTypeView.composeTypeItems = items
        self.selectComposeTypeView.reloadData()
    }
    
    func addToDo() {
        if self.selectedComposeView.composeItems.count == 4, let isEmpty = self.inputBox.textField.text?.isEmpty, !isEmpty {
            //Add ToDo
            let alert = UIAlertController(title: "Input Title", message: "Please input todo title.", preferredStyle: .alert)
            
            let okAction = UIAlertAction(title: "OK", style: .default) { (_) in
                if let title = alert.textFields?.first?.text {
                    let toDo = PJ_ToDo(mode: .insert)
                    for item in self.selectedComposeView.composeItems {
                        switch item.composeType {
                        case .type:
                            toDo.toDoTypeId = Int32(item.id)
                        case .tag:
                            toDo.toDoTagId = Int32(item.id)
                        case .priority:
                            toDo.priority = Int32(item.id)
                        case .remindTime:
                            toDo.remindTime = item.title
                            toDo.dueTime = item.title
                        }
                    }
                    toDo.content = self.inputBox.textField.text ?? ""
                    toDo.title = title
                    let now = self.dateFormatter.string(from: Date())
                    toDo.createTime = now
                    toDo.updateTime = now
                    toDo.state = .inProgress
                    self.toDoController.insert(toDo: toDo)
                } else {
                    SVProgressHUD.showError(withStatus: "Please input title.")
                }
            }
            
            let cancelAction = UIAlertAction(title: "Cancel", style: .cancel, handler: nil)
            
            alert.addTextField { (textField) in
                textField.placeholder = "Please input todo title."
            }
            alert.addAction(cancelAction)
            alert.addAction(okAction)
            self.present(alert, animated: true, completion: nil)
        } else {
            SVProgressHUD.showError(withStatus: "Are you sure had selected Type, Tag or Priority and input todo content?")
        }
    }
    
    private func showDatePicker(show: Bool) {
        self.inputBox.textField.resignFirstResponder()
        if show {
            self.inputBox.textField.inputAccessoryView = self.doneView
            self.inputBox.textField.inputView = self.datePicker
            self.inputBox.textField.becomeFirstResponder()
        } else {
            self.inputBox.textField.inputAccessoryView = nil
            self.inputBox.textField.inputView = nil
        }
    }
    
    @objc func dateChangeAction(datePicker: UIDatePicker) {
        
    }
    
    @objc func doneAction() {
        self.showDatePicker(show: false)
        self.inputBox.textField.becomeFirstResponder()
        self.showSelectedComposeView(show: true)
        let dateString = self.dateFormatter.string(from: self.datePicker.date)
        let remindTimeComposeItem = ComposeTypeItem(id: -1, title: dateString, imageNamed: "white_calendar", composeType: .remindTime)
        self.selectedComposeView.insertSelectedComposeItem(item: remindTimeComposeItem)
    }
    
    @objc func keyboardWillShow(note: NSNotification) {
        if let info = note.userInfo, let value = info[UIKeyboardFrameEndUserInfoKey] as? NSValue {
            UIView.animate(withDuration: 0.3) {
                var safeAreaInsetsBottom: CGFloat = 0
                if #available(iOS 11.0, *) {
                    safeAreaInsetsBottom = self.view.safeAreaInsets.bottom
                }
                self.bottomAnchorLayoutConstraint?.constant = -value.cgRectValue.size.height + safeAreaInsetsBottom
            }
        }
    }
    
    @objc func keyboardDidHidden(note: NSNotification) {
        UIView.animate(withDuration: 0.3) {
            self.bottomAnchorLayoutConstraint?.constant = 0
        }
    }
    
    @objc private func closeAction() {
        self.inputBox.textField.resignFirstResponder()
        self.dismiss(animated: true, completion: nil)
    }
    
    private func showSelectComposeTypeView(show: Bool) {
        if show {
            if let height = self.selectComposeTypeViewHeightConstraint?.constant {
                if height <= 0 {
                    self.selectComposeTypeViewHeightConstraint?.constant = AddToDoViewController.selectComposeTypeViewHeight
                }
            }
        } else {
            self.selectComposeTypeViewHeightConstraint?.constant = 0
        }
    }
    
    private func showSelectedComposeView(show: Bool) {
        if show {
            if let height = self.selectedComposeViewHeightConstraint?.constant {
                if height <= 0 {
                    self.selectedComposeViewHeightConstraint?.constant = AddToDoViewController.selectedComposeViewHeight
                }
            }
        } else {
            self.selectedComposeViewHeightConstraint?.constant = 0
        }
    }
}

extension AddToDoViewController: SelectComposeTypeViewDelegate {
    func didSelectItem(selectComposeTypeView: SelectComposeTypeView, composeTypeItem: ComposeTypeItem) {
        self.selectComposeTypeViewHeightConstraint?.constant = 0
        self.showSelectedComposeView(show: true)
        self.selectedComposeView.insertSelectedComposeItem(item: composeTypeItem)
    }
}

extension AddToDoViewController: ToDoTagDelegate {
    
    func fetchTagDataResult(isSuccess: Bool) {
        if isSuccess {
            var items: [ComposeTypeItem] = []
            for index in 0..<self.tagController.getCount() {
                let tagItem = self.tagController.toDoTagAt(index: Int32(index))
                let item = ComposeTypeItem(id: Int(tagItem.tagId), title: tagItem.tagName, imageNamed: "white_tag", composeType: .tag)
                items.append(item)
            }
            self.selectComposeTypeView.composeTypeItems = items
            DispatchQueue.main.async {
                self.selectComposeTypeView.reloadData()
            }
        } else {
            SVProgressHUD.showError(withStatus: "Fetch Tag Data Error!")
        }
    }
}

extension AddToDoViewController: ToDoTypeDelegate {
    func fetchTypeDataResult(isSuccess: Bool) {
        if isSuccess {
            var items: [ComposeTypeItem] = []
            for index in 0..<self.typeController.getCount() {
                let typeItem = self.typeController.toDoTypeAt(index: Int32(index))
                let item = ComposeTypeItem(id: Int(typeItem.typeId), title: typeItem.typeName, imageNamed: "white_type", composeType: .type)
                items.append(item)
            }
            self.selectComposeTypeView.composeTypeItems = items
            DispatchQueue.main.async {
                self.selectComposeTypeView.reloadData()
            }
        } else {
            SVProgressHUD.showError(withStatus: "Fetch Type Data Error!")
        }
    }
}

extension AddToDoViewController: ToDoDelegate {
    func insertToDoResult(isSuccess: Bool) {
        DispatchQueue.main.async {
            if isSuccess {
                NotificationCenter.default.post(name: NSNotification.Name.init(PJKeyCenter.InsertToDoNotification), object: nil)
                SVProgressHUD.showSuccess(withStatus: "Add ToDo Success!")
            } else {
                SVProgressHUD.showSuccess(withStatus: "Add ToDo Failure!")
            }
            self.dismiss(animated: true, completion: nil)
        }
    }
    
    func fetchToDoDataResult(isSuccess: Bool) {
        
    }
}
