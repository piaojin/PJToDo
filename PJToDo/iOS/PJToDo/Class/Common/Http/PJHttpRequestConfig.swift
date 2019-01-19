//
//  PJHttpRequestConfig.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/11/12.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import Foundation
//statusCode: UInt16
public typealias PJResponseBlock = (_ data: UnsafeMutablePointer<Int8>?, _ statusCode: Int, _ isSuccess : Bool) -> Void

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
        
        let requestResultBackBlock: (@convention(c) (UnsafeMutableRawPointer?, UnsafeMutablePointer<Int8>?, UInt16, Bool) -> Void)! = { (pointer, dataPointer, statusCode, isSuccess) in
            if let tempPointer = pointer {
                PJToDo.requestResult(user: tempPointer, data: dataPointer, statusCode: statusCode, isSuccess: isSuccess)
            }
        }
        
        let iDelegate = IPJToDoHttpRequestDelegate(user: ownedPointer, destroy: destroyBlock, request_result: requestResultBackBlock)
        return iDelegate
    }()
    
    init(responseBlock: PJResponseBlock?) {
        self.responseBlock = responseBlock
    }
    
    //Rust回调Swift
    fileprivate func requestResult(data: UnsafeMutablePointer<Int8>?, statusCode: UInt16, isSuccess: Bool) {
        print("PJHttpRequestConfig: received findByIdResult callback with  \(isSuccess)")
        if let dataPointer = data {
            self.responseBlock?(dataPointer, Int(statusCode), isSuccess)
        }
    }
    
    deinit {
        print("PJHttpRequestConfig -> deinit")
    }
}

//Rust回调Swift
fileprivate func requestResult(user: UnsafeMutableRawPointer, data: UnsafeMutablePointer<Int8>?, statusCode: UInt16, isSuccess: Bool) {
    let obj: PJHttpRequestConfig = Unmanaged.fromOpaque(user).takeUnretainedValue()
    obj.requestResult(data: data, statusCode: statusCode, isSuccess: isSuccess)
    destroy(user: user)
}

//Rust回调Swift用以销毁Swift对象
fileprivate func destroy(user: UnsafeMutableRawPointer) {
    let _ = Unmanaged<PJHttpRequestConfig>.fromOpaque(user).takeRetainedValue()
}

