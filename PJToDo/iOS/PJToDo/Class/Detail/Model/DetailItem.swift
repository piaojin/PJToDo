//
//  DetailItem.swift
//  PJToDo
//
//  Created by piaojin on 2019/1/2.
//  Copyright © 2019 piaojin. All rights reserved.
//

import UIKit

enum DetailItemType {
    case type
    case tag
    case dueTime
    case remindTime
    case title
    case content
    case priority
}

class DetailItem: NSObject {
    var imageName: String = ""
    var title: String = ""
    var detailText: String = ""
    var type: DetailItemType = .type
    
    init(imageName: String, title: String, detailText: String, type: DetailItemType) {
        self.imageName = imageName
        self.title = title
        self.detailText = detailText
        self.type = type
    }
}
