//
//  ToDoSyncGitHubDataController.swift
//  PJToDo
//
//  Created by piaojin on 2019/2/23.
//  Copyright © 2019 piaojin. All rights reserved.
//

import UIKit

@objc public protocol ToDoSyncGitHubDataDelegate: NSObjectProtocol {
    @objc optional func syncGitHubToDoDataResult(isSuccess: Bool)
    @objc optional func syncGitHubTypeDataResult(isSuccess: Bool)
    @objc optional func syncGitHubTagDataResult(isSuccess: Bool)
    @objc optional func syncGitHubSettingsDataResult(isSuccess: Bool)
}

class ToDoSyncGitHubDataController {
    
    private lazy var controller: UnsafeMutablePointer<PJToDoSyncGitHubDataController>? = {
        let controller = createPJToDoSyncGitHubDataController(self.iDelegate)
        return controller
    }()
    
    private lazy var iDelegate: IPJToDoSyncGitHubDataDelegate = {
        let ownedPointer = UnsafeMutableRawPointer(Unmanaged.passUnretained(self).toOpaque())
        
        /*call back for C*/
        let destroyBlock: @convention(c) (UnsafeMutableRawPointer?) -> Void = {(pointer) in
            if let tempPointer = pointer {
                destroy(user: tempPointer)
            }
        }
        
        let syncGitHubToDoDataBackBlock: (@convention(c) (UnsafeMutableRawPointer?, Bool) -> Void)! = { (pointer, isSuccess) in
            if let tempPointer = pointer {
                PJToDo.syncGitHubToDoDataResult(user: tempPointer, isSuccess: isSuccess)
            }
        }
        
        let syncGitHubTypeDataBackBlock: (@convention(c) (UnsafeMutableRawPointer?, Bool) -> Void)! = { (pointer, isSuccess) in
            if let tempPointer = pointer {
                PJToDo.syncGitHubTypeDataResult(user: tempPointer, isSuccess: isSuccess)
            }
        }
        
        let syncGitHubTagDataBackBlock: (@convention(c) (UnsafeMutableRawPointer?, Bool) -> Void)! = { (pointer, isSuccess) in
            if let tempPointer = pointer {
                PJToDo.syncGitHubTagDataResult(user: tempPointer, isSuccess: isSuccess)
            }
        }
        
        let syncGitHubSettingsDataBackBlock: (@convention(c) (UnsafeMutableRawPointer?, Bool) -> Void)! = { (pointer, isSuccess) in
            if let tempPointer = pointer {
                PJToDo.syncGitHubSettingsDataResult(user: tempPointer, isSuccess: isSuccess)
            }
        }
        
        let iDelegate = IPJToDoSyncGitHubDataDelegate(user: ownedPointer, destroy: destroyBlock, sync_todo_data_result: syncGitHubToDoDataBackBlock, sync_type_data_result: syncGitHubTypeDataBackBlock, sync_tag_data_result: syncGitHubTagDataBackBlock, sync_settings_data_result: syncGitHubSettingsDataBackBlock)
        
        return iDelegate
    }()
    
    public weak var delegate: ToDoSyncGitHubDataDelegate?
    
    init(delegate: ToDoSyncGitHubDataDelegate) {
        self.delegate = delegate
    }
    
    public func syncGitHubToDoDataResult(filePath: String) {
        let ownedPointer = PJARCManager.retain(object: self)
        self.iDelegate.user = ownedPointer
        syncGitHubToDoData(self.controller, filePath)
    }
    
    public func syncGitHubTypeDataResult(filePath: String) {
        let ownedPointer = PJARCManager.retain(object: self)
        self.iDelegate.user = ownedPointer
        syncGitHubTypeData(self.controller, filePath)
    }
    
    public func syncGitHubTagDataResult(filePath: String) {
        let ownedPointer = PJARCManager.retain(object: self)
        self.iDelegate.user = ownedPointer
        syncGitHubTagData(self.controller, filePath)
    }
    
    public func syncGitHubSettingsDataResult(filePath: String) {
        let ownedPointer = PJARCManager.retain(object: self)
        self.iDelegate.user = ownedPointer
        syncGitHubSettingsData(self.controller, filePath)
    }
    
    fileprivate func syncGitHubTypeDataResult(isSuccess: Bool) {
        self.delegate?.syncGitHubTypeDataResult?(isSuccess: isSuccess)
    }
    
    fileprivate func syncGitHubTagDataResult(isSuccess: Bool) {
        self.delegate?.syncGitHubTagDataResult?(isSuccess: isSuccess)
    }
    
    fileprivate func syncGitHubToDoDataResult(isSuccess: Bool) {
        self.delegate?.syncGitHubToDoDataResult?(isSuccess: isSuccess)
    }
    
    fileprivate func syncGitHubSettingsDataResult(isSuccess: Bool) {
        self.delegate?.syncGitHubSettingsDataResult?(isSuccess: isSuccess)
    }
    
    deinit {
        print("deinit -> ToDoTypeController")
        free_rust_PJToDoSyncGitHubDataController(self.controller)
    }
}

fileprivate func syncGitHubToDoDataResult(user: UnsafeMutableRawPointer, isSuccess: Bool) {
    let obj: ToDoSyncGitHubDataController = Unmanaged.fromOpaque(user).takeUnretainedValue()
    obj.syncGitHubToDoDataResult(isSuccess: isSuccess)
}

fileprivate func syncGitHubTypeDataResult(user: UnsafeMutableRawPointer, isSuccess: Bool) {
    let obj: ToDoSyncGitHubDataController = Unmanaged.fromOpaque(user).takeUnretainedValue()
    obj.syncGitHubTypeDataResult(isSuccess: isSuccess)
}

fileprivate func syncGitHubTagDataResult(user: UnsafeMutableRawPointer, isSuccess: Bool) {
    let obj: ToDoSyncGitHubDataController = Unmanaged.fromOpaque(user).takeUnretainedValue()
    obj.syncGitHubTagDataResult(isSuccess: isSuccess)
}

fileprivate func syncGitHubSettingsDataResult(user: UnsafeMutableRawPointer, isSuccess: Bool) {
    let obj: ToDoSyncGitHubDataController = Unmanaged.fromOpaque(user).takeUnretainedValue()
    obj.syncGitHubSettingsDataResult(isSuccess: isSuccess)
}

//Rust回调Swift用以销毁Swift对象
fileprivate func destroy(user: UnsafeMutableRawPointer) {
    let _ = Unmanaged<ToDoSyncGitHubDataController>.fromOpaque(user).takeRetainedValue()
}

