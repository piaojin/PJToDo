//
//  TextItem.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/12/8.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

class TextItem {
    var id: Int = -1
    var text: String = ""
    
    init(id: Int, text: String) {
        self.id = id
        self.text = text
    }
}
