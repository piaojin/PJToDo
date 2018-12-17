//
//  ComposeTypeItem.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/12/9.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

enum ComposeType {
    case type
    case tag
    case priority
}

class ComposeTypeItem: NSObject {
    var id: Int = -1
    var title: String = ""
    var imageNamed: String = ""
    var composeType: ComposeType = .type
    
    init(id: Int, title: String, imageNamed: String , composeType: ComposeType) {
        super.init()
        self.id = id
        self.title = title
        self.imageNamed = imageNamed
        self.composeType = composeType
    }
}
