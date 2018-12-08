//
//  ToDoTypeController.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/10/5.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

public protocol ToDoTypeDelegate: NSObjectProtocol {
    func insertTypeResult(isSuccess: Bool)
    func deleteTypeResult(isSuccess: Bool)
    func updateTypeResult(isSuccess: Bool)
    func fetchTypeDataResult(isSuccess: Bool)
    func findTypeByIdResult(toDoType: PJToDoType?, isSuccess: Bool)
    func findTypeByNameResult(toDoType: PJToDoType?, isSuccess: Bool)
}

class ToDoTypeController {
    
    private lazy var controller: UnsafeMutablePointer<PJToDoTypeController>? = {
        let controller = createPJToDoTypeController(self.iDelegate)
        return controller
    }()
    
    private lazy var iDelegate: IPJToDoTypeDelegate = {
//        let ownedPointer = UnsafeMutableRawPointer(Unmanaged.passRetained(self).toOpaque())
        
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
        
        let iDelegate = IPJToDoTypeDelegate(user: nil, destroy: destroyBlock, insert_result: insertBackBlock, delete_result: deleteBackBlock, update_result: updateBackBlock, find_byId_result: findByIdBackBlock, find_byName_result: findByNameBackBlock, fetch_data_result: fetchDataBackBlock)
        return iDelegate
    }()
    
    public weak var delegate: ToDoTypeDelegate?
    
    init(delegate: ToDoTypeDelegate) {
        self.delegate = delegate
    }
    
    //插入数据成功后再更新数据到当前的PJToDoType对象
    public func insert(toDoType: PJToDoType) {
        let ownedPointer = ARCManager.retain(object: self)
        self.iDelegate.user = ownedPointer
        insertToDoType(self.controller, toDoType.iToDoTypeInsert)
    }
    
    public func delete(toDoTypeId: Int32) {
        let ownedPointer = ARCManager.retain(object: self)
        self.iDelegate.user = ownedPointer
        deleteToDoType(self.controller, toDoTypeId)
    }
    
    public func update(toDoType: PJToDoType) {
        let ownedPointer = ARCManager.retain(object: self)
        self.iDelegate.user = ownedPointer
        updateToDoType(self.controller, toDoType.iToDoType)
    }
    
    public func findById(toDoTypeId: Int32) {
        let ownedPointer = ARCManager.retain(object: self)
        self.iDelegate.user = ownedPointer
        findToDoType(self.controller, toDoTypeId)
    }
    
    public func findByName(typeName: String) {
        let ownedPointer = ARCManager.retain(object: self)
        self.iDelegate.user = ownedPointer
        findToDoTypeByName(self.controller, typeName)
    }
    
    public func fetchData() {
        let ownedPointer = ARCManager.retain(object: self)
        self.iDelegate.user = ownedPointer
        fetchToDoTypeData(self.controller)
    }
    
    public func getCount() -> Int32 {
        return getToDoTypeCount(self.controller)
    }
    
    public func toDoTypeAt(index: Int32) -> PJToDoType {
        return PJToDoType(iToDoType: todoTypeAtIndex(self.controller, index))
    }
    
    //Rust回调Swift
    fileprivate func insertResult(isSuccess: Bool) {
        print("ToDoTypeController: received insertResult callback with  \(isSuccess)")
        self.delegate?.insertTypeResult(isSuccess: isSuccess)
    }
    
    fileprivate func deleteResult(isSuccess: Bool) {
        print("ToDoTypeController: received deleteResult callback with  \(isSuccess)")
        self.delegate?.deleteTypeResult(isSuccess: isSuccess)
    }
    
    fileprivate func updateResult(isSuccess: Bool) {
        print("ToDoTypeController: received updateResult callback with  \(isSuccess)")
        self.delegate?.updateTypeResult(isSuccess: isSuccess)
    }
    
    fileprivate func findByIdResult(toDoType: OpaquePointer?, isSuccess: Bool) {
        print("ToDoTypeController: received findByIdResult callback with  \(isSuccess)")
        let tempToDoType = isSuccess ? PJToDoType(iToDoType: toDoType) : nil
        self.delegate?.findTypeByIdResult(toDoType: tempToDoType, isSuccess: isSuccess)
    }
    
    fileprivate func findByNameResult(toDoType: OpaquePointer?, isSuccess: Bool) {
        print("ToDoTypeController: received findByIdResult callback with  \(isSuccess)")
        let tempToDoType = isSuccess ? PJToDoType(iToDoType: toDoType) : nil
        self.delegate?.findTypeByNameResult(toDoType: tempToDoType, isSuccess: isSuccess)
    }
    
    fileprivate func fetchDataResult(isSuccess: Bool) {
        print("ToDoTypeController: received fetchDataResult callback with  \(isSuccess)")
        self.delegate?.fetchTypeDataResult(isSuccess: isSuccess)
    }
    
    deinit {
        print("deinit -> ToDoTypeController")
        free_rust_PJToDoTypeController(self.controller)
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
