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
        tempTableView.tableHeaderView = UIView(frame: CGRect(x: 0, y: 0, width: 0, height: CGFloat.leastNormalMagnitude))
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
    
    static let ToDoId = "ToDoId"
    
    override func viewDidLoad() {
        super.viewDidLoad()
        self.initView()
        self.initData()
    }
    
    private func initView() {
        self.view.backgroundColor = .white
        self.title = "Tasks"
        self.view.addSubview(tableView)
        if #available(iOS 11.0, *) {
            tableView.contentInsetAdjustmentBehavior = .never
        } else {
            self.automaticallyAdjustsScrollViewInsets = false
        }
        tableView.leadingAnchor.constraint(equalTo: view.leadingAnchor).isActive = true
        tableView.trailingAnchor.constraint(equalTo: view.trailingAnchor).isActive = true
        if #available(iOS 11.0, *) {
            self.tableView.topAnchor.constraint(equalTo: self.view.safeAreaLayoutGuide.topAnchor).isActive = true
            self.tableView.bottomAnchor.constraint(equalTo: self.view.safeAreaLayoutGuide.bottomAnchor).isActive = true
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
        self.tableView.register(ToDoCell.classForCoder(), forCellReuseIdentifier: HomeTasksViewController.ToDoId)
        self.tableView.delegate = self
        self.tableView.dataSource = self
        self.toDoController.fetchData()
        NotificationCenter.default.addObserver(self, selector: #selector(updateDate), name: NSNotification.Name.init(PJKeyCenter.InsertToDoNotification), object: nil)
    }
    
    @objc private func updateDate() {
        self.toDoController.fetchData()
    }
    
    private func deleteAction(indexPath: IndexPath) {
        
    }
    
    private func completeAction(indexPath: IndexPath) {
        
    }
    
    private func unDeterminedAction(indexPath: IndexPath) {
        
    }

    override func didReceiveMemoryWarning() {
        super.didReceiveMemoryWarning()
        // Dispose of any resources that can be recreated.
    }
}

extension HomeTasksViewController: UITableViewDelegate, UITableViewDataSource {
    func tableView(_ tableView: UITableView, numberOfRowsInSection section: Int) -> Int {
        return Int(self.toDoController.getToDoCountAtSection(section: section))
    }
    
    func tableView(_ tableView: UITableView, cellForRowAt indexPath: IndexPath) -> UITableViewCell {
        let cell = tableView.dequeueReusableCell(withIdentifier: HomeTasksViewController.ToDoId, for: indexPath)
        if let tempCell = cell as? ToDoCell {
            tempCell.item = self.toDoController.toDoAt(section: indexPath.section, index: indexPath.row)
        }
        return cell
    }
    
    func tableView(_ tableView: UITableView, editActionsForRowAt indexPath: IndexPath) -> [UITableViewRowAction]? {
        var actions: [UITableViewRowAction] = []
        let model = self.toDoController.toDoAt(section: indexPath.section, index: indexPath.row)
        let unDeterminedAction = UITableViewRowAction(style: UITableViewRowActionStyle.normal, title: "待定") { (action, tempIndexPath) in
            self.unDeterminedAction(indexPath: tempIndexPath)
        }
        unDeterminedAction.backgroundColor = .orange
        
        let completeAction = UITableViewRowAction(style: UITableViewRowActionStyle.normal, title: "完成") { (action, tempIndexPath) in
            self.completeAction(indexPath: tempIndexPath)
        }
        completeAction.backgroundColor = .green
        
        let deleteAction = UITableViewRowAction(style: UITableViewRowActionStyle.destructive, title: "删除") { (action, tempIndexPath) in
            self.deleteAction(indexPath: tempIndexPath)
        }
        deleteAction.backgroundColor = .red
        
        switch model.state {
            case .inProgress:
                actions.append(completeAction)
                actions.append(unDeterminedAction)
                actions.append(deleteAction)
            case .unDetermined:
                actions.append(completeAction)
                actions.append(deleteAction)
            case .completed:
                actions.append(unDeterminedAction)
                actions.append(deleteAction)
            case .overdue:
                actions.append(completeAction)
                actions.append(unDeterminedAction)
                actions.append(deleteAction)
        }
        
        return actions
    }
    
    func tableView(_ tableView: UITableView, viewForHeaderInSection section: Int) -> UIView? {
        if self.toDoController.getToDoCountAtSection(section: section) > 0 {
            let headerView = TasksHeaderView()
            headerView.title = self.toDoController.toDoTitle(section: section)
            let model = self.toDoController.toDoAt(section: section, index: 0)
            if model.state == .overdue {
                headerView.setTitleColor(color: .red)
            } else {
                headerView.setTitleColor(color: .black)
            }
            return headerView
        }
        return nil
    }
    
    func tableView(_ tableView: UITableView, heightForHeaderInSection section: Int) -> CGFloat {
        let count = self.toDoController.getToDoCountAtSection(section: section)
        return count == 0 ? CGFloat.leastNormalMagnitude : UITableViewAutomaticDimension
    }
    
    func tableView(_ tableView: UITableView, didSelectRowAt indexPath: IndexPath) {
        tableView.deselectRow(at: indexPath, animated: true)
    }
}

extension HomeTasksViewController: ToDoDelegate {
    
    func deleteToDoResult(isSuccess: Bool) {
        
    }
    
    func updateToDoResult(isSuccess: Bool) {
        
    }
    
    func fetchToDoDataResult(isSuccess: Bool) {
        if isSuccess {
            DispatchQueue.main.async {
                self.tableView.reloadData()
            }
        }
    }
    
    func findToDoByIdResult(toDo: PJ_ToDo?, isSuccess: Bool) {
        
    }
}

extension HomeTasksViewController: UISearchResultsUpdating {
    // MARK: - UISearchResultsUpdating Delegate
    func updateSearchResults(for searchController: UISearchController) {
        // TODO
    }
}
