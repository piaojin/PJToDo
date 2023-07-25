//
//  SearchToDoViewController.swift
//  PJToDo
//
//  Created by piaojin on 2019/1/5.
//  Copyright © 2019 piaojin. All rights reserved.
//

import UIKit
import SVProgressHUD

class SearchToDoViewController: PJBaseViewController {
    
    private lazy var tableView: UITableView = {
        let tempTableView = UITableView(frame: .zero, style: .plain)
        tempTableView.translatesAutoresizingMaskIntoConstraints = false
        tempTableView.backgroundColor = .white
        tempTableView.estimatedRowHeight = 44.0
        tempTableView.rowHeight = UITableView.automaticDimension
        tempTableView.estimatedSectionHeaderHeight = UITableView.automaticDimension
        tempTableView.tableFooterView = UIView()
        tempTableView.tableHeaderView = UIView(frame: CGRect(x: 0, y: 0, width: 0, height: CGFloat.leastNormalMagnitude))
        tempTableView.keyboardDismissMode = .onDrag
        tempTableView.sectionIndexBackgroundColor = UIColor.clear
        tempTableView.tableFooterView?.backgroundColor = .white
        tempTableView.cellLayoutMarginsFollowReadableWidth = false
        return tempTableView
    }()
    
    static let SearchToDoCellId = "SearchToDoCellId"
    
    lazy var toDoSearchController: ToDoSearchController = {
        let toDoSearchController = ToDoSearchController(delegate: self)
        return toDoSearchController
    }()
    
    lazy var toDoController: ToDoController = {
        let controller = ToDoController(delegate: self)
        return controller
    }()
    
    var isSearching = false
    
    var isNeedReSearch = false
    
    weak var searchController: UISearchController?
    
    weak var superNavigationController: UINavigationController?
    
    override func viewDidLoad() {
        super.viewDidLoad()
        self.initView()
        self.initData()
    }
    
    private func initView() {
        self.view.backgroundColor = .white
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
    }
    
    private func initData() {
        self.tableView.register(SearchToDoCell.classForCoder(), forCellReuseIdentifier: SearchToDoViewController.SearchToDoCellId)
        self.tableView.delegate = self
        self.tableView.dataSource = self
    }
    
    private func doSearch(searchText: String) {
        if !self.isSearching {
            self.isSearching = true
            self.toDoSearchController.findToDoLikeTitle(title: searchText)
        } else {
            self.isNeedReSearch = true
        }
    }
    
    private func deleteAction(toDo: PJ_ToDo) {
        self.toDoController.delete(toDoId: Int(toDo.toDoId))
        SVProgressHUD.show(withStatus: "Please wait...")
    }
    
    private func completeAction(toDo: PJ_ToDo) {
        toDo.state = .completed
        self.updateToDoState(toDo: toDo)
    }
    
    private func unDeterminedAction(toDo: PJ_ToDo) {
        toDo.state = .unDetermined
        self.updateToDoState(toDo: toDo)
    }
    
    private func inProgressAction(toDo: PJ_ToDo) {
        toDo.state = .inProgress
        self.updateToDoState(toDo: toDo)
    }
    
    private func updateToDoState(toDo: PJ_ToDo) {
        self.toDoController.update(toDo: toDo)
        SVProgressHUD.show(withStatus: "Please wait...")
    }
}

extension SearchToDoViewController: UISearchResultsUpdating, UISearchControllerDelegate {
    // MARK: - UISearchResultsUpdating Delegate
    func updateSearchResults(for searchController: UISearchController) {
        if self.searchController == nil {
            self.searchController = searchController
        }
        if let searchText = searchController.searchBar.text, !searchText.isEmpty {
            self.doSearch(searchText: searchText)
        }
    }
}

extension SearchToDoViewController: UITableViewDelegate, UITableViewDataSource {
    func tableView(_ tableView: UITableView, numberOfRowsInSection section: Int) -> Int {
        return Int(self.toDoSearchController.getSearchToDoResultCount())
    }
    
    func tableView(_ tableView: UITableView, cellForRowAt indexPath: IndexPath) -> UITableViewCell {
        let cell = tableView.dequeueReusableCell(withIdentifier: SearchToDoViewController.SearchToDoCellId, for: indexPath)
        if let tempCell = cell as? SearchToDoCell {
            tempCell.item = self.toDoSearchController.searchToDoResultAtIndex(index: Int32(indexPath.row))
        }
        return cell
    }
    
    func tableView(_ tableView: UITableView, editActionsForRowAt indexPath: IndexPath) -> [UITableViewRowAction]? {
        var actions: [UITableViewRowAction] = []
        let toDo = self.toDoSearchController.searchToDoResultAtIndex(index: Int32(indexPath.row))
        let unDeterminedAction = UITableViewRowAction(style: UITableViewRowAction.Style.normal, title: "待定") { (action, tempIndexPath) in
            self.unDeterminedAction(toDo: toDo)
        }
        unDeterminedAction.backgroundColor = .orange
        
        let completeAction = UITableViewRowAction(style: UITableViewRowAction.Style.normal, title: "完成") { (action, tempIndexPath) in
            self.completeAction(toDo: toDo)
        }
        completeAction.backgroundColor = .green
        
        let deleteAction = UITableViewRowAction(style: UITableViewRowAction.Style.destructive, title: "删除") { (action, tempIndexPath) in
            self.deleteAction(toDo: toDo)
        }
        deleteAction.backgroundColor = .red
        
        let inProgressAction = UITableViewRowAction(style: UITableViewRowAction.Style.destructive, title: "开始") { (action, tempIndexPath) in
            self.inProgressAction(toDo: toDo)
        }
        inProgressAction.backgroundColor = .blue
        
        switch toDo.state {
        case .inProgress:
            actions.append(completeAction)
            actions.append(unDeterminedAction)
            actions.append(deleteAction)
        case .unDetermined:
            actions.append(inProgressAction)
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
    
    func tableView(_ tableView: UITableView, didSelectRowAt indexPath: IndexPath) {
        tableView.deselectRow(at: indexPath, animated: true)
        let toDo = self.toDoSearchController.searchToDoResultAtIndex(index: Int32(indexPath.row))
        let detailViewController = DetailViewController(toDo: toDo)
        detailViewController.hidesBottomBarWhenPushed = true
        detailViewController.delegate = self
        self.superNavigationController?.pushViewController(detailViewController, animated: true)
    }
}

extension SearchToDoViewController: ToDoSearchDelegate {
    func findToDoLikeTitleResult(isSuccess: Bool) {
        DispatchQueue.main.async {
            self.isSearching = false
            self.tableView.reloadData()
            if self.isNeedReSearch {
                self.isNeedReSearch = false
                if let searchText = self.searchController?.searchBar.text {
                    self.doSearch(searchText: searchText)
                }
            } else {
                let count = self.toDoSearchController.getSearchToDoResultCount()
                if count == 0, let searchText = self.searchController?.searchBar.text, !searchText.isEmpty {
                    self.showEmpty(show: true)
                } else {
                    self.showEmpty(show: false)
                }
            }
        }
    }
    
    func findToDoByTitleResult(toDo: PJ_ToDo?, isSuccess: Bool) {
        
    }
}

extension SearchToDoViewController: DetailViewControllerDelegate {
    func didUpdateToDo(detailViewController: DetailViewController, isSuccess: Bool) {
        DispatchQueue.main.async {
            if isSuccess, let searchText = self.searchController?.searchBar.text {
                self.doSearch(searchText: searchText)
            }
        }
    }
    
    func didDeleteToDo(detailViewController: DetailViewController, isSuccess: Bool) {
        DispatchQueue.main.async {
            if isSuccess, let searchText = self.searchController?.searchBar.text {
                self.doSearch(searchText: searchText)
            }
        }
    }
}

extension SearchToDoViewController: ToDoDelegate {
    func fetchToDoDataResult(isSuccess: Bool) {
        
    }
    
    func deleteToDoResult(isSuccess: Bool) {
        DispatchQueue.main.async {
            if isSuccess, let searchText = self.searchController?.searchBar.text {
                self.doSearch(searchText: searchText)
            }
        }
    }
    
    func updateToDoResult(isSuccess: Bool) {
        DispatchQueue.main.async {
            if isSuccess, let searchText = self.searchController?.searchBar.text {
                self.doSearch(searchText: searchText)
            }
        }
    }
}
