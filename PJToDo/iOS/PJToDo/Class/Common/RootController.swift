//
//  RootController.swift
//  ZWMeeting
//
//  Created by piaojin on 2023/9/11.
//

import Foundation

class RootController {
    static let shared = RootController()
    var window: UIWindow?

    var tabBarController: UITabBarController? {
        return window?.rootViewController as? UITabBarController
    }

    func mainRootViewController() -> UIViewController {
        return rootViewController()!
    }

    func rootViewController(on window: UIWindow? = RootController.shared.window) -> UIViewController? {
        return window?.rootViewController
    }

    func modalViewController(on window: UIWindow? = RootController.shared.window) -> UIViewController {
        var viewController: UIViewController? = rootViewController(on: window)
        
        while let presentedViewController = viewController?.presentedViewController {
            viewController = presentedViewController
        }

        return viewController ?? mainRootViewController()
    }

    func displayViewController() -> UIViewController? {
        var result = window?.rootViewController
        while result?.presentedViewController != nil {
            result = result?.presentedViewController
        }

        if result is UITabBarController {
            result = (result as? UITabBarController)?.selectedViewController
        }
        
        if result is UINavigationController {
            result = (result as? UINavigationController)?.topViewController
        }

        return result
    }
}
