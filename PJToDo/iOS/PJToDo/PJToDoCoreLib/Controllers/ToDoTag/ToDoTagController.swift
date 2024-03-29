//
//  ToDoTagController.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/10/5.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit
import CocoaLumberjack

@objc public protocol ToDoTagDelegate: NSObjectProtocol {
    @objc optional func insertTagResult(isSuccess: Bool)
    @objc optional func deleteTagResult(isSuccess: Bool)
    @objc optional func updateTagResult(isSuccess: Bool)
    func fetchTagDataResult(isSuccess: Bool)
    @objc optional func findTagByIdResult(toDoTag: PJToDoTag?, isSuccess: Bool)
    @objc optional func findTagByNameResult(toDoTag: PJToDoTag?, isSuccess: Bool)
}

class ToDoTagController {
    
    private lazy var controller: UnsafeMutablePointer<PJToDoTagController>? = {
        let controller = pj_create_PJToDoTagController(self.iDelegate)
        return controller
    }()
    
    private lazy var iDelegate: IPJToDoTagDelegate = {
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
        let ownedPointer = PJARCManager.retain(object: self)
        self.iDelegate.user = ownedPointer
        pj_insert_todo_tag(self.controller, toDoTag.iToDoTagInsert)
    }
    
    public func delete(toDoTagId: Int32) {
        let ownedPointer = PJARCManager.retain(object: self)
        self.iDelegate.user = ownedPointer
        pj_delete_todo_tag(self.controller, toDoTagId)
    }
    
    public func update(toDoTag: PJToDoTag) {
        let ownedPointer = PJARCManager.retain(object: self)
        self.iDelegate.user = ownedPointer
        pj_update_todo_tag(self.controller, toDoTag.iToDoTag)
    }
    
    public func findById(toDoTagId: Int32) {
        let ownedPointer = PJARCManager.retain(object: self)
        self.iDelegate.user = ownedPointer
        pj_find_todo_tag(self.controller, toDoTagId)
    }
    
    public func findByName(tagName: String) {
        let ownedPointer = PJARCManager.retain(object: self)
        self.iDelegate.user = ownedPointer
        pj_find_todo_tag_by_name(self.controller, tagName)
    }
    
    public func fetchData() {
        let ownedPointer = PJARCManager.retain(object: self)
        self.iDelegate.user = ownedPointer
        pj_fetch_todo_tag_data(self.controller)
    }
    
    public func getCount() -> Int32 {
        return pj_get_todo_tag_count(self.controller)
    }
    
    public func toDoTagAt(index: Int32) -> PJToDoTag {
        return PJToDoTag(iToDoTag: pj_todo_tag_at_index(self.controller, index))
    }
    
    //Rust回调Swift
    fileprivate func insertResult(isSuccess: Bool) {
        DDLogInfo("ToDoTagController: received insertResult callback with  \(isSuccess)")
        self.delegate?.insertTagResult?(isSuccess: isSuccess)
    }
    
    fileprivate func deleteResult(isSuccess: Bool) {
        DDLogInfo("ToDoTagController: received deleteResult callback with  \(isSuccess)")
        self.delegate?.deleteTagResult?(isSuccess: isSuccess)
    }
    
    fileprivate func updateResult(isSuccess: Bool) {
        DDLogInfo("ToDoTagController: received updateResult callback with  \(isSuccess)")
        self.delegate?.updateTagResult?(isSuccess: isSuccess)
    }
    
    fileprivate func findByIdResult(toDoTag: OpaquePointer?, isSuccess: Bool) {
        DDLogInfo("ToDoTagController: received findByIdResult callback with  \(isSuccess)")
        let tempToDoTag = isSuccess ? PJToDoTag(iToDoTag: toDoTag) : nil
        self.delegate?.findTagByIdResult?(toDoTag: tempToDoTag, isSuccess: isSuccess)
    }
    
    fileprivate func findByNameResult(toDoTag: OpaquePointer?, isSuccess: Bool) {
        DDLogInfo("ToDoTagController: received findByIdResult callback with  \(isSuccess)")
        let tempToDoTag = isSuccess ? PJToDoTag(iToDoTag: toDoTag) : nil
        self.delegate?.findTagByNameResult?(toDoTag: tempToDoTag, isSuccess: isSuccess)
    }
    
    fileprivate func fetchDataResult(isSuccess: Bool) {
        DDLogInfo("ToDoTagController: received fetchDataResult callback with  \(isSuccess)")
        self.delegate?.fetchTagDataResult(isSuccess: isSuccess)
    }
    
    deinit {
        DDLogInfo("deinit -> ToDoTagController")
        pj_free_rust_PJToDoTagController(self.controller)
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
