//
//  ToDoTypeController.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/10/5.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

public protocol ToDoTypeDelegate: NSObjectProtocol {
    func insertResult(isSuccess: Bool)
    func deleteResult(isSuccess: Bool)
    func updateResult(isSuccess: Bool)
    func findByIdResult(toDoType: PJToDoType, isSuccess: Bool)
    func findByNameResult(toDoType: PJToDoType, isSuccess: Bool)
}

class ToDoTypeController {
    
    private lazy var controller: UnsafeMutablePointer<PJToDoTypeController>? = {
        let controller = createPJToDoTypeController(self.iDelegate)
        return controller
    }()
    
    private lazy var iDelegate: IPJToDoTypeDelegate = {
        let ownedPointer = UnsafeMutableRawPointer(Unmanaged.passRetained(self).toOpaque())
        
        /*call back for C*/
        let destroyBlock: @convention(c) (UnsafeMutableRawPointer?) -> Void = {(pointer) in
            if let tempPointer = pointer {
                destroy(user: tempPointer)
            }
        }
        
        let insertBackBlock: (@convention(c) (UnsafeMutableRawPointer?, Bool) -> Void)! = { (pointer, isSuccess) in
            if let tempPointer = pointer {
                PJToDo.insertResult(user: tempPointer, isSuccess: isSuccess)
            }
        }
        
        let deleteBackBlock: (@convention(c) (UnsafeMutableRawPointer?, Bool) -> Void)! = { (pointer, isSuccess) in
            if let tempPointer = pointer {
                PJToDo.deleteResult(user: tempPointer, isSuccess: isSuccess)
            }
        }
        
        let updateBackBlock: (@convention(c) (UnsafeMutableRawPointer?, Bool) -> Void)! = { (pointer, isSuccess) in
            if let tempPointer = pointer {
                PJToDo.updateResult(user: tempPointer, isSuccess: isSuccess)
            }
        }
        
        let findByIdBackBlock: (@convention(c) (UnsafeMutableRawPointer?, OpaquePointer?, Bool) -> Void)! = { (pointer, toDoTypePointer, isSuccess) in
            if let tempPointer = pointer {
                PJToDo.findByIdResult(user: tempPointer, toDoType: toDoTypePointer, isSuccess: isSuccess)
            }
        }
        
        let findByNameBackBlock: (@convention(c) (UnsafeMutableRawPointer?, OpaquePointer?, Bool) -> Void)! = { (pointer, toDoTypePointer, isSuccess) in
            if let tempPointer = pointer {
                PJToDo.findByNameResult(user: tempPointer, toDoType: toDoTypePointer, isSuccess: isSuccess)
            }
        }
        
        let iDelegate = IPJToDoTypeDelegate(user: ownedPointer, destroy: destroyBlock, insert_result: insertBackBlock, delete_result: deleteBackBlock, update_result: updateBackBlock, find_byId_result: findByIdBackBlock, find_byName_result: findByNameBackBlock)
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
    
    public func delete(toDoTypeId: Int32) {
        deleteToDoType(self.controller, toDoTypeId)
    }
    
    public func update(toDoType: PJToDoType) {
        updateToDoType(self.controller, toDoType.iToDoType)
    }
    
    public func findById(toDoTypeId: Int32) {
        findToDoType(self.controller, toDoTypeId)
    }
    
    public func findByName(typeName: String) {
        findToDoTypeByName(self.controller, typeName)
    }
    
    public func setTypeName(name: String) {
        setToDoTypeTypeName(self.toDoType.iToDoType, name)
    }
    
    //Rust回调Swift
    fileprivate func insertResult(isSuccess: Bool) {
        print("ToDoTypeController: received insertResult callback with  \(isSuccess)")
        self.delegate?.insertResult(isSuccess: isSuccess)
    }
    
    fileprivate func deleteResult(isSuccess: Bool) {
        print("ToDoTypeController: received deleteResult callback with  \(isSuccess)")
        self.delegate?.deleteResult(isSuccess: isSuccess)
    }
    
    fileprivate func updateResult(isSuccess: Bool) {
        print("ToDoTypeController: received updateResult callback with  \(isSuccess)")
        self.delegate?.updateResult(isSuccess: isSuccess)
    }
    
    fileprivate func findByIdResult(toDoType: OpaquePointer?, isSuccess: Bool) {
        print("ToDoTypeController: received findByIdResult callback with  \(isSuccess)")
        let tempToDoType = PJToDoType(iToDoType: toDoType)
        self.delegate?.findByIdResult(toDoType: tempToDoType, isSuccess: isSuccess)
    }
    
    fileprivate func findByNameResult(toDoType: OpaquePointer?, isSuccess: Bool) {
        print("ToDoTypeController: received findByIdResult callback with  \(isSuccess)")
        let tempToDoType = PJToDoType(iToDoType: toDoType)
        self.delegate?.findByNameResult(toDoType: tempToDoType, isSuccess: isSuccess)
    }
    
    deinit {
        print("deinit -> ToDoTypeController")
        free_rust_PJToDoTypeController(self.controller)
    }
    
    func free() {
        
    }
}

//Rust回调Swift
fileprivate func insertResult(user: UnsafeMutableRawPointer, isSuccess: Bool) {
    let obj: ToDoTypeController = Unmanaged.fromOpaque(user).takeUnretainedValue()
    obj.insertResult(isSuccess: isSuccess)
}

fileprivate func deleteResult(user: UnsafeMutableRawPointer, isSuccess: Bool) {
    let obj: ToDoTypeController = Unmanaged.fromOpaque(user).takeUnretainedValue()
    obj.deleteResult(isSuccess: isSuccess)
}

fileprivate func updateResult(user: UnsafeMutableRawPointer, isSuccess: Bool) {
    let obj: ToDoTypeController = Unmanaged.fromOpaque(user).takeUnretainedValue()
    obj.updateResult(isSuccess: isSuccess)
}

fileprivate func findByIdResult(user: UnsafeMutableRawPointer, toDoType: OpaquePointer?, isSuccess: Bool) {
    let obj: ToDoTypeController = Unmanaged.fromOpaque(user).takeUnretainedValue()
    obj.findByIdResult(toDoType: toDoType, isSuccess: isSuccess)
}

fileprivate func findByNameResult(user: UnsafeMutableRawPointer, toDoType: OpaquePointer?, isSuccess: Bool) {
    let obj: ToDoTypeController = Unmanaged.fromOpaque(user).takeUnretainedValue()
    obj.findByNameResult(toDoType: toDoType, isSuccess: isSuccess)
}

//Rust回调Swift用以销毁Swift对象
fileprivate func destroy(user: UnsafeMutableRawPointer) {
    let _ = Unmanaged<ToDoTypeController>.fromOpaque(user).takeRetainedValue()
}
