//
//  PJToDoSettingsFileDelegate.swift
//  PJToDo
//
//  Created by piaojin on 2019/1/30.
//  Copyright © 2019 piaojin. All rights reserved.
//

import UIKit

public typealias PJSettingsFileActionCompletedHandler = (_ isSuccess : Bool) -> Void

class PJToDoSettingsFileDelegate {
    
    public var fileActionCompletedHandler: PJSettingsFileActionCompletedHandler?
    
    public lazy var iDelegate: IPJToDoSettingsFileDelegate = {
        let ownedPointer = UnsafeMutableRawPointer(Unmanaged.passRetained(self).toOpaque())
        
        /*call back for C*/
        let destroyBlock: @convention(c) (UnsafeMutableRawPointer?) -> Void = {(pointer) in
            if let tempPointer = pointer {
                destroy(user: tempPointer)
            }
        }
        
        let actionResultBlock: (@convention(c) (UnsafeMutableRawPointer?, Bool) -> Void)! = { (pointer, isSuccess) in
            if let tempPointer = pointer {
                PJToDo.actionResult(user: tempPointer, isSuccess: isSuccess)
            }
        }
        
        let iDelegate = IPJToDoSettingsFileDelegate(user: ownedPointer, destroy: destroyBlock, action_result: actionResultBlock)
        return iDelegate
    }()
    
    init(fileActionCompletedHandler: PJSettingsFileActionCompletedHandler?) {
        self.fileActionCompletedHandler = fileActionCompletedHandler
    }
    
    //Rust回调Swift
    fileprivate func actionResult(isSuccess: Bool) {
        self.fileActionCompletedHandler?(isSuccess)
    }
}

//Rust回调Swift
fileprivate func actionResult(user: UnsafeMutableRawPointer, isSuccess: Bool) {
    let obj: PJToDoSettingsFileDelegate = Unmanaged.fromOpaque(user).takeUnretainedValue()
    obj.actionResult(isSuccess: isSuccess)
    destroy(user: user)
}

//Rust回调Swift用以销毁Swift对象
fileprivate func destroy(user: UnsafeMutableRawPointer) {
    let _ = Unmanaged<PJToDoSettingsFileDelegate>.fromOpaque(user).takeRetainedValue()
}

