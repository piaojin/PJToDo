//
//  DetailValueCell.swift
//  PJToDo
//
//  Created by piaojin on 2019/1/2.
//  Copyright © 2019 piaojin. All rights reserved.
//

import UIKit

class DetailValueCell: DetailItemCell {

    var valueLabel: UILabel = {
        let valueLabel = UILabel()
        valueLabel.translatesAutoresizingMaskIntoConstraints = false
        valueLabel.textAlignment = .right
        valueLabel.textColor = UIColor.colorWithRGB(red: 159, green: 159, blue: 162)
        return valueLabel
    }()
    
    override var item: DetailItem {
        didSet {
            self.imageView?.image = UIImage(named: item.imageName)
            self.textLabel?.text = item.title
            self.valueLabel.text = item.detailText
            if item.type == .type || item.type == .tag {
                self.accessoryType = .disclosureIndicator
            } else {
                self.accessoryType = .none
            }
        }
    }
    
    required override public init(style: UITableViewCellStyle, reuseIdentifier: String?) {
        super.init(style: style, reuseIdentifier: reuseIdentifier)
        self.initView()
    }
    
    required public init?(coder aDecoder: NSCoder) {
        super.init(coder: aDecoder)
    }
    
    private func initView() {
        self.contentView.addSubview(self.valueLabel)
        self.valueLabel.topAnchor.constraint(equalTo: self.contentView.topAnchor, constant: 10).isActive = true
        self.valueLabel.bottomAnchor.constraint(equalTo: self.contentView.bottomAnchor, constant: -10).isActive = true
        self.valueLabel.trailingAnchor.constraint(equalTo: self.accessoryView?.leadingAnchor ?? self.contentView.trailingAnchor, constant: -10).isActive = true
    }
}
