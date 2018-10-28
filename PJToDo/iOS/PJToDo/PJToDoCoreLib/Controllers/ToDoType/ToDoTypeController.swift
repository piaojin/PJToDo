//
//  ToDoTypeController.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/10/5.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

public protocol ToDoTypeDelegate: NSObjectProtocol {
    func insertResult(_id: Int, isSuccess: Bool)
}

class ToDoTypeController {
    
    private lazy var controller: UnsafeMutablePointer<PJToDoTypeController>? = {
        let controller = createPJToDoTypeController(self.iDelegate)
        return controller
    }()
    
    private lazy var iDelegate: IPJToDoTypeDelegate = {
        let ownedPointer = UnsafeMutableRawPointer(Unmanaged.passRetained(self).toOpaque())
        
        let destroyBlock: @convention(c) (UnsafeMutableRawPointer?) -> Void = {(pointer) in
            destroy(user: pointer!)
        }
        
        let callBackBlock: (@convention(c) (UnsafeMutableRawPointer?, Int32, Bool) -> Void)! = { (pointer, _id, isSuccess) in
            PJToDo.insertResult(user: pointer!, _id: _id, isSuccess: isSuccess)
        }
        
        let iDelegate = IPJToDoTypeDelegate(user: ownedPointer, destroy: destroyBlock, insert_result: callBackBlock)
        return iDelegate
    }()
    
    public var toDoType: PJToDoType = PJToDoType(typeName: "")
    
    public weak var delegate: ToDoTypeDelegate?
    
    init(delegate: ToDoTypeDelegate) {
        self.delegate = delegate
    }
    
    //插入数据成功后再更新数据到当前的PJToDoType对象
    public func insert(toDoType: PJToDoType) {
        insertToDoType(self.controller, toDoType.iToDoTypeInsert)
    }
    
    public func setTypeName(name: String) {
        setToDoTypeTypeName(self.toDoType.iToDoType, name)
    }
    
    //Rust回调Swift
    fileprivate func insertResult(_id: Int32, isSuccess: Bool) {
        print("ToDoTypeController: received callback with id \(_id)")
        self.delegate?.insertResult(_id: Int(_id), isSuccess: isSuccess)
    }
    
    deinit {
        print("deinit -> ToDoTypeController")
//        free_rust_object(self.controller)
        free_rust_PJToDoTypeController(self.controller)
    }
    
    func free() {
        
    }
}

//Rust回调Swift
fileprivate func insertResult(user: UnsafeMutableRawPointer, _id: Int32, isSuccess: Bool) {
    let obj: ToDoTypeController = Unmanaged.fromOpaque(user).takeUnretainedValue()
    obj.insertResult(_id: _id, isSuccess: isSuccess)
}

//Rust回调Swift用以销毁Swift对象
fileprivate func destroy(user: UnsafeMutableRawPointer) {
    let _ = Unmanaged<ToDoTypeController>.fromOpaque(user).takeRetainedValue()
}
