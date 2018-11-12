//
//  ToDoModelProtocol.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/11/11.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import Foundation

public protocol ToDoModelProtocol: NSObjectProtocol {
//    associatedtype ObjectType
    func transformData(pointer: UnsafeMutableRawPointer) -> Self
}
