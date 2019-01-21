//
//  ReposContent.swift
//  PJToDo
//
//  Created by piaojin on 2019/1/20.
//  Copyright © 2019 piaojin. All rights reserved.
//

import UIKit

public class ReposContent: Codable {
    var name: String = ""
    var path: String = ""
    var sha: String = ""
    var size: Int = 0
    var url: String = ""
    var html_url: String = ""
    var git_url: String = ""
    var download_url: String = ""
    var type: String = ""
}
