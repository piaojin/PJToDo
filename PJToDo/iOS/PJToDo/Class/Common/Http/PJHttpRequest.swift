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
        let httpRequestConfig = PJHttpRequestConfig(responseBlock: { (data, isSuccess) -> Void in
            print("isSuccess: \(isSuccess)")
            /*data to User and then free data pointer or free data pointer in User's deinit*/
            responseBlock?(nil, isSuccess)
        })
        
        PJ_Login(httpRequestConfig.iDelegate, name, passWord)
    }
}
