//
//  ToDoSearchController.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/10/5.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit
import CocoaLumberjack

public protocol ToDoSearchDelegate: NSObjectProtocol {
    func findToDoLikeTitleResult(isSuccess: Bool)
    func findToDoByTitleResult(toDo: PJ_ToDo?, isSuccess: Bool)
}

class ToDoSearchController {
    private lazy var controller: UnsafeMutablePointer<PJToDoSearchController>? = {
        let controller = pj_create_PJToDoSearchController(self.iDelegate)
        return controller
    }()
    
    private lazy var iDelegate: IPJToDoSearchDelegate = {
         let ownedPointer = UnsafeMutableRawPointer(Unmanaged.passUnretained(self).toOpaque())
        
        /*call back for C*/
        let destroyBlock: @convention(c) (UnsafeMutableRawPointer?) -> Void = {(pointer) in
            if let tempPointer = pointer {
                destroy(user: tempPointer)
            }
        }
        
        let findByTitleBackBlock: (@convention(c) (UnsafeMutableRawPointer?, OpaquePointer?, Bool) -> Void)! = { (pointer, toDoPointer, isSuccess) in
            if let tempPointer = pointer {
                PJToDo.findByTitleResult(user: tempPointer, toDo: toDoPointer, isSuccess: isSuccess)
            }
        }
        
        let findToDoLikeTitleBackBlock: (@convention(c) (UnsafeMutableRawPointer?, Bool) -> Void)! = { (pointer, isSuccess) in
            if let tempPointer = pointer {
                PJToDo.findToDoLikeTitleResult(user: tempPointer, isSuccess: isSuccess)
            }
        }
        
        let iDelegate = IPJToDoSearchDelegate(user: ownedPointer, destroy: destroyBlock, find_byTitle_result: findByTitleBackBlock, find_byLike_result: findToDoLikeTitleBackBlock)
        return iDelegate
    }()
    
    public weak var delegate: ToDoSearchDelegate?
    
    init(delegate: ToDoSearchDelegate) {
        self.delegate = delegate
    }
    
    public func findByTitle(title: String) {
        let ownedPointer = PJARCManager.retain(object: self)
        self.iDelegate.user = ownedPointer
        pj_find_todo_by_title(self.controller, title)
    }
    
    public func findToDoLikeTitle(title: String) {
        let ownedPointer = PJARCManager.retain(object: self)
        self.iDelegate.user = ownedPointer
        pj_find_todo_like_title(self.controller, title)
    }
    
    public func getSearchToDoResultCount() -> Int32 {
        return pj_search_todo_result_count(self.controller)
    }
    
    public func searchToDoResultAtIndex(index: Int32) -> PJ_ToDo {
        let iToDoQuery = pj_search_todo_result_at_index(self.controller, index)
        let typeId = pj_get_todo_query_todo_type_id(iToDoQuery)
        let tagId = pj_get_todo_query_todo_tag_id(iToDoQuery)
        let iToDoType = pj_get_search_todo_type_with_id(self.controller, typeId)
        let iToDoTag = pj_get_search_todo_tag_with_id(self.controller, tagId)
        return PJ_ToDo(iToDoQuery: iToDoQuery, iToDoType: iToDoType, iToDoTag: iToDoTag)
    }
    
    //Rust回调Swift
    
    public func findByTitleResult(toDo: OpaquePointer?, isSuccess: Bool) {
        DDLogInfo("ToDoSearchController: received findByIdResult callback with  \(isSuccess)")
        var tempToDo: PJ_ToDo? = nil
        if isSuccess {
            let typeId = pj_get_todo_query_todo_type_id(toDo)
            let tagId = pj_get_todo_query_todo_tag_id(toDo)
            let iToDoType = pj_get_search_todo_type_with_id(self.controller, typeId)
            let iToDoTag = pj_get_search_todo_tag_with_id(self.controller, tagId)
            tempToDo = PJ_ToDo(iToDoQuery: toDo, iToDoType: iToDoType, iToDoTag: iToDoTag)
        }
        self.delegate?.findToDoByTitleResult(toDo: tempToDo, isSuccess: isSuccess)
    }
    
    public func findToDoLikeTitleResult(isSuccess: Bool) {
        DDLogInfo("ToDoSearchController: received fetchDataResult callback with  \(isSuccess)")
        self.delegate?.findToDoLikeTitleResult(isSuccess: isSuccess)
    }
    
    deinit {
        DDLogInfo("deinit -> ToDoSearchController")
        pj_free_rust_PJToDoSearchController(self.controller)
    }
}

//Rust回调Swift
fileprivate func findByTitleResult(user: UnsafeMutableRawPointer, toDo: OpaquePointer?, isSuccess: Bool) {
    let obj: ToDoSearchController = Unmanaged.fromOpaque(user).takeUnretainedValue()
    obj.findByTitleResult(toDo: toDo, isSuccess: isSuccess)
}

fileprivate func findToDoLikeTitleResult(user: UnsafeMutableRawPointer, isSuccess: Bool) {
    let obj: ToDoSearchController = Unmanaged.fromOpaque(user).takeUnretainedValue()
    obj.findToDoLikeTitleResult(isSuccess: isSuccess)
}

//Rust回调Swift用以销毁Swift对象
fileprivate func destroy(user: UnsafeMutableRawPointer) {
    let _ = Unmanaged<ToDoSearchController>.fromOpaque(user).takeRetainedValue()
}
