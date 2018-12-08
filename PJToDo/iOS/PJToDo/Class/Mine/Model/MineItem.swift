//
//  MineItem.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/12/6.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

enum MineItemType {
    case type
    case tag
    case email
    case remindDays
    case blog
    case about
}

class MineItem {
    var imageName: String = ""
    var title: String = ""
    var detailText: String = ""
    var type: MineItemType = .type
    
    init(imageName: String, title: String, detailText: String, type: MineItemType) {
        self.imageName = imageName
        self.title = title
        self.detailText = detailText
        self.type = type
    }
}
