//
//  MineViewController.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/11/23.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

class MineViewController: PJBaseViewController {

    override func viewDidLoad() {
        super.viewDidLoad()
        self.initView()
        self.initData()
    }

    override func didReceiveMemoryWarning() {
        super.didReceiveMemoryWarning()
        // Dispose of any resources that can be recreated.
    }
    
    private func initView() {
        self.title = "Mine"
    }
    
    private func initData() {
        
    }
}
