//
//  PJHttpRequest.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/11/11.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import Foundation

public struct PJHttpRequest {
    public static func login(name: String, passWord: String, responseBlock: ((_ data: User?, _ isSuccess : Bool) -> Void)?) {
        let httpRequestConfig = PJHttpRequestConfig(responseBlock: { (pointer, isSuccess) -> Void in
            if let tempPointer = pointer {
                do {
                    let jsonString = String(cString: tempPointer)
                    if let jsonData = jsonString.data(using: .utf8) {
                        let user = try JSONDecoder().decode(User.self, from: jsonData)
                        responseBlock?(user, isSuccess)
                    } else {
                        responseBlock?(nil, isSuccess)
                    }
                } catch {
                    // 异常处理
                    responseBlock?(nil, isSuccess)
                }
                
                /*data to User and then free data pointer*/
                free_rust_object(UnsafeMutableRawPointer(mutating: tempPointer))
            } else {
                responseBlock?(nil, isSuccess)
            }
        })
        
        PJ_Login(httpRequestConfig.iDelegate, name, passWord)
    }
}
