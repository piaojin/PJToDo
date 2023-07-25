//
//  DetailPriorityCell.swift
//  PJToDo
//
//  Created by piaojin on 2019/1/3.
//  Copyright © 2019 piaojin. All rights reserved.
//

import UIKit

class PriorityImageButton: UIButton {
    var priorityTag: Int = 0
}

class DetailPriorityCell: DetailItemCell {
    
    var lastSelectPriorityTag: Int = -1
    static let PriorityImageButtonBaseTag = 1000
    
    override var item: DetailItem {
        didSet {
            self.imageView?.image = UIImage(named: item.imageName)
            self.textLabel?.text = item.title
            if let priorityTag = Int(item.detailText) {
                if let view = self.contentView.viewWithTag(priorityTag + DetailPriorityCell.PriorityImageButtonBaseTag), let priorityImageButton = view as? PriorityImageButton {
                    priorityImageButton.backgroundColor = UIColor.colorWithRGB(red: 160, green: 160, blue: 160)
                }
                self.lastSelectPriorityTag = priorityTag
            }
        }
    }
    
    required override public init(style: UITableViewCell.CellStyle, reuseIdentifier: String?) {
        super.init(style: style, reuseIdentifier: reuseIdentifier)
        self.initView()
    }
    
    required public init?(coder aDecoder: NSCoder) {
        super.init(coder: aDecoder)
    }
    
    private func initView() {
        let priorityImageButton0 = self.createPriorityImageButton(imageNamed: "priority_0", priorityTag: 0)
        let priorityImageButton1 = self.createPriorityImageButton(imageNamed: "priority_1", priorityTag: 1)
        let priorityImageButton2 = self.createPriorityImageButton(imageNamed: "priority_2", priorityTag: 2)
        let priorityImageButton3 = self.createPriorityImageButton(imageNamed: "priority_3", priorityTag: 3)
        let priorityImageButton4 = self.createPriorityImageButton(imageNamed: "priority_4", priorityTag: 4)
        let priorityImageButton5 = self.createPriorityImageButton(imageNamed: "priority_5", priorityTag: 5)
        
        self.contentView.addSubview(priorityImageButton0)
        self.contentView.addSubview(priorityImageButton1)
        self.contentView.addSubview(priorityImageButton2)
        self.contentView.addSubview(priorityImageButton3)
        self.contentView.addSubview(priorityImageButton4)
        self.contentView.addSubview(priorityImageButton5)
        
        priorityImageButton5.widthAnchor.constraint(equalToConstant: 20).isActive = true
        priorityImageButton5.heightAnchor.constraint(equalToConstant: 20).isActive = true
        priorityImageButton5.trailingAnchor.constraint(equalTo: self.contentView.trailingAnchor, constant: -10).isActive = true
        priorityImageButton5.centerYAnchor.constraint(equalTo: self.contentView.centerYAnchor).isActive = true
        
        self.addLayoutXAxisAnchor(priorityImageButton: priorityImageButton4, referenceView: priorityImageButton5)
        
        self.addLayoutXAxisAnchor(priorityImageButton: priorityImageButton3, referenceView: priorityImageButton4)
        
        self.addLayoutXAxisAnchor(priorityImageButton: priorityImageButton2, referenceView: priorityImageButton3)
        
        self.addLayoutXAxisAnchor(priorityImageButton: priorityImageButton1, referenceView: priorityImageButton2)
        
        self.addLayoutXAxisAnchor(priorityImageButton: priorityImageButton0, referenceView: priorityImageButton1)
    }
    
    @objc private func clickAction(sender: UIButton) {
        var priorityTag: Int = -1
        if let priorityImageButton = sender as? PriorityImageButton {
            priorityImageButton.backgroundColor = UIColor.colorWithRGB(red: 160, green: 160, blue: 160)
            priorityTag = priorityImageButton.priorityTag
            self.imageView?.image = priorityImageButton.imageView?.image
        }
        
        if let view = self.contentView.viewWithTag(self.lastSelectPriorityTag + DetailPriorityCell.PriorityImageButtonBaseTag), let lastSelectPriorityImageButton = view as? PriorityImageButton {
            lastSelectPriorityImageButton.backgroundColor = .white
        }
        self.lastSelectPriorityTag = priorityTag
        self.item.detailText = "\(priorityTag)"
        self.item.value = priorityTag
    }
    
    private func createPriorityImageButton(imageNamed: String, priorityTag: Int) -> PriorityImageButton {
        let priorityImageButton = PriorityImageButton()
        priorityImageButton.priorityTag = priorityTag
        priorityImageButton.tag = priorityTag + DetailPriorityCell.PriorityImageButtonBaseTag
        priorityImageButton.layer.borderColor = UIColor.lightGray.cgColor
        priorityImageButton.setImage(UIImage(named: imageNamed), for: .normal)
        priorityImageButton.translatesAutoresizingMaskIntoConstraints = false
        priorityImageButton.backgroundColor = .white
        priorityImageButton.addTarget(self, action: #selector(clickAction(sender:)), for: .touchUpInside)
        return priorityImageButton
    }
    
    private func addLayoutXAxisAnchor(priorityImageButton: PriorityImageButton, referenceView: PriorityImageButton) {
        priorityImageButton.widthAnchor.constraint(equalToConstant: 20).isActive = true
        priorityImageButton.heightAnchor.constraint(equalToConstant: 20).isActive = true
        priorityImageButton.trailingAnchor.constraint(equalTo: referenceView.leadingAnchor).isActive = true
        priorityImageButton.centerYAnchor.constraint(equalTo: self.contentView.centerYAnchor).isActive = true
    }
}
