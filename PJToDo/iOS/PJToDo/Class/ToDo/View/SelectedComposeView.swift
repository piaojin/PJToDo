//
//  SelectedComposeView.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/12/12.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

typealias SelectedComposeViewDidSelectBlock = (SelectedComposeView, ComposeTypeItem) -> Void

class SelectedComposeView: UIView {

    var selectedComposeViewDidSelectBlock: SelectedComposeViewDidSelectBlock?
    
    lazy var typeImageViewTitleView: ImageViewTitleView = {
        return self.createImageViewTitleView(imageName: "white_type")
    }()
    
    lazy var tagImageViewTitleView: ImageViewTitleView = {
        return self.createImageViewTitleView(imageName: "white_tag")
    }()
    
    lazy var calendarImageViewTitleView: ImageViewTitleView = {
        return self.createImageViewTitleView(imageName: "white_calendar")
    }()
    
    lazy var priorityImageViewTitleView: ImageViewTitleView = {
        return self.createImageViewTitleView(imageName: "priority_5")
    }()
    
    var typeWidthConstraint: NSLayoutConstraint?
    var tagWidthConstraint: NSLayoutConstraint?
    var calendarWidthConstraint: NSLayoutConstraint?
    var priorityWidthConstraint: NSLayoutConstraint?
    
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
    
    private func createImageViewTitleView(imageName: String) -> ImageViewTitleView {
        let imageViewTitleView = ImageViewTitleView()
        imageViewTitleView.setImage(imageName: imageName)
        imageViewTitleView.translatesAutoresizingMaskIntoConstraints = false
        imageViewTitleView.imageViewTitleViewDeleteBlock = { [weak self] (_, composeTypeItem) in
            self?.deleteAction(composeTypeItem: composeTypeItem)
        }
        return imageViewTitleView
    }
    
    private func initView() {
        self.backgroundColor = .white
        self.addSubview(self.typeImageViewTitleView)
        self.typeImageViewTitleView.topAnchor.constraint(equalTo: self.topAnchor, constant: 10).isActive = true
        self.typeImageViewTitleView.bottomAnchor.constraint(equalTo: self.bottomAnchor, constant: -10).isActive = true
        self.typeImageViewTitleView.leadingAnchor.constraint(equalTo: self.leadingAnchor, constant: 5).isActive = true
        self.typeWidthConstraint = self.typeImageViewTitleView.widthAnchor.constraint(lessThanOrEqualToConstant: 200)
        self.typeWidthConstraint?.isActive = true
        
        self.addSubview(self.tagImageViewTitleView)
        self.tagImageViewTitleView.topAnchor.constraint(equalTo: self.topAnchor, constant: 10).isActive = true
        self.tagImageViewTitleView.bottomAnchor.constraint(equalTo: self.bottomAnchor, constant: -10).isActive = true
        self.tagImageViewTitleView.leadingAnchor.constraint(equalTo: self.typeImageViewTitleView.trailingAnchor, constant: 10).isActive = true
        self.tagWidthConstraint = self.tagImageViewTitleView.widthAnchor.constraint(lessThanOrEqualToConstant: 200)
        self.tagWidthConstraint?.isActive = true
        
        self.addSubview(self.calendarImageViewTitleView)
        self.calendarImageViewTitleView.topAnchor.constraint(equalTo: self.topAnchor, constant: 10).isActive = true
        self.calendarImageViewTitleView.bottomAnchor.constraint(equalTo: self.bottomAnchor, constant: -10).isActive = true
        self.calendarImageViewTitleView.leadingAnchor.constraint(equalTo: self.tagImageViewTitleView.trailingAnchor, constant: 10).isActive = true
        self.calendarWidthConstraint = self.calendarImageViewTitleView.widthAnchor.constraint(lessThanOrEqualToConstant: 200)
        self.calendarWidthConstraint?.isActive = true
        
        self.addSubview(self.priorityImageViewTitleView)
        self.priorityImageViewTitleView.topAnchor.constraint(equalTo: self.topAnchor, constant: 10).isActive = true
        self.priorityImageViewTitleView.bottomAnchor.constraint(equalTo: self.bottomAnchor, constant: -10).isActive = true
        self.priorityImageViewTitleView.leadingAnchor.constraint(equalTo: self.calendarImageViewTitleView.trailingAnchor, constant: 10).isActive = true
        self.priorityWidthConstraint = self.priorityImageViewTitleView.widthAnchor.constraint(lessThanOrEqualToConstant: 60)
        self.priorityWidthConstraint?.isActive = true
    }

    @objc private func deleteAction(composeTypeItem: ComposeTypeItem) {
        self.selectedComposeViewDidSelectBlock?(self, composeTypeItem)
    }
}
