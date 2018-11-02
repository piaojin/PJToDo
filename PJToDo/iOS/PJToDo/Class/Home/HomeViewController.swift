//
//  HomeViewController.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/10/24.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

class HomeViewController: UIViewController {

    lazy var controller: ToDoTypeController = {
        let controller = ToDoTypeController(delegate: self)
        return controller
    }()
    
    override func viewDidLoad() {
        super.viewDidLoad()
        self.initView()
        self.initData()
    }
    
    private func initView() {
        self.view.backgroundColor = .white
    }
    
    private func initData() {
//        let controller = ToDoTypeController(delegate: self)
//        controller.fetchData()
//        let toDoType = PJToDoType(typeName: "piaojin2!")
//        self.controller.insert(toDoType: toDoType)
//        self.controller.findById(toDoTypeId: 3)
//        self.controller.findByName(typeName: "piaojin2!")
        self.controller.fetchData()
//        print("\(self.controller)")
    }

    override func didReceiveMemoryWarning() {
        super.didReceiveMemoryWarning()
        // Dispose of any resources that can be recreated.
    }
}

extension HomeViewController: ToDoTypeDelegate {
    func fetchDataResult(isSuccess: Bool) {
        if isSuccess {
            
        }
    }
    
    func findByNameResult(toDoType: PJToDoType, isSuccess: Bool) {
        if isSuccess {
            print("typeName: \(toDoType.typeName)")
        }
    }
    
    func findByIdResult(toDoType: PJToDoType, isSuccess: Bool) {
        if isSuccess {
            print("typeName: \(toDoType.typeName)")
        }
    }
    
    func updateResult(isSuccess: Bool) {
        
    }
    
    func deleteResult(isSuccess: Bool) {
        
    }
    
    func insertResult(isSuccess: Bool) {
        
    }
}
