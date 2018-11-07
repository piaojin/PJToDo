//
//  ToDoTagController.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/10/5.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

public protocol ToDoTagDelegate: NSObjectProtocol {
    func insertTagResult(isSuccess: Bool)
    func deleteTagResult(isSuccess: Bool)
    func updateTagResult(isSuccess: Bool)
    func fetchTagDataResult(isSuccess: Bool)
    func findTagByIdResult(toDoTag: PJToDoTag, isSuccess: Bool)
    func findTagByNameResult(toDoTag: PJToDoTag, isSuccess: Bool)
}

class ToDoTagController {
    
    private lazy var controller: UnsafeMutablePointer<PJToDoTagController>? = {
        let controller = createPJToDoTagController(self.iDelegate)
        return controller
    }()
    
    private lazy var iDelegate: IPJToDoTagDelegate = {
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
        
        let findByIdBackBlock: (@convention(c) (UnsafeMutableRawPointer?, OpaquePointer?, Bool) -> Void)! = { (pointer, toDoTagPointer, isSuccess) in
            if let tempPointer = pointer {
                PJToDo.findByIdResult(user: tempPointer, toDoTag: toDoTagPointer, isSuccess: isSuccess)
            }
        }
        
        let findByNameBackBlock: (@convention(c) (UnsafeMutableRawPointer?, OpaquePointer?, Bool) -> Void)! = { (pointer, toDoTagPointer, isSuccess) in
            if let tempPointer = pointer {
                PJToDo.findByNameResult(user: tempPointer, toDoTag: toDoTagPointer, isSuccess: isSuccess)
            }
        }
        
        let fetchDataBackBlock: (@convention(c) (UnsafeMutableRawPointer?, Bool) -> Void)! = { (pointer, isSuccess) in
            if let tempPointer = pointer {
                PJToDo.fetchDataResult(user: tempPointer, isSuccess: isSuccess)
            }
        }
        
        let iDelegate = IPJToDoTagDelegate(user: ownedPointer, destroy: destroyBlock, insert_result: insertBackBlock, delete_result: deleteBackBlock, update_result: updateBackBlock, find_byId_result: findByIdBackBlock, find_byName_result: findByNameBackBlock, fetch_data_result: fetchDataBackBlock)
        return iDelegate
    }()
    
    public weak var delegate: ToDoTagDelegate?
    
    init(delegate: ToDoTagDelegate) {
        self.delegate = delegate
    }
    
    //插入数据成功后再更新数据到当前的PJToDoTag对象
    public func insert(toDoTag: PJToDoTag) {
        insertToDoTag(self.controller, toDoTag.iToDoTagInsert)
    }
    
    public func delete(toDoTagId: Int32) {
        deleteToDoTag(self.controller, toDoTagId)
    }
    
    public func update(toDoTag: PJToDoTag) {
        updateToDoTag(self.controller, toDoTag.iToDoTag)
    }
    
    public func findById(toDoTagId: Int32) {
        findToDoTag(self.controller, toDoTagId)
    }
    
    public func findByName(tagName: String) {
        findToDoTagByName(self.controller, tagName)
    }
    
    public func fetchData() {
        fetchToDoTagData(self.controller)
    }
    
    public func getCount() -> Int32 {
        return getToDoTagCount(self.controller)
    }
    
    public func toDoTagAt(index: Int32) -> PJToDoTag {
        return PJToDoTag(iToDoTag: todoTagAtIndex(self.controller, index))
    }
    
    //Rust回调Swift
    fileprivate func insertResult(isSuccess: Bool) {
        print("ToDoTagController: received insertResult callback with  \(isSuccess)")
        self.delegate?.insertTagResult(isSuccess: isSuccess)
    }
    
    fileprivate func deleteResult(isSuccess: Bool) {
        print("ToDoTagController: received deleteResult callback with  \(isSuccess)")
        self.delegate?.deleteTagResult(isSuccess: isSuccess)
    }
    
    fileprivate func updateResult(isSuccess: Bool) {
        print("ToDoTagController: received updateResult callback with  \(isSuccess)")
        self.delegate?.updateTagResult(isSuccess: isSuccess)
    }
    
    fileprivate func findByIdResult(toDoTag: OpaquePointer?, isSuccess: Bool) {
        print("ToDoTagController: received findByIdResult callback with  \(isSuccess)")
        let tempToDoTag = PJToDoTag(iToDoTag: toDoTag)
        self.delegate?.findTagByIdResult(toDoTag: tempToDoTag, isSuccess: isSuccess)
    }
    
    fileprivate func findByNameResult(toDoTag: OpaquePointer?, isSuccess: Bool) {
        print("ToDoTagController: received findByIdResult callback with  \(isSuccess)")
        let tempToDoTag = PJToDoTag(iToDoTag: toDoTag)
        self.delegate?.findTagByNameResult(toDoTag: tempToDoTag, isSuccess: isSuccess)
    }
    
    fileprivate func fetchDataResult(isSuccess: Bool) {
        print("ToDoTagController: received fetchDataResult callback with  \(isSuccess)")
        self.delegate?.fetchTagDataResult(isSuccess: isSuccess)
    }
    
    deinit {
        print("deinit -> ToDoTagController")
        free_rust_PJToDoTagController(self.controller)
    }
}

//Rust回调Swift
fileprivate func insertResult(user: UnsafeMutableRawPointer, isSuccess: Bool) {
    let obj: ToDoTagController = Unmanaged.fromOpaque(user).takeUnretainedValue()
    obj.insertResult(isSuccess: isSuccess)
}

fileprivate func deleteResult(user: UnsafeMutableRawPointer, isSuccess: Bool) {
    let obj: ToDoTagController = Unmanaged.fromOpaque(user).takeUnretainedValue()
    obj.deleteResult(isSuccess: isSuccess)
}

fileprivate func updateResult(user: UnsafeMutableRawPointer, isSuccess: Bool) {
    let obj: ToDoTagController = Unmanaged.fromOpaque(user).takeUnretainedValue()
    obj.updateResult(isSuccess: isSuccess)
}

fileprivate func findByIdResult(user: UnsafeMutableRawPointer, toDoTag: OpaquePointer?, isSuccess: Bool) {
    let obj: ToDoTagController = Unmanaged.fromOpaque(user).takeUnretainedValue()
    obj.findByIdResult(toDoTag: toDoTag, isSuccess: isSuccess)
}

fileprivate func findByNameResult(user: UnsafeMutableRawPointer, toDoTag: OpaquePointer?, isSuccess: Bool) {
    let obj: ToDoTagController = Unmanaged.fromOpaque(user).takeUnretainedValue()
    obj.findByNameResult(toDoTag: toDoTag, isSuccess: isSuccess)
}

fileprivate func fetchDataResult(user: UnsafeMutableRawPointer, isSuccess: Bool) {
    let obj: ToDoTagController = Unmanaged.fromOpaque(user).takeUnretainedValue()
    obj.fetchDataResult(isSuccess: isSuccess)
}

//Rust回调Swift用以销毁Swift对象
fileprivate func destroy(user: UnsafeMutableRawPointer) {
    let _ = Unmanaged<ToDoTagController>.fromOpaque(user).takeRetainedValue()
}
