//
//  ToDoTypeController.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/10/5.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

@objc public protocol ToDoTypeDelegate: NSObjectProtocol {
    @objc optional func insertTypeResult(isSuccess: Bool)
    @objc optional func deleteTypeResult(isSuccess: Bool)
    @objc optional func updateTypeResult(isSuccess: Bool)
    func fetchTypeDataResult(isSuccess: Bool)
    @objc optional func findTypeByIdResult(toDoType: PJToDoType?, isSuccess: Bool)
    @objc optional func findTypeByNameResult(toDoType: PJToDoType?, isSuccess: Bool)
}

class ToDoTypeController {
    
    private lazy var controller: UnsafeMutablePointer<PJToDoTypeController>? = {
        let controller = pj_create_PJToDoTypeController(self.iDelegate)
        return controller
    }()
    
    private lazy var iDelegate: IPJToDoTypeDelegate = {
        let ownedPointer = UnsafeMutableRawPointer(Unmanaged.passUnretained(self).toOpaque())
        
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
        
        let fetchDataBackBlock: (@convention(c) (UnsafeMutableRawPointer?, Bool) -> Void)! = { (pointer, isSuccess) in
            if let tempPointer = pointer {
                PJToDo.fetchDataResult(user: tempPointer, isSuccess: isSuccess)
            }
        }
        
        let iDelegate = IPJToDoTypeDelegate(user: ownedPointer, destroy: destroyBlock, insert_result: insertBackBlock, delete_result: deleteBackBlock, update_result: updateBackBlock, find_byId_result: findByIdBackBlock, find_byName_result: findByNameBackBlock, fetch_data_result: fetchDataBackBlock)
        return iDelegate
    }()
    
    public weak var delegate: ToDoTypeDelegate?
    
    init(delegate: ToDoTypeDelegate) {
        self.delegate = delegate
    }
    
    //插入数据成功后再更新数据到当前的PJToDoType对象
    public func insert(toDoType: PJToDoType) {
        let ownedPointer = PJARCManager.retain(object: self)
        self.iDelegate.user = ownedPointer
        pj_insert_todo_type(self.controller, toDoType.iToDoTypeInsert)
    }
    
    public func delete(toDoTypeId: Int32) {
        let ownedPointer = PJARCManager.retain(object: self)
        self.iDelegate.user = ownedPointer
        pj_delete_todo_type(self.controller, toDoTypeId)
    }
    
    public func update(toDoType: PJToDoType) {
        let ownedPointer = PJARCManager.retain(object: self)
        self.iDelegate.user = ownedPointer
        pj_update_todo_type(self.controller, toDoType.iToDoType)
    }
    
    public func findById(toDoTypeId: Int32) {
        let ownedPointer = PJARCManager.retain(object: self)
        self.iDelegate.user = ownedPointer
        pj_find_todo_type(self.controller, toDoTypeId)
    }
    
    public func findByName(typeName: String) {
        let ownedPointer = PJARCManager.retain(object: self)
        self.iDelegate.user = ownedPointer
        pj_find_todo_type_by_name(self.controller, typeName)
    }
    
    public func fetchData() {
        let ownedPointer = PJARCManager.retain(object: self)
        self.iDelegate.user = ownedPointer
        pj_fetch_todo_type_data(self.controller)
    }
    
    public func getCount() -> Int32 {
        return pj_get_todo_type_count(self.controller)
    }
    
    public func toDoTypeAt(index: Int32) -> PJToDoType {
        return PJToDoType(iToDoType: pj_todo_type_at_index(self.controller, index))
    }
    
    //Rust回调Swift
    fileprivate func insertResult(isSuccess: Bool) {
        print("ToDoTypeController: received insertResult callback with  \(isSuccess)")
        self.delegate?.insertTypeResult?(isSuccess: isSuccess)
    }
    
    fileprivate func deleteResult(isSuccess: Bool) {
        print("ToDoTypeController: received deleteResult callback with  \(isSuccess)")
        self.delegate?.deleteTypeResult?(isSuccess: isSuccess)
    }
    
    fileprivate func updateResult(isSuccess: Bool) {
        print("ToDoTypeController: received updateResult callback with  \(isSuccess)")
        self.delegate?.updateTypeResult?(isSuccess: isSuccess)
    }
    
    fileprivate func findByIdResult(toDoType: OpaquePointer?, isSuccess: Bool) {
        print("ToDoTypeController: received findByIdResult callback with  \(isSuccess)")
        let tempToDoType = isSuccess ? PJToDoType(iToDoType: toDoType) : nil
        self.delegate?.findTypeByIdResult?(toDoType: tempToDoType, isSuccess: isSuccess)
    }
    
    fileprivate func findByNameResult(toDoType: OpaquePointer?, isSuccess: Bool) {
        print("ToDoTypeController: received findByIdResult callback with  \(isSuccess)")
        let tempToDoType = isSuccess ? PJToDoType(iToDoType: toDoType) : nil
        self.delegate?.findTypeByNameResult?(toDoType: tempToDoType, isSuccess: isSuccess)
    }
    
    fileprivate func fetchDataResult(isSuccess: Bool) {
        print("ToDoTypeController: received fetchDataResult callback with  \(isSuccess)")
        self.delegate?.fetchTypeDataResult(isSuccess: isSuccess)
    }
    
    deinit {
        print("deinit -> ToDoTypeController")
        pj_free_rust_PJToDoTypeController(self.controller)
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

fileprivate func fetchDataResult(user: UnsafeMutableRawPointer, isSuccess: Bool) {
    let obj: ToDoTypeController = Unmanaged.fromOpaque(user).takeUnretainedValue()
    obj.fetchDataResult(isSuccess: isSuccess)
}

//Rust回调Swift用以销毁Swift对象
fileprivate func destroy(user: UnsafeMutableRawPointer) {
    let _ = Unmanaged<ToDoTypeController>.fromOpaque(user).takeRetainedValue()
}
