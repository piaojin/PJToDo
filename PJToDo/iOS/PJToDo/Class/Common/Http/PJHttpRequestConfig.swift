//
//  PJHttpRequestConfig.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/11/12.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import Foundation

public typealias PJResponseBlock = (_ data: UnsafePointer<Int8>?, _ isSuccess : Bool) -> Void

class PJHttpRequestConfig {
    
    public var responseBlock: PJResponseBlock?
    
    public lazy var iDelegate: IPJToDoHttpRequestDelegate = {
        let ownedPointer = UnsafeMutableRawPointer(Unmanaged.passRetained(self).toOpaque())
        
        /*call back for C*/
        let destroyBlock: @convention(c) (UnsafeMutableRawPointer?) -> Void = {(pointer) in
            if let tempPointer = pointer {
                destroy(user: tempPointer)
            }
        }
        
        let requestResultBackBlock: (@convention(c) (UnsafeMutableRawPointer?, UnsafePointer<Int8>?, Bool) -> Void)! = { (pointer, dataPointer, isSuccess) in
            if let tempPointer = pointer {
                PJToDo.requestResult(user: tempPointer, data: dataPointer, isSuccess: isSuccess)
            }
        }
        
        let iDelegate = IPJToDoHttpRequestDelegate(user: ownedPointer, destroy: destroyBlock, request_result: requestResultBackBlock)
        return iDelegate
    }()
    
    init(responseBlock: PJResponseBlock?) {
        self.responseBlock = responseBlock
    }
    
    //Rust回调Swift
    fileprivate func requestResult(data: UnsafePointer<Int8>?, isSuccess: Bool) {
        print("PJHttpRequestConfig: received findByIdResult callback with  \(isSuccess)")
        if let dataPointer = data {
            self.responseBlock?(dataPointer, isSuccess)
        }
    }
    
    deinit {
        print("PJHttpRequestConfig -> deinit")
    }
}

//Rust回调Swift
fileprivate func requestResult(user: UnsafeMutableRawPointer, data: UnsafePointer<Int8>?, isSuccess: Bool) {
    let obj: PJHttpRequestConfig = Unmanaged.fromOpaque(user).takeUnretainedValue()
    obj.requestResult(data: data, isSuccess: isSuccess)
    destroy(user: user)
}

//Rust回调Swift用以销毁Swift对象
fileprivate func destroy(user: UnsafeMutableRawPointer) {
    let _ = Unmanaged<PJHttpRequestConfig>.fromOpaque(user).takeRetainedValue()
}

