//
//  PJHttpError.swift
//  PJToDo
//
//  Created by piaojin on 2019/1/19.
//  Copyright © 2019 piaojin. All rights reserved.
//

import UIKit

public class PJHttpError: PJHttpBaseModel {
    public var errorCode: Int = 0
    
    override init() {
        super.init()
    }
    
    required init(from decoder: Decoder) throws {
        try super.init(from: decoder)
    }
    
    convenience init(errorCode: Int, errorMessage: String) {
        self.init()
        self.errorCode = errorCode
        self.message = errorMessage
    }
}
