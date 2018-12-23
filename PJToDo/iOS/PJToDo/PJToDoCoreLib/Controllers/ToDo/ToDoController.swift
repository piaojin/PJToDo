//
//  ToDoController.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/10/5.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import Foundation

@objc public protocol ToDoDelegate: NSObjectProtocol {
    @objc optional func insertToDoResult(isSuccess: Bool)
    @objc optional func deleteToDoResult(isSuccess: Bool)
    @objc optional func updateToDoResult(isSuccess: Bool)
    func fetchToDoDataResult(isSuccess: Bool)
    @objc optional func findToDoByIdResult(toDo: PJ_ToDo?, isSuccess: Bool)
}

class ToDoController {
    private lazy var controller: UnsafeMutablePointer<PJToDoController>? = {
        let controller = createPJToDoController(self.iDelegate)
        return controller
    }()
    
    private lazy var iDelegate: IPJToDoDelegate = {
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
        
        let findByIdBackBlock: (@convention(c) (UnsafeMutableRawPointer?, OpaquePointer?, Bool) -> Void)! = { (pointer, toDoPointer, isSuccess) in
            if let tempPointer = pointer {
                PJToDo.findByIdResult(user: tempPointer, toDo: toDoPointer, isSuccess: isSuccess)
            }
        }
        
        let fetchDataBackBlock: (@convention(c) (UnsafeMutableRawPointer?, Bool) -> Void)! = { (pointer, isSuccess) in
            if let tempPointer = pointer {
                PJToDo.fetchDataResult(user: tempPointer, isSuccess: isSuccess)
            }
        }
        
        let iDelegate = IPJToDoDelegate(user: ownedPointer, destroy: destroyBlock, insert_result: insertBackBlock, delete_result: deleteBackBlock, update_result: updateBackBlock, find_byId_result: findByIdBackBlock, fetch_data_result: fetchDataBackBlock)
        return iDelegate
    }()
    
    public weak var delegate: ToDoDelegate?
    
    init(delegate: ToDoDelegate) {
        self.delegate = delegate
    }
    
    //插入数据成功后再更新数据到当前的PJToDo对象
    public func insert(toDo: PJ_ToDo) {
        let ownedPointer = ARCManager.retain(object: self)
        self.iDelegate.user = ownedPointer
        insertToDo(self.controller, toDo.iToDoInsert)
    }
    
    public func delete(toDoId: Int) {
        let ownedPointer = ARCManager.retain(object: self)
        self.iDelegate.user = ownedPointer
        deleteToDo(self.controller, Int32(toDoId))
    }
    
    public func update(toDo: PJ_ToDo) {
        let ownedPointer = ARCManager.retain(object: self)
        self.iDelegate.user = ownedPointer
        updateToDo(self.controller, toDo.iToDoQuery)
    }
    
    public func findById(toDoId: Int) {
        let ownedPointer = ARCManager.retain(object: self)
        self.iDelegate.user = ownedPointer
        findToDo(self.controller, Int32(toDoId))
    }
    
    public func fetchData() {
        let ownedPointer = ARCManager.retain(object: self)
        self.iDelegate.user = ownedPointer
        fetchToDoData(self.controller)
    }
    
    public func getToDoCountAtSection(section: Int) -> Int {
        return Int(getToDoCountsAtSection(self.controller, Int32(section)))
    }
    
    public func toDoAt(section: Int, index: Int) -> PJ_ToDo {
        let iToDoQuery = todoAtSection(self.controller, Int32(section), Int32(index))
        let typeId = getToDoQuery_ToDoTypeId(iToDoQuery)
        let tagId = getToDoQuery_ToDoTagId(iToDoQuery)
        let iToDoType = toDoTypeWithId(self.controller, typeId)
        let iToDoTag = toDoTagWithId(self.controller, tagId)
        return PJ_ToDo(iToDoQuery: iToDoQuery, iToDoType: iToDoType, iToDoTag: iToDoTag)
    }
    
    public func toDoTitle(section: Int) -> String {
        return String.create(cString: todoTitleAtSection(self.controller, Int32(section)))
    }
    
    //Rust回调Swift
    fileprivate func insertResult(isSuccess: Bool) {
        print("ToDoController: received insertResult callback with  \(isSuccess)")
        self.delegate?.insertToDoResult?(isSuccess: isSuccess)
    }
    
    fileprivate func deleteResult(isSuccess: Bool) {
        print("ToDoController: received deleteResult callback with  \(isSuccess)")
        self.delegate?.deleteToDoResult?(isSuccess: isSuccess)
    }
    
    fileprivate func updateResult(isSuccess: Bool) {
        print("ToDoController: received updateResult callback with  \(isSuccess)")
        self.delegate?.updateToDoResult?(isSuccess: isSuccess)
    }
    
    fileprivate func findByIdResult(toDo: OpaquePointer?, isSuccess: Bool) {
        print("ToDoController: received findByIdResult callback with  \(isSuccess)")
        var tempToDo: PJ_ToDo? = nil
        if isSuccess {
            let typeId = getToDoQuery_ToDoTypeId(toDo)
            let tagId = getToDoQuery_ToDoTagId(toDo)
            let iToDoType = toDoTypeWithId(self.controller, typeId)
            let iToDoTag = toDoTagWithId(self.controller, tagId)
            tempToDo = PJ_ToDo(iToDoQuery: toDo, iToDoType: iToDoType, iToDoTag: iToDoTag)
        }
        self.delegate?.findToDoByIdResult?(toDo: tempToDo, isSuccess: isSuccess)
    }
    
    fileprivate func fetchDataResult(isSuccess: Bool) {
        print("ToDoController: received fetchDataResult callback with  \(isSuccess)")
        self.delegate?.fetchToDoDataResult(isSuccess: isSuccess)
    }
    
    deinit {
        print("deinit -> ToDoController")
        free_rust_PJToDoController(self.controller)
    }
}

//Rust回调Swift
fileprivate func insertResult(user: UnsafeMutableRawPointer, isSuccess: Bool) {
    let obj: ToDoController = Unmanaged.fromOpaque(user).takeUnretainedValue()
    obj.insertResult(isSuccess: isSuccess)
}

fileprivate func deleteResult(user: UnsafeMutableRawPointer, isSuccess: Bool) {
    let obj: ToDoController = Unmanaged.fromOpaque(user).takeUnretainedValue()
    obj.deleteResult(isSuccess: isSuccess)
}

fileprivate func updateResult(user: UnsafeMutableRawPointer, isSuccess: Bool) {
    let obj: ToDoController = Unmanaged.fromOpaque(user).takeUnretainedValue()
    obj.updateResult(isSuccess: isSuccess)
}

fileprivate func findByIdResult(user: UnsafeMutableRawPointer, toDo: OpaquePointer?, isSuccess: Bool) {
    let obj: ToDoController = Unmanaged.fromOpaque(user).takeUnretainedValue()
    obj.findByIdResult(toDo: toDo, isSuccess: isSuccess)
}

fileprivate func fetchDataResult(user: UnsafeMutableRawPointer, isSuccess: Bool) {
    let obj: ToDoController = Unmanaged.fromOpaque(user).takeUnretainedValue()
    obj.fetchDataResult(isSuccess: isSuccess)
}

//Rust回调Swift用以销毁Swift对象
fileprivate func destroy(user: UnsafeMutableRawPointer) {
    let _ = Unmanaged<ToDoController>.fromOpaque(user).takeRetainedValue()
}

