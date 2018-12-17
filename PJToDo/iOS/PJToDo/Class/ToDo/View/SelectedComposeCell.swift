//
//  ImageViewTitleView.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/12/12.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

typealias SelectedComposeDeleteBlock = (ComposeTypeItem) -> Void

class PJButton: UIButton {
    override func point(inside point: CGPoint, with event: UIEvent?) -> Bool {
        let margin: CGFloat = 44
        let area = self.bounds.insetBy(dx: -margin, dy: -margin) //负值是方法响应范围
        return area.contains(point)
    }
}

class SelectedComposeCell: UICollectionViewCell {

    var imageView: UIImageView = UIImageView()
    var titleLabel: UILabel = UILabel()
    var deleteButton: PJButton = PJButton()
    
    var selectedComposeDeleteBlock: SelectedComposeDeleteBlock?
    
    var composeTypeItem: ComposeTypeItem = ComposeTypeItem(id: -1, title: "", imageNamed: "", composeType: .type) {
        didSet {
            self.imageView.image = UIImage(named: composeTypeItem.imageNamed)
            self.titleLabel.text = composeTypeItem.title
        }
    }

    override init(frame: CGRect) {
        super.init(frame: .zero)
        self.initView()
    }
    
    required init?(coder aDecoder: NSCoder) {
        super.init(coder: aDecoder)
    }
    
    convenience init() {
        self.init(frame: .zero)
    }
    
    private func initView() {
        self.addSubview(self.imageView)
        self.backgroundColor = .black
        self.imageView.translatesAutoresizingMaskIntoConstraints = false
        self.imageView.leadingAnchor.constraint(equalTo: self.leadingAnchor).isActive = true
        self.imageView.centerYAnchor.constraint(equalTo: self.centerYAnchor).isActive = true
        self.imageView.widthAnchor.constraint(equalToConstant: 17).isActive = true
        self.imageView.heightAnchor.constraint(equalToConstant: 17).isActive = true
        
        self.addSubview(self.titleLabel)
        self.titleLabel.font = UIFont.systemFont(ofSize: 17)
        self.titleLabel.textColor = .white
        self.titleLabel.translatesAutoresizingMaskIntoConstraints = false
        self.titleLabel.leadingAnchor.constraint(equalTo: self.imageView.trailingAnchor, constant: 5).isActive = true
        self.titleLabel.topAnchor.constraint(equalTo: self.topAnchor).isActive = true
        self.titleLabel.bottomAnchor.constraint(equalTo: self.bottomAnchor).isActive = true
        self.titleLabel.trailingAnchor.constraint(equalTo: self.trailingAnchor).isActive = true
        
        self.addSubview(self.deleteButton)
        self.deleteButton.setImage(UIImage(named: "delete"), for: .normal)
        self.deleteButton.translatesAutoresizingMaskIntoConstraints = false
        self.deleteButton.centerXAnchor.constraint(equalTo: self.trailingAnchor).isActive = true
        self.deleteButton.centerYAnchor.constraint(equalTo: self.topAnchor).isActive = true
        self.deleteButton.widthAnchor.constraint(equalToConstant: 15).isActive = true
        self.deleteButton.heightAnchor.constraint(equalToConstant: 15).isActive = true
        self.deleteButton.addTarget(self, action: #selector(deleteAction), for: .touchUpInside)
    }
    
    @objc private func deleteAction() {
        self.selectedComposeDeleteBlock?(self.composeTypeItem)
    }
}
