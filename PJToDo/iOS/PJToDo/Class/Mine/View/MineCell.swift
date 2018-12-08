//
//  MineCell.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/12/6.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

class MineCell: UITableViewCell {

    lazy var typeImageView: UIImageView = {
        let typeImageView = UIImageView()
        typeImageView.translatesAutoresizingMaskIntoConstraints = false
        return typeImageView
    }()
    
    var item: MineItem? {
        didSet {
            if let tempItem = item {
                self.imageView?.image = UIImage(named: tempItem.imageName)
                self.textLabel?.text = tempItem.title
                self.detailTextLabel?.text = tempItem.detailText
            }
        }
    }
    
    required override public init(style: UITableViewCellStyle, reuseIdentifier: String?) {
        super.init(style: .subtitle, reuseIdentifier: reuseIdentifier)
        self.initView()
    }
    
    required public init?(coder aDecoder: NSCoder) {
        super.init(coder: aDecoder)
    }
    
    private func initView() {
        self.accessoryType = .disclosureIndicator
    }
    
    override func awakeFromNib() {
        super.awakeFromNib()
        // Initialization code
    }

    override func setSelected(_ selected: Bool, animated: Bool) {
        super.setSelected(selected, animated: animated)

        // Configure the view for the selected state
    }

}
