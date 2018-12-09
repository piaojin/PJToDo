//
//  ComposeTypeCell.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/12/9.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

class ComposeTypeCell: UITableViewCell {

    var composeTypeItem: ComposeTypeItem? {
        didSet {
            if let item = composeTypeItem {
                self.textLabel?.text = item.title
                if item.composeType == .priority {
                    self.imageView?.image = UIImage(named: "priority_\(item.id)")
                } else {
                    self.imageView?.image = nil
                }
            }
        }
    }
    
    required override public init(style: UITableViewCellStyle, reuseIdentifier: String?) {
        super.init(style: style, reuseIdentifier: reuseIdentifier)
    }
    
    required public init?(coder aDecoder: NSCoder) {
        super.init(coder: aDecoder)
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
