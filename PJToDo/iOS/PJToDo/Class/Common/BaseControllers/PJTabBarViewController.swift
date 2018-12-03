//
//  PJTabBarViewController.swift
//  PJQuicklyDev
//
//  Created by Zoey Weng on 2018/8/7.
//  Copyright © 2018年 飘金. All rights reserved.
//

import UIKit

class PJTabBarViewController: UITabBarController {
    
    lazy var addButton: UIButton = {
       let addButton = UIButton()
        addButton.backgroundColor = UIColor.colorWithRGB(red: 0, green: 123, blue: 249)
        addButton.setTitle("+", for: .normal)
        addButton.setTitleColor(.white, for: .normal)
        addButton.setTitleColor(.orange, for: .highlighted)
        addButton.titleLabel?.font = UIFont.systemFont(ofSize: 28)
        addButton.translatesAutoresizingMaskIntoConstraints = false
        return addButton
    }()
    
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
        let homeViewController = HomeTasksViewController()
        homeViewController.tabBarItem = UITabBarItem(title: "Home", image: UIImage(named: "home_unselected"), selectedImage: UIImage(named: "home_selected"))
        let homeNav = UINavigationController(rootViewController: homeViewController)
        
        let emptyViewController = UIViewController()
        emptyViewController.tabBarItem = UITabBarItem(title: "", image: nil, selectedImage: nil)
        emptyViewController.tabBarItem.tag = -1
        
        let mineViewControler = MineViewController()
        mineViewControler.tabBarItem = UITabBarItem(title: "Mine", image: UIImage(named: "me_unselected"), selectedImage: UIImage(named: "me_selected"))
        let mineNav = UINavigationController(rootViewController: mineViewControler)
        
        self.viewControllers = [homeNav, emptyViewController, mineNav]
        
        self.tabBar.addSubview(self.addButton)
        self.addButton.widthAnchor.constraint(equalToConstant: 38).isActive = true
        self.addButton.heightAnchor.constraint(equalToConstant: 38).isActive = true
        self.addButton.topAnchor.constraint(equalTo: self.tabBar.topAnchor, constant: 5).isActive = true
        self.addButton.centerXAnchor.constraint(equalTo: self.tabBar.centerXAnchor).isActive = true
    }
    
    private func initData() {
        self.delegate = self
        
        self.addButton.addTarget(self, action: #selector(addToDoAction), for: .touchUpInside)
    }
    
    @objc private func addToDoAction() {
        let addToDoViewController = AddToDoViewController()
        let nav = UINavigationController(rootViewController: addToDoViewController)
        self.present(nav, animated: true, completion: nil)
    }
}
extension PJTabBarViewController: UITabBarControllerDelegate {
    func tabBarController(_ tabBarController: UITabBarController, shouldSelect viewController: UIViewController) -> Bool {
        return viewController.tabBarItem.tag != -1
    }
}
