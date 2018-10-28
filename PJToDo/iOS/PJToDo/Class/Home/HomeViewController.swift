//
//  HomeViewController.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/10/24.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

class HomeViewController: UIViewController {

//    lazy var controller: ToDoTypeController = {
//        let controller = ToDoTypeController(delegate: self)
//        return controller
//    }()
    
    override func viewDidLoad() {
        super.viewDidLoad()
        self.initView()
        self.initData()
    }
    
    private func initView() {
        self.view.backgroundColor = .white
    }
    
    private func initData() {
        let controller = ToDoTypeController(delegate: self)
        let toDoType = PJToDoType(typeName: "hello piaojin!")
        controller.insert(toDoType: toDoType)
    }

    override func didReceiveMemoryWarning() {
        super.didReceiveMemoryWarning()
        // Dispose of any resources that can be recreated.
    }
}

extension HomeViewController: ToDoTypeDelegate {
    func insertResult(_id: Int, isSuccess: Bool) {
        
    }
}
