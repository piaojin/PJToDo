//
//  AppDelegate.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/8/5.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit
import CocoaLumberjack

@UIApplicationMain
class AppDelegate: UIResponder, UIApplicationDelegate {

    var window: UIWindow?


    func application(_ application: UIApplication, didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]?) -> Bool {
        // Override point for customization after application launch.
        self.initLog()
        PJToDoCoreLibInit.initRustCoreLib()
        self.window = UIWindow(frame: UIScreen.main.bounds)
        self.window?.makeKeyAndVisible()
        self.initRootViewController()
        self.initGitHub()
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
        initGitHub()
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
        DDLog.add(DDOSLogger.sharedInstance)
        
        let fileLogger: DDFileLogger = DDFileLogger() // File Logger
        fileLogger.rollingFrequency = TimeInterval(60 * 60 * 24)  // 24 hours
        fileLogger.logFileManager.maximumNumberOfLogFiles = 7
        DDLog.add(fileLogger)
    }
    
    func initRootViewController() {
        var rootTabBarViewController: UIViewController
        if PJUserInfoManager.shared.isLogin {
            rootTabBarViewController = PJTabBarViewController()
        } else {
            rootTabBarViewController = UINavigationController(rootViewController: WelcomeViewController())
        }
        self.window?.rootViewController = rootTabBarViewController
        RootController.shared.window = window
    }
    
    //init gethub data
    func initGitHub() {
        PJReposManager.initGitHubRepos { _, _, _ in
            PJReposFileManager.initGitHubReposFile { _, _, _ in
                self.fetchGitHubReposFile { _, _, _ in
                    self.syncDBToGitHub()
                }
            }
        }
    }
    
    private func fetchGitHubReposFile(_ completedHandle: ((Bool, ReposFile?, PJHttpError?) -> ())?) {
        if PJUserInfoManager.shared.isLogin {
            PJReposFileManager.getReposFile(completedHandle: completedHandle)
        } else {
            completedHandle?(false, nil, PJHttpError(errorCode: 0, errorMessage: "Not login"))
        }
    }
    
    private func syncDBToGitHub() {
        if let shouldUpdateDBToGitHubKey = PJCacheManager.getDefault(key: PJKeyCenter.ShouldUpdateDBToGitHubKey) as? Bool, shouldUpdateDBToGitHubKey, PJUserInfoManager.shared.isLogin {
            PJReposFileManager.updateReposFile { (isSuccess, _, error) in
                if !isSuccess {
                    DDLogError("❌❌❌❌❌❌\(error?.message ?? "")❌❌❌❌❌❌")
                }
            }
        }
    }
}

