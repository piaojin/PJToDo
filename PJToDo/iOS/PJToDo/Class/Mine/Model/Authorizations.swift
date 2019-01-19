//
//  Authorizations.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/11/15.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

public class Authorizations: PJHttpBaseModel {
    var id: Int = -1
    var url: String = ""
    var token: String = ""
    var hashedToken: String = ""
    var createdTime: String = ""
    var updatedTime: String = ""
}
