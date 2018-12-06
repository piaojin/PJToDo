//
//  AddToDoViewController.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/11/30.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

class AddToDoViewController: PJBaseViewController {
    
    private lazy var tableView: UITableView = {
        let tempTableView = UITableView(frame: .zero, style: .plain)
        tempTableView.translatesAutoresizingMaskIntoConstraints = false
        tempTableView.backgroundColor = .white
        tempTableView.estimatedRowHeight = 44.0
        tempTableView.rowHeight = UITableViewAutomaticDimension
        tempTableView.tableFooterView = UIView()
        tempTableView.tableFooterView?.backgroundColor = .white
        tempTableView.cellLayoutMarginsFollowReadableWidth = false
        return tempTableView
    }()
    
    lazy var inputBox: PJInputBox = {
        let inputBox = PJInputBox()
        inputBox.translatesAutoresizingMaskIntoConstraints = false
        return inputBox
    }()
    
    var tableViewHeightAnchorLayoutConstraint: NSLayoutConstraint?
    var bottomAnchorLayoutConstraint: NSLayoutConstraint?
    
    static let tableViewHeight = 100
    
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
        
        self.view.addSubview(self.tableView)
        self.tableView.leadingAnchor.constraint(equalTo: self.view.leadingAnchor).isActive = true
        self.tableView.trailingAnchor.constraint(equalTo: self.view.trailingAnchor).isActive = true
        self.tableView.bottomAnchor.constraint(equalTo: self.inputBox.topAnchor).isActive = true
        self.tableViewHeightAnchorLayoutConstraint = self.tableView.heightAnchor.constraint(equalToConstant: 0)
        self.tableViewHeightAnchorLayoutConstraint?.isActive = true
    }
    
    private func initData() {
        NotificationCenter.default.addObserver(self, selector: #selector(self.keyboardWillShow(note:)), name: NSNotification.Name.UIKeyboardWillShow, object: nil)
        NotificationCenter.default.addObserver(self, selector: #selector(self.keyboardDidHidden(note:)), name: NSNotification.Name.UIKeyboardDidHide, object: nil)
        
        self.tableView.delegate = self
        self.tableView.dataSource = self
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
}

extension AddToDoViewController: UITableViewDelegate, UITableViewDataSource {
    func tableView(_ tableView: UITableView, numberOfRowsInSection section: Int) -> Int {
        return 0
    }
    
    func tableView(_ tableView: UITableView, cellForRowAt indexPath: IndexPath) -> UITableViewCell {
        return UITableViewCell()
    }
}
