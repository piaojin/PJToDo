//
//  HomeTasksViewController.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/10/24.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

class HomeTasksViewController: PJBaseViewController {

    private lazy var tableView: UITableView = {
        let tempTableView = UITableView(frame: .zero, style: .grouped)
        tempTableView.translatesAutoresizingMaskIntoConstraints = false
        tempTableView.backgroundColor = .white
        tempTableView.estimatedRowHeight = 44.0
        tempTableView.rowHeight = UITableViewAutomaticDimension
        tempTableView.estimatedSectionHeaderHeight = UITableViewAutomaticDimension
        tempTableView.tableFooterView = UIView()
        tempTableView.keyboardDismissMode = .onDrag
        tempTableView.sectionIndexBackgroundColor = UIColor.clear
        tempTableView.tableFooterView?.backgroundColor = .white
        tempTableView.cellLayoutMarginsFollowReadableWidth = false
        return tempTableView
    }()
    
    let searchController = UISearchController(searchResultsController: nil)
    
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
    
    override func viewDidLoad() {
        super.viewDidLoad()
        self.initView()
        self.initData()
    }
    
    override func touchesEnded(_ touches: Set<UITouch>, with event: UIEvent?) {
//        self.tagController.findById(toDoTagId: 1)
//        self.tagController.fetchData()
//        PJHttpRequest.login(name: "piaojin", passWord: "weng804488815", responseBlock: { (data, isSuccess) -> Void in
//            print("isSuccess: \(isSuccess)")
//        })
        
//        PJHttpRequest.authorization(authorization: "Basic cGlhb2ppbjp3ZW5nODA0NDg4ODE1") { (authorization, isSuccess) in
//            print("isSuccess: \(isSuccess)")
//        }
    }
    
    private func initView() {
        self.view.backgroundColor = .white
        self.title = "Tasks"
        self.view.addSubview(tableView)
        tableView.leadingAnchor.constraint(equalTo: view.leadingAnchor).isActive = true
        tableView.trailingAnchor.constraint(equalTo: view.trailingAnchor).isActive = true
        if #available(iOS 11.0, *) {
            self.tableView.topAnchor.constraint(equalTo: self.view.topAnchor).isActive = true
            self.tableView.bottomAnchor.constraint(equalTo: self.view.bottomAnchor).isActive = true
        } else {
            tableView.topAnchor.constraint(equalTo: bottomLayoutGuide.topAnchor).isActive = true
            tableView.bottomAnchor.constraint(equalTo: bottomLayoutGuide.bottomAnchor).isActive = true
        }
        
        // Setup the Search Controller
        searchController.searchResultsUpdater = self
        searchController.obscuresBackgroundDuringPresentation = false
        searchController.searchBar.placeholder = "Search Tasks"
        if #available(iOS 11.0, *) {
            navigationItem.searchController = searchController
        } else {
            // Fallback on earlier versions
            self.tableView.tableHeaderView = searchController.searchBar
        }
        definesPresentationContext = true
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
        self.tableView.delegate = self
        self.tableView.dataSource = self
//        self.tagController.fetchData()
        self.toDoController.fetchData()
//        let controller = ToDoController(delegate: self)
//        controller.fetchData()
    }

    override func didReceiveMemoryWarning() {
        super.didReceiveMemoryWarning()
        // Dispose of any resources that can be recreated.
    }
}

extension HomeTasksViewController: UITableViewDelegate, UITableViewDataSource {
    func tableView(_ tableView: UITableView, numberOfRowsInSection section: Int) -> Int {
        return Int(self.toDoController.getToDoCountAtSection(section: Int32(section)))
    }
    
    func tableView(_ tableView: UITableView, cellForRowAt indexPath: IndexPath) -> UITableViewCell {
        return UITableViewCell()
    }
    
    func tableView(_ tableView: UITableView, viewForHeaderInSection section: Int) -> UIView? {
        let headerView = TasksHeaderView.initWith(tableView: tableView)
        let model = self.toDoController.toDoAt(section: Int32(section), index: 0)
        if model.state == .determined {
            headerView.setTitleColor(color: .red)
            headerView.setTitle(title: "title")
        }
        return headerView
    }
    
    func tableView(_ tableView: UITableView, heightForHeaderInSection section: Int) -> CGFloat {
        return 0
    }
}

extension HomeTasksViewController: ToDoDelegate {
    func insertToDoResult(isSuccess: Bool) {
        
    }
    
    func deleteToDoResult(isSuccess: Bool) {
        
    }
    
    func updateToDoResult(isSuccess: Bool) {
        
    }
    
    func fetchToDoDataResult(isSuccess: Bool) {
        if isSuccess {
            self.tableView.reloadData()
        }
    }
    
    func findToDoByIdResult(toDo: PJ_ToDo?, isSuccess: Bool) {
        
    }
}

extension HomeTasksViewController: ToDoTypeDelegate {
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

extension HomeTasksViewController: ToDoTagDelegate {
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

extension HomeTasksViewController: UISearchResultsUpdating {
    // MARK: - UISearchResultsUpdating Delegate
    func updateSearchResults(for searchController: UISearchController) {
        // TODO
    }
}
