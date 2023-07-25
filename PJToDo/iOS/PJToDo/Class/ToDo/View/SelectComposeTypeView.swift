//
//  SelectTypeOrTagView.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/12/8.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

protocol SelectComposeTypeViewDelegate: NSObjectProtocol {
    func didSelectItem(selectComposeTypeView: SelectComposeTypeView, composeTypeItem: ComposeTypeItem)
}

class SelectComposeTypeView: UIView {
    
    var composeTypeItems: [ComposeTypeItem] = []
    
    private lazy var tableView: UITableView = {
        let tempTableView = UITableView(frame: .zero, style: .plain)
        tempTableView.translatesAutoresizingMaskIntoConstraints = false
        tempTableView.backgroundColor = UIColor.colorWithRGB(red: 249, green: 249, blue: 249)
        tempTableView.estimatedRowHeight = 44.0
        tempTableView.rowHeight = UITableView.automaticDimension
        tempTableView.tableFooterView = UIView()
        tempTableView.sectionIndexBackgroundColor = UIColor.clear
        tempTableView.tableFooterView?.backgroundColor = .white
        tempTableView.cellLayoutMarginsFollowReadableWidth = false
        return tempTableView
    }()
    
    static let ComposeTypeCellId = "ComposeTypeCellId"
    
    weak var delegate: SelectComposeTypeViewDelegate?
    
    override init(frame: CGRect) {
        super.init(frame: .zero)
        self.initView()
        self.initData()
    }
    
    required init?(coder aDecoder: NSCoder) {
        super.init(coder: aDecoder)
    }
    
    convenience init() {
        self.init(frame: .zero)
    }
    
    private func initView() {
        self.addSubview(self.tableView)
        self.tableView.leadingAnchor.constraint(equalTo: self.leadingAnchor).isActive = true
        self.tableView.trailingAnchor.constraint(equalTo: self.trailingAnchor).isActive = true
        self.tableView.topAnchor.constraint(equalTo: self.topAnchor).isActive = true
        self.tableView.bottomAnchor.constraint(equalTo: self.bottomAnchor).isActive = true
    }
    
    private func initData() {
        self.tableView.register(ComposeTypeCell.classForCoder(), forCellReuseIdentifier: SelectComposeTypeView.ComposeTypeCellId)
        self.tableView.delegate = self
        self.tableView.dataSource = self
    }
    
    func reloadData() {
        self.tableView.reloadData()
    }
}

extension SelectComposeTypeView: UITableViewDelegate, UITableViewDataSource {
    func tableView(_ tableView: UITableView, numberOfRowsInSection section: Int) -> Int {
        return self.composeTypeItems.count
    }
    
    func tableView(_ tableView: UITableView, cellForRowAt indexPath: IndexPath) -> UITableViewCell {
        let cell = tableView.dequeueReusableCell(withIdentifier: SelectComposeTypeView.ComposeTypeCellId, for: indexPath)
        if let tempCell = cell as? ComposeTypeCell {
            tempCell.composeTypeItem = self.composeTypeItems[indexPath.row]
            return tempCell
        }
        return UITableViewCell()
    }
    
    func tableView(_ tableView: UITableView, didSelectRowAt indexPath: IndexPath) {
        tableView.deselectRow(at: indexPath, animated: true)
        let item = self.composeTypeItems[indexPath.row]
        self.delegate?.didSelectItem(selectComposeTypeView: self, composeTypeItem: item)
    }
}
