//
//  ImageViewTitleView.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/12/12.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

typealias ImageViewTitleViewDeleteBlock = (ImageViewTitleView, ComposeTypeItem) -> Void

class ImageViewTitleView: UIView {

    var imageView: UIImageView = UIImageView()
    var titleLabel: UILabel = UILabel()
    var deleteButton: UIButton = UIButton()
    
    var imageViewTitleViewDeleteBlock: ImageViewTitleViewDeleteBlock?
    
    var composeTypeItem: ComposeTypeItem = ComposeTypeItem(id: -1, title: "", composeType: .type) {
        didSet {
            self.setTitle(title: composeTypeItem.title)
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
        self.imageViewTitleViewDeleteBlock?(self, self.composeTypeItem)
    }
    
    func setImage(imageName: String) {
        self.imageView.image = UIImage(named: imageName)
    }
    
    func setTitle(title: String) {
        self.titleLabel.text = title
    }
}
