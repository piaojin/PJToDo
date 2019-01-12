//
//  AppDelegate.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/8/5.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit
import CocoaLumberjack
import SSZipArchive

@UIApplicationMain
class AppDelegate: UIResponder, UIApplicationDelegate {

    var window: UIWindow?


    func application(_ application: UIApplication, didFinishLaunchingWithOptions launchOptions: [UIApplicationLaunchOptionsKey: Any]?) -> Bool {
        // Override point for customization after application launch.
        self.initLog()
        PJToDoCoreLibInit.initRustCoreLib()
//        SSZipArchive.createZipFile(atPath: PJToDoConst.dbGZipPath, withFilesAtPaths: [PJToDoConst.dbPath])
//        let isSuccess = try? SSZipArchive.unzipFile(atPath: PJToDoConst.dbGZipPath, toDestination: PJCacheManager.shared.documnetPath, overwrite: true, password: nil)
//        let homeViewController = HomeViewController()
//        let nav = UINavigationController(rootViewController: homeViewController)
//        let loginViewController = LoginViewController()
//        let nav = UINavigationController(rootViewController: loginViewController)
        self.window = UIWindow(frame: UIScreen.main.bounds)
        self.window?.makeKeyAndVisible()
        self.initRootViewController()
        return true
    }

    func applicationWillResignActive(_ application: UIApplication) {
        // Sent when the application is about to move from active to inactive state. This can occur for certain types of temporary interruptions (such as an incoming phone call or SMS message) or when the user quits the application and it begins the transition to the background state.
        // Use this method to pause ongoing tasks, disable timers, and invalidate graphics rendering callbacks. Games should use this method to pause the game.
    }

    func applicationDidEnterBackground(_ application: UIApplication) {
        // Use this method to release shared resources, save user data, invalidate timers, and store enough application state information to restore your application to its current state in case it is terminated later.
        // If your application supports background execution, this method is called instead of applicationWillTerminate: when the user quits.
    }

    func applicationWillEnterForeground(_ application: UIApplication) {
        // Called as part of the transition from the background to the active state; here you can undo many of the changes made on entering the background.
    }

    func applicationDidBecomeActive(_ application: UIApplication) {
        // Restart any tasks that were paused (or not yet started) while the application was inactive. If the application was previously in the background, optionally refresh the user interface.
    }

    func applicationWillTerminate(_ application: UIApplication) {
        // Called when the application is about to terminate. Save data if appropriate. See also applicationDidEnterBackground:.
    }
}

extension AppDelegate {
    func initLog() {
        DDLog.add(DDTTYLogger.sharedInstance) // TTY = Xcode console
        DDLog.add(DDASLLogger.sharedInstance) // ASL = Apple System Logs
        
        let fileLogger: DDFileLogger = DDFileLogger() // File Logger
        fileLogger.rollingFrequency = TimeInterval(60 * 60 * 24)  // 24 hours
        fileLogger.logFileManager.maximumNumberOfLogFiles = 7
        DDLog.add(fileLogger)
    }
    
    func initRootViewController() {
        let rootTabBarViewController = PJTabBarViewController()
//        let homeViewController = HomeViewController()
//        let homeNav = UINavigationController(rootViewController: homeViewController)
//
//        let mineViewControler = MineViewController()
//        let mineNav = UINavigationController(rootViewController: mineViewControler)
//
//        let rootTabBarViewController = CYLTabBarController()
//        rootTabBarViewController.setViewControllers([homeNav, mineNav], animated: true)
//
//        let homeTabBarDic = [CYLTabBarItemTitle: "Home", CYLTabBarItemImage: "home_unselected", CYLTabBarItemSelectedImage: "home_selected"]
//
//        let mineTabBarDic = [CYLTabBarItemTitle: "Mine", CYLTabBarItemImage: "me_unselected", CYLTabBarItemSelectedImage: "me_selected"]
//
//        rootTabBarViewController.tabBarItemsAttributes = [homeTabBarDic, mineTabBarDic]
        
        self.window?.rootViewController = rootTabBarViewController
    }
}

