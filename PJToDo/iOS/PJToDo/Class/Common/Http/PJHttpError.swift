//
//  PJHttpError.swift
//  PJToDo
//
//  Created by piaojin on 2019/1/17.
//  Copyright © 2019 piaojin. All rights reserved.
//

//{
//    "message": "Not Found",
//    "documentation_url": "https://developer.github.com/v3/repos/contents/#update-a-file"
//}

import UIKit

public class PJHttpError: Codable {
    var message: String = ""
    var documentation_url: String = ""
}
