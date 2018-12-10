//
//  MineController.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/10/5.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

public protocol ToDoSettingsDelegate: NSObjectProtocol {
    func insertSettingsResult(isSuccess: Bool)
    func deleteSettingsResult(isSuccess: Bool)
    func updateSettingsResult(isSuccess: Bool)
    func fetchSettingsDataResult(mySettings: PJMySettings?, isSuccess: Bool)
}

class MineController {
    
    private lazy var controller: UnsafeMutablePointer<PJToDoSettingsController>? = {
        let controller = createPJToDoSettingsController(self.iDelegate)
        return controller
    }()
    
    private lazy var iDelegate: IPJToDoSettingsDelegate = {
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
        
        let fetchDataBackBlock: (@convention(c) (UnsafeMutableRawPointer?, Bool) -> Void)! = { (pointer, isSuccess) in
            if let tempPointer = pointer {
                PJToDo.fetchDataResult(user: tempPointer, isSuccess: isSuccess)
            }
        }
        
        let iDelegate = IPJToDoSettingsDelegate(user: ownedPointer, destroy: destroyBlock, insert_result: insertBackBlock, delete_result: deleteBackBlock, update_result: updateBackBlock, fetch_data_result: fetchDataBackBlock)
        return iDelegate
    }()
    
    public weak var delegate: ToDoSettingsDelegate?
    
    init(delegate: ToDoSettingsDelegate) {
        self.delegate = delegate
    }
    
    public func insert(toDoSettings: PJMySettings) {
        let ownedPointer = ARCManager.retain(object: self)
        self.iDelegate.user = ownedPointer
        insertToDoSettings(self.controller, toDoSettings.iToDoSettingsInsert)
    }
    
    public func delete(toDoSettingsId: Int32) {
        let ownedPointer = ARCManager.retain(object: self)
        self.iDelegate.user = ownedPointer
        deleteToDoSettings(self.controller, toDoSettingsId)
    }
    
    public func update(toDoSettings: PJMySettings) {
        let ownedPointer = ARCManager.retain(object: self)
        self.iDelegate.user = ownedPointer
        updateToDoSettings(self.controller, toDoSettings.iToDoSettings)
    }
    
    public func fetchData() {
        let ownedPointer = ARCManager.retain(object: self)
        self.iDelegate.user = ownedPointer
        fetchToDoSettingsData(self.controller)
    }
    
    public func getCount() -> Int32 {
        return getToDoSettingsCount(self.controller)
    }
    
    public func toDoSettingsAt(index: Int32) -> PJMySettings {
        return PJMySettings(iToDoSettings: todoSettingsAtIndex(self.controller, index))
    }
    
    //Rust回调Swift
    fileprivate func insertResult(isSuccess: Bool) {
        print("MineController: received insertResult callback with  \(isSuccess)")
        self.delegate?.insertSettingsResult(isSuccess: isSuccess)
    }
    
    fileprivate func deleteResult(isSuccess: Bool) {
        print("MineController: received deleteResult callback with  \(isSuccess)")
        self.delegate?.deleteSettingsResult(isSuccess: isSuccess)
    }
    
    fileprivate func updateResult(isSuccess: Bool) {
        print("MineController: received updateResult callback with  \(isSuccess)")
        self.delegate?.updateSettingsResult(isSuccess: isSuccess)
    }
    
    fileprivate func fetchDataResult(isSuccess: Bool) {
        print("MineController: received fetchDataResult callback with  \(isSuccess)")
        let mySettings: PJMySettings? = isSuccess ? self.toDoSettingsAt(index: 0) : nil
        self.delegate?.fetchSettingsDataResult(mySettings: mySettings, isSuccess: isSuccess)
    }
    
    deinit {
        print("deinit -> MineController")
        free_rust_PJToDoSettingsController(self.controller)
    }
}

//Rust回调Swift
fileprivate func insertResult(user: UnsafeMutableRawPointer, isSuccess: Bool) {
    let obj: MineController = Unmanaged.fromOpaque(user).takeUnretainedValue()
    obj.insertResult(isSuccess: isSuccess)
}

fileprivate func deleteResult(user: UnsafeMutableRawPointer, isSuccess: Bool) {
    let obj: MineController = Unmanaged.fromOpaque(user).takeUnretainedValue()
    obj.deleteResult(isSuccess: isSuccess)
}

fileprivate func updateResult(user: UnsafeMutableRawPointer, isSuccess: Bool) {
    let obj: MineController = Unmanaged.fromOpaque(user).takeUnretainedValue()
    obj.updateResult(isSuccess: isSuccess)
}

fileprivate func fetchDataResult(user: UnsafeMutableRawPointer, isSuccess: Bool) {
    let obj: MineController = Unmanaged.fromOpaque(user).takeUnretainedValue()
    obj.fetchDataResult(isSuccess: isSuccess)
}

//Rust回调Swift用以销毁Swift对象
fileprivate func destroy(user: UnsafeMutableRawPointer) {
    let _ = Unmanaged<MineController>.fromOpaque(user).takeRetainedValue()
}
