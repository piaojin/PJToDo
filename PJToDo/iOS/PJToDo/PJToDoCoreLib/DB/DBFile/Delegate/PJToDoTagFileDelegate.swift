//
//  PJToDoTagFileDelegate.swift
//  PJToDo
//
//  Created by piaojin on 2019/1/30.
//  Copyright © 2019 piaojin. All rights reserved.
//

import UIKit

public typealias PJTagFileActionCompletedHandler = (_ isSuccess : Bool) -> Void

class PJToDoTagFileDelegate {
    
    public var fileActionCompletedHandler: PJTagFileActionCompletedHandler?
    
    public lazy var iDelegate: IPJToDoTagFileDelegate = {
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
        
        let iDelegate = IPJToDoTagFileDelegate(user: ownedPointer, destroy: destroyBlock, action_result: actionResultBlock)
        return iDelegate
    }()
    
    init(fileActionCompletedHandler: PJTagFileActionCompletedHandler?) {
        self.fileActionCompletedHandler = fileActionCompletedHandler
    }
    
    //Rust回调Swift
    fileprivate func actionResult(isSuccess: Bool) {
        self.fileActionCompletedHandler?(isSuccess)
    }
}

//Rust回调Swift
fileprivate func actionResult(user: UnsafeMutableRawPointer, isSuccess: Bool) {
    let obj: PJToDoTagFileDelegate = Unmanaged.fromOpaque(user).takeUnretainedValue()
    obj.actionResult(isSuccess: isSuccess)
    destroy(user: user)
}

//Rust回调Swift用以销毁Swift对象
fileprivate func destroy(user: UnsafeMutableRawPointer) {
    let _ = Unmanaged<PJToDoTagFileDelegate>.fromOpaque(user).takeRetainedValue()
}

