//
//  Authorizations.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/11/15.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

public class Authorizations: Codable {
    var id: Int = -1
    var url: String = ""
    var token: String = ""
    var hashedToken: String = ""
    var createdTime: String = ""
    var updatedTime: String = ""
}

public class AccessToken: Codable {
    var access_token: String = ""
    var scope: String = ""
    var token_type: String = ""
}
