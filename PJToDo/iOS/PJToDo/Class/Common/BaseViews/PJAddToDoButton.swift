//
//  PJAddToDoButton.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/11/30.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit
import CYLTabBarController

class PJAddToDoButton: CYLPlusButton, CYLPlusButtonSubclassing {
    static func plusButton() -> Any! {
        let addButton = PJAddToDoButton()
        addButton.backgroundColor = UIColor.colorWithRGB(red: 0, green: 123, blue: 249)
        addButton.setImage(UIImage(named: "add"), for: .normal)
        return addButton
    }
    
    static func indexOfPlusButtonInTabBar() -> UInt {
        return 1
    }
}
