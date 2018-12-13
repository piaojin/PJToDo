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
    
    lazy var dataPicker: UIDatePicker = {
        let dataPicker = UIDatePicker()
        dataPicker.translatesAutoresizingMaskIntoConstraints = false
        return dataPicker
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
    
    lazy var typeController: ToDoTypeController = {
        let controller = ToDoTypeController(delegate: self)
        return controller
    }()
    
    lazy var tagController: ToDoTagController = {
        let controller = ToDoTagController(delegate: self)
        return controller
    }()
    
    var priorityItems: [(Int, String)] = []
    
    var typeCompose: ComposeTypeItem = ComposeTypeItem(id: -1, title: "", composeType: .type)
    var tagCompose: ComposeTypeItem = ComposeTypeItem(id: -1, title: "", composeType: .tag)
    var priorityCompose: ComposeTypeItem = ComposeTypeItem(id: -1, title: "", composeType: .priority)
    
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
        
        self.view.addSubview(self.selectComposeTypeView)
        self.selectComposeTypeView.leadingAnchor.constraint(equalTo: self.view.leadingAnchor).isActive = true
        self.selectComposeTypeView.trailingAnchor.constraint(equalTo: self.view.trailingAnchor).isActive = true
        self.selectComposeTypeView.bottomAnchor.constraint(equalTo: self.selectedComposeView.topAnchor).isActive = true
        self.selectComposeTypeViewHeightConstraint = self.selectComposeTypeView.heightAnchor.constraint(equalToConstant: 0)
        self.selectComposeTypeViewHeightConstraint?.isActive = true
        self.selectedComposeViewHeightConstraint?.constant = AddToDoViewController.selectedComposeViewHeight
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
        
        self.dataPicker.addTarget(self, action: #selector(dateChangeAction(datePicker:)), for: .valueChanged)
    }
    
    func handlePriorityAction() {
        var items: [ComposeTypeItem] = []
        for (id, title) in priorityItems {
            let item = ComposeTypeItem(id: id, title: title, composeType: .priority)
            items.append(item)
        }
        self.selectComposeTypeView.composeTypeItems = items
        self.selectComposeTypeView.reloadData()
    }
    
    func addToDo() {
        if self.typeCompose.id == -1 || self.tagCompose.id == -1 || self.priorityCompose.id == -1 {
            SVProgressHUD.showError(withStatus: "Are you sure had selected Type, Tag or Priority?")
        }
        //Add ToDo
    }
    
    private func showDatePicker(show: Bool) {
        self.inputBox.textField.resignFirstResponder()
        if show {
            self.inputBox.textField.inputAccessoryView = self.doneView
            self.inputBox.textField.inputView = self.dataPicker
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
        UIView.animate(withDuration: 0.3) {
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
    }
}

extension AddToDoViewController: SelectComposeTypeViewDelegate {
    func didSelectItem(selectComposeTypeView: SelectComposeTypeView, composeTypeItem: ComposeTypeItem) {
        switch composeTypeItem.composeType {
            case .type:
                self.typeCompose = composeTypeItem
            case .tag:
                self.tagCompose = composeTypeItem
            case .priority:
                self.priorityCompose = composeTypeItem
        }
        UIView.animate(withDuration: 0.3) {
            self.selectComposeTypeViewHeightConstraint?.constant = 0
        }
    }
}

extension AddToDoViewController: ToDoTagDelegate {
    
    func fetchTagDataResult(isSuccess: Bool) {
        if isSuccess {
            var items: [ComposeTypeItem] = []
            for index in 0..<self.tagController.getCount() {
                let tagItem = self.tagController.toDoTagAt(index: Int32(index))
                let item = ComposeTypeItem(id: Int(tagItem.tagId), title: tagItem.tagName, composeType: .tag)
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
                let item = ComposeTypeItem(id: Int(typeItem.typeId), title: typeItem.typeName, composeType: .type)
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
