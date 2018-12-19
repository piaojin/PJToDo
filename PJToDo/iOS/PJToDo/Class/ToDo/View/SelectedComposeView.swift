//
//  SelectedComposeView.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/12/12.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

typealias SelectedComposeViewDeleteBlock = (SelectedComposeView, ComposeTypeItem) -> Void

class SelectedComposeView: UIView {
    
    lazy var collectionView: UICollectionView = {
        let layout = UICollectionViewFlowLayout()
        layout.scrollDirection = .horizontal
        layout.minimumInteritemSpacing = 10
        layout.sectionInset = UIEdgeInsets(top: 0, left: 10, bottom: 0, right: 10)
       let collectionView = UICollectionView(frame: .zero, collectionViewLayout: layout)
        collectionView.translatesAutoresizingMaskIntoConstraints = false
        collectionView.backgroundColor = .white
        collectionView.showsHorizontalScrollIndicator = false
        return collectionView
    }()
    
    var composeItems: [ComposeTypeItem] = []
    
    static let SelectedComposeCellId = "SelectedComposeCellId"
    
    var deleteBlock: SelectedComposeViewDeleteBlock?
    
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
        self.backgroundColor = .white
        self.addSubview(self.collectionView)
        self.collectionView.topAnchor.constraint(equalTo: self.topAnchor).isActive = true
        self.collectionView.bottomAnchor.constraint(equalTo: self.bottomAnchor).isActive = true
        self.collectionView.leadingAnchor.constraint(equalTo: self.leadingAnchor).isActive = true
        self.collectionView.trailingAnchor.constraint(equalTo: self.trailingAnchor).isActive = true
    }
    
    private func initData() {
        self.collectionView.register(SelectedComposeCell.classForCoder(), forCellWithReuseIdentifier: SelectedComposeView.SelectedComposeCellId)
        self.collectionView.delegate = self
        self.collectionView.dataSource = self
    }
    
    func insertSelectedComposeItem(item: ComposeTypeItem) {
        
        if self.composeItems.count == 0 {
            self.composeItems.append(item)
        } else {
            if let index = self.composeItems.firstIndex(where: { (tempItem) -> Bool in
                tempItem.composeType == item.composeType
            }) {
                self.composeItems[index] = item
            } else {
                self.composeItems.append(item)
            }
        }
        
        self.collectionView.reloadData()
    }
    
    private func deleteItem(at index: Int) {
        self.composeItems.remove(at: index)
        self.collectionView.reloadData()
    }
}

extension SelectedComposeView: UICollectionViewDelegate, UICollectionViewDataSource, UICollectionViewDelegateFlowLayout {
    func collectionView(_ collectionView: UICollectionView, numberOfItemsInSection section: Int) -> Int {
        return self.composeItems.count
    }
    
    func collectionView(_ collectionView: UICollectionView, cellForItemAt indexPath: IndexPath) -> UICollectionViewCell {
        let cell = collectionView.dequeueReusableCell(withReuseIdentifier: SelectedComposeView.SelectedComposeCellId, for: indexPath)
        if let tempCell = cell as? SelectedComposeCell {
            tempCell.composeTypeItem = self.composeItems[indexPath.item]
            tempCell.selectedComposeDeleteBlock = { [weak self] (item) in
                if let index = self?.composeItems.firstIndex(of: item) {
                    self?.deleteItem(at: index)
                    if let self = self {
                        self.deleteBlock?(self, item)
                    }
                }
            }
        }
        return cell
    }
    
    func collectionView(_ collectionView: UICollectionView, layout collectionViewLayout: UICollectionViewLayout, sizeForItemAt indexPath: IndexPath) -> CGSize {
        let title = self.composeItems[indexPath.item].title
        let titleWidth = title.pj_boundingSizeBySize(CGSize(width: CGFloat.greatestFiniteMagnitude, height: collectionView.frame.size.height), font: UIFont.systemFont(ofSize: 17)).width
        return CGSize(width: titleWidth + 17 + 5, height: 40)
    }
}
