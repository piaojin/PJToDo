//
//  Repos.swift
//  PJToDo
//
//  Created by piaojin on 2019/1/16.
//  Copyright © 2019 piaojin. All rights reserved.
//

import UIKit

public class Repos: PJHttpBaseModel {
    var id: Int = -1
    var node_id: String = ""
    var name: String = ""
    var full_name: String = ""
    var html_url: String = ""
    var description: String = ""
    var url: String = ""
    var contents_url: String = ""
    var downloads_url: String = ""
}
