//
//  HomeViewController.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/10/24.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

class HomeViewController: UIViewController {

    lazy var typeController: ToDoTypeController = {
        let controller = ToDoTypeController(delegate: self)
        return controller
    }()
    
    lazy var tagController: ToDoTagController = {
        let controller = ToDoTagController(delegate: self)
        return controller
    }()
    
    override func viewDidLoad() {
        super.viewDidLoad()
        self.initView()
        self.initData()
    }
    
    override func touchesEnded(_ touches: Set<UITouch>, with event: UIEvent?) {
//        self.tagController.findById(toDoTagId: 1)
        self.tagController.fetchData()
    }
    
    private func initView() {
        self.view.backgroundColor = .white
    }
    
    private func initData() {
//        let typeController = ToDoTypeController(delegate: self)
//        typeController.fetchData()
//        let toDoType = PJToDoType(typeName: "piaojin3!")
//        self.typeController.insert(toDoType: toDoType)
//        self.typeController.findById(toDoTypeId: 3)
//        self.typeController.findByName(typeName: "piaojin2!")
//        self.typeController.fetchData()
//        let toDoTag = PJToDoTag(tagName: "piaojin tag")
//        self.tagController.insert(toDoTag: toDoTag)
//        self.tagController.findById(toDoTagId: 1)
//        self.tagController.findByName(tagName: "piaojin tag")
    }

    override func didReceiveMemoryWarning() {
        super.didReceiveMemoryWarning()
        // Dispose of any resources that can be recreated.
    }
}

extension HomeViewController: ToDoTypeDelegate {
    func fetchTypeDataResult(isSuccess: Bool) {
        
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

        }
    }
}

extension HomeViewController: ToDoTagDelegate {
    func findTagByIdResult(toDoTag: PJToDoTag?, isSuccess: Bool) {
        
    }
    
    func findTagByNameResult(toDoTag: PJToDoTag?, isSuccess: Bool) {
        
    }
    
    func fetchTagDataResult(isSuccess: Bool) {
        if isSuccess {

        }
    }

    func findTagByNameResult(toDoTag: PJToDoTag, isSuccess: Bool) {
        if isSuccess {

        }
    }

    func findTagByIdResult(toDoTag: PJToDoTag, isSuccess: Bool) {
        if isSuccess {
            self.tagController.findByName(tagName: toDoTag.tagName)
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

        }
    }
}
