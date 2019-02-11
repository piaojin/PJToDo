//
//  PJToDoFileDelegate.swift
//  PJToDo
//
//  Created by piaojin on 2019/1/29.
//  Copyright © 2019 piaojin. All rights reserved.
//

import UIKit

public typealias PJTypeFileActionCompletedHandler = (_ isSuccess : Bool) -> Void

class PJToDoTypeFileDelegate {
    
    public var fileActionCompletedHandler: PJTypeFileActionCompletedHandler?
    
    public lazy var iDelegate: IPJToDoTypeFileDelegate = {
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
        
        let iDelegate = IPJToDoTypeFileDelegate(user: ownedPointer, destroy: destroyBlock, action_result: actionResultBlock)
        return iDelegate
    }()
    
    init(fileActionCompletedHandler: PJTypeFileActionCompletedHandler?) {
        self.fileActionCompletedHandler = fileActionCompletedHandler
    }
    
    //Rust回调Swift
    fileprivate func actionResult(isSuccess: Bool) {
        self.fileActionCompletedHandler?(isSuccess)
    }
}

//Rust回调Swift
fileprivate func actionResult(user: UnsafeMutableRawPointer, isSuccess: Bool) {
    let obj: PJToDoTypeFileDelegate = Unmanaged.fromOpaque(user).takeUnretainedValue()
    obj.actionResult(isSuccess: isSuccess)
    destroy(user: user)
}

//Rust回调Swift用以销毁Swift对象
fileprivate func destroy(user: UnsafeMutableRawPointer) {
    let _ = Unmanaged<PJToDoTypeFileDelegate>.fromOpaque(user).takeRetainedValue()
}
