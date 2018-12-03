//
//  AddToDoViewController.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/11/30.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

class AddToDoViewController: PJBaseViewController {
    override func viewDidLoad() {
        super.viewDidLoad()
        self.initView()
    }
    
    private func initView() {
        self.title = "New ToDo"
        self.navigationItem.leftBarButtonItem = UIBarButtonItem(title: "Close", style: UIBarButtonItemStyle.done, target: self, action: #selector(closeAction))
    }
    
    @objc private func closeAction() {
        self.dismiss(animated: true, completion: nil)
    }
}
