//
//  ToDoController.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/10/5.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import Foundation

public protocol ToDoDelegate: NSObjectProtocol {
    func insertToDoResult(isSuccess: Bool)
    func deleteToDoResult(isSuccess: Bool)
    func updateToDoResult(isSuccess: Bool)
    func fetchToDoDataResult(isSuccess: Bool)
    func fetchToDoLikeTitleResult(isSuccess: Bool)
    func findToDoByIdResult(toDo: PJ_ToDo, isSuccess: Bool)
    func findToDoByTitleResult(toDo: PJ_ToDo, isSuccess: Bool)
    func fetchToDoDateFutureDayMoreThanResult(isSuccess: Bool)
    func fetchTodosOrderByStateResult(isSuccess: Bool)
}

class ToDoController {
    private lazy var controller: UnsafeMutablePointer<PJToDoController>? = {
        let controller = createPJToDoController(self.iDelegate)
        return controller
    }()
    
    private lazy var iDelegate: IPJToDoDelegate = {
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
        
        let findByIdBackBlock: (@convention(c) (UnsafeMutableRawPointer?, OpaquePointer?, Bool) -> Void)! = { (pointer, toDoPointer, isSuccess) in
            if let tempPointer = pointer {
                PJToDo.findByIdResult(user: tempPointer, toDo: toDoPointer, isSuccess: isSuccess)
            }
        }
        
        let findByTitleBackBlock: (@convention(c) (UnsafeMutableRawPointer?, OpaquePointer?, Bool) -> Void)! = { (pointer, toDoPointer, isSuccess) in
            if let tempPointer = pointer {
                PJToDo.findByTitleResult(user: tempPointer, toDo: toDoPointer, isSuccess: isSuccess)
            }
        }
        
        let fetchDataBackBlock: (@convention(c) (UnsafeMutableRawPointer?, Bool) -> Void)! = { (pointer, isSuccess) in
            if let tempPointer = pointer {
                PJToDo.fetchDataResult(user: tempPointer, isSuccess: isSuccess)
            }
        }
        
        let fetchToDoLikeTitleBackBlock: (@convention(c) (UnsafeMutableRawPointer?, Bool) -> Void)! = { (pointer, isSuccess) in
            if let tempPointer = pointer {
                PJToDo.fetchToDoLikeTitleResult(user: tempPointer, isSuccess: isSuccess)
            }
        }
        
        let fetchToDoDateFutureDayMoreThanBackBlock: (@convention(c) (UnsafeMutableRawPointer?, Bool) -> Void)! = { (pointer, isSuccess) in
            if let tempPointer = pointer {
                PJToDo.fetchToDoDateFutureDayMoreThanResult(user: tempPointer, isSuccess: isSuccess)
            }
        }
        
        let fetchTodosOrderByStateBackBlock: (@convention(c) (UnsafeMutableRawPointer?, Bool) -> Void)! = { (pointer, isSuccess) in
            if let tempPointer = pointer {
                PJToDo.fetchTodosOrderByStateResult(user: tempPointer, isSuccess: isSuccess)
            }
        }
        
        let iDelegate = IPJToDoDelegate(user: ownedPointer, destroy: destroyBlock, insert_result: insertBackBlock, delete_result: deleteBackBlock, update_result: updateBackBlock, find_byId_result: findByIdBackBlock, find_byTitle_result: findByTitleBackBlock, fetch_data_result: fetchDataBackBlock, find_byLike_result: fetchToDoLikeTitleBackBlock, todo_date_future_day_more_than_result: fetchToDoDateFutureDayMoreThanBackBlock, fetch_todos_order_by_state_result: fetchTodosOrderByStateBackBlock)
        return iDelegate
    }()
    
    public weak var delegate: ToDoDelegate?
    
    init(delegate: ToDoDelegate) {
        self.delegate = delegate
    }
    
    //插入数据成功后再更新数据到当前的PJToDo对象
    public func insert(toDo: PJ_ToDo) {
        insertToDo(self.controller, toDo.iToDoInsert)
    }
    
    public func delete(toDoId: Int32) {
        deleteToDo(self.controller, toDoId)
    }
    
    public func update(toDo: PJ_ToDo) {
        updateToDo(self.controller, toDo.iToDoQuery)
    }
    
    public func findById(toDoId: Int32) {
        findToDo(self.controller, toDoId)
    }
    
    public func findByTitle(title: String) {
        findToDoByTitle(self.controller, title)
    }
    
    public func fetchData() {
        fetchToDoData(self.controller)
    }
    
    public func getToDoCountAtSection(section: Int32) -> Int32 {
        return getToDoCountsAtSection(self.controller, section)
    }
    
    public func toDoAt(section: Int32, index: Int32) -> PJ_ToDo {
        let iToDoQuery = todoAtSection(self.controller, section, index)
        let typeId = getToDoQuery_ToDoTypeId(iToDoQuery)
        let tagId = getToDoQuery_ToDoTagId(iToDoQuery)
        let iToDoType = toDoTypeWithId(self.controller, typeId)
        let iToDoTag = toDoTagWithId(self.controller, tagId)
        return PJ_ToDo(iToDoQuery: iToDoQuery, iToDoType: iToDoType, iToDoTag: iToDoTag)
    }
    
    //Rust回调Swift
    fileprivate func insertResult(isSuccess: Bool) {
        print("ToDoTypeController: received insertResult callback with  \(isSuccess)")
        self.delegate?.insertToDoResult(isSuccess: isSuccess)
    }
    
    fileprivate func deleteResult(isSuccess: Bool) {
        print("ToDoTypeController: received deleteResult callback with  \(isSuccess)")
        self.delegate?.deleteToDoResult(isSuccess: isSuccess)
    }
    
    fileprivate func updateResult(isSuccess: Bool) {
        print("ToDoTypeController: received updateResult callback with  \(isSuccess)")
        self.delegate?.updateToDoResult(isSuccess: isSuccess)
    }
    
    fileprivate func findByIdResult(toDo: OpaquePointer?, isSuccess: Bool) {
        print("ToDoTypeController: received findByIdResult callback with  \(isSuccess)")
        let typeId = getToDoQuery_ToDoTypeId(toDo)
        let tagId = getToDoQuery_ToDoTagId(toDo)
        let iToDoType = toDoTypeWithId(self.controller, typeId)
        let iToDoTag = toDoTagWithId(self.controller, tagId)
        let tempToDo = PJ_ToDo(iToDoQuery: toDo, iToDoType: iToDoType, iToDoTag: iToDoTag)
        self.delegate?.findToDoByIdResult(toDo: tempToDo, isSuccess: isSuccess)
    }
    
    fileprivate func findByTitleResult(toDo: OpaquePointer?, isSuccess: Bool) {
        print("ToDoTypeController: received findByIdResult callback with  \(isSuccess)")
        let typeId = getToDoQuery_ToDoTypeId(toDo)
        let tagId = getToDoQuery_ToDoTagId(toDo)
        let iToDoType = toDoTypeWithId(self.controller, typeId)
        let iToDoTag = toDoTagWithId(self.controller, tagId)
        let tempToDo = PJ_ToDo(iToDoQuery: toDo, iToDoType: iToDoType, iToDoTag: iToDoTag)
        self.delegate?.findToDoByTitleResult(toDo: tempToDo, isSuccess: isSuccess)
    }
    
    fileprivate func fetchDataResult(isSuccess: Bool) {
        print("ToDoTypeController: received fetchDataResult callback with  \(isSuccess)")
        self.delegate?.fetchToDoDataResult(isSuccess: isSuccess)
    }
    
    fileprivate func fetchToDoLikeTitleResult(isSuccess: Bool) {
        print("ToDoTypeController: received fetchDataResult callback with  \(isSuccess)")
        self.delegate?.fetchToDoLikeTitleResult(isSuccess: isSuccess)
    }
    
    fileprivate func fetchToDoDateFutureDayMoreThanResult(isSuccess: Bool) {
        print("ToDoTypeController: received fetchDataResult callback with  \(isSuccess)")
        self.delegate?.fetchToDoDateFutureDayMoreThanResult(isSuccess: isSuccess)
    }
    
    fileprivate func fetchTodosOrderByStateResult(isSuccess: Bool) {
        print("ToDoTypeController: received fetchDataResult callback with  \(isSuccess)")
        self.delegate?.fetchTodosOrderByStateResult(isSuccess: isSuccess)
    }
    
    deinit {
        print("deinit -> ToDoTypeController")
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

fileprivate func findByTitleResult(user: UnsafeMutableRawPointer, toDo: OpaquePointer?, isSuccess: Bool) {
    let obj: ToDoController = Unmanaged.fromOpaque(user).takeUnretainedValue()
    obj.findByTitleResult(toDo: toDo, isSuccess: isSuccess)
}

fileprivate func fetchDataResult(user: UnsafeMutableRawPointer, isSuccess: Bool) {
    let obj: ToDoController = Unmanaged.fromOpaque(user).takeUnretainedValue()
    obj.fetchDataResult(isSuccess: isSuccess)
}

fileprivate func fetchToDoLikeTitleResult(user: UnsafeMutableRawPointer, isSuccess: Bool) {
    let obj: ToDoController = Unmanaged.fromOpaque(user).takeUnretainedValue()
    obj.fetchToDoLikeTitleResult(isSuccess: isSuccess)
}

fileprivate func fetchToDoDateFutureDayMoreThanResult(user: UnsafeMutableRawPointer, isSuccess: Bool) {
    let obj: ToDoController = Unmanaged.fromOpaque(user).takeUnretainedValue()
    obj.fetchToDoDateFutureDayMoreThanResult(isSuccess: isSuccess)
}

fileprivate func fetchTodosOrderByStateResult(user: UnsafeMutableRawPointer, isSuccess: Bool) {
    let obj: ToDoController = Unmanaged.fromOpaque(user).takeUnretainedValue()
    obj.fetchTodosOrderByStateResult(isSuccess: isSuccess)
}

//Rust回调Swift用以销毁Swift对象
fileprivate func destroy(user: UnsafeMutableRawPointer) {
    let _ = Unmanaged<ToDoController>.fromOpaque(user).takeRetainedValue()
}

