//
//  PJCacheManager.swift
//  PJQuicklyDev
//
//  Created by Zoey Weng on 2017/11/21.
//  Copyright © 2017年 飘金. All rights reserved.
//

import UIKit
import CocoaLumberjack

// MARK: 数据缓存类
public struct PJCacheManager {
    
    public static var shared = PJCacheManager()
    public static let fileManager = FileManager()
    ///获取程序的Home目录
    public static let homeDirectory = NSHomeDirectory()
    
    ///用户文档目录，苹果建议将程序中建立的或在程序中浏览到的文件数据保存在该目录下，iTunes备份和恢复的时候会包括此目录
    public lazy var documnetPath: String = {
        let documentPaths = NSSearchPathForDirectoriesInDomains(FileManager.SearchPathDirectory.documentDirectory, FileManager.SearchPathDomainMask.userDomainMask, true)
        if let path = documentPaths.first {
            return path
        } else {
            return ""
        }
    }()
    
    public lazy var libraryPath: String = {
        let libraryPaths = NSSearchPathForDirectoriesInDomains(FileManager.SearchPathDirectory.libraryDirectory, FileManager.SearchPathDomainMask.userDomainMask, true)
        if let path = libraryPaths.first {
            return path
        } else {
            return ""
        }
    }()
    
    ///主要存放缓存文件，iTunes不会备份此目录，此目录下文件不会再应用退出时删除
    public lazy var cachePath: String = {
        let cachePaths = NSSearchPathForDirectoriesInDomains(FileManager.SearchPathDirectory.cachesDirectory, FileManager.SearchPathDomainMask.userDomainMask, true)
        if let path = cachePaths.first {
            return path
        } else {
            return ""
        }
    }()
    
    ///tmp目录 ./tmp.用于存放临时文件，保存应用程序再次启动过程中不需要的信息，重启后清空
    public static let tmpDir = NSTemporaryDirectory()
    
    public static let userDefaults = UserDefaults.standard
    
    public static func saveCustomObject<T: Encodable>(customObject object: T, key: String) {
        let encoder = JSONEncoder()
        if let data = try? encoder.encode(object) {
            print(String(data: data, encoding: .utf8)!)
            self.userDefaults.set(data, forKey: key)
            self.userDefaults.synchronize()
        }
    }
    
    public static func removeCustomObject(key: String) {
        self.userDefaults.removeObject(forKey: key)
    }
    
    public static func getCustomObject<T: Decodable>(type: T, forKey key: String) -> T? {
        if let decodedObject = self.userDefaults.object(forKey: key), let data = decodedObject as? Data {
            let decoder = JSONDecoder()
            if let object = try? decoder.decode(T.self, from: data) {
                print("\(String(describing: object))")
                return object
            } else {
                return nil
            }
        }
        return nil
    }
    
    public static func createDirectory(path: String) throws {
        do {
            //创建子目录对应的文件夹
            try self.fileManager.createDirectory(atPath: path, withIntermediateDirectories: true, attributes: nil)
        } catch let error {
            DDLogError("createDirectory error:\(error)")
        }
    }
    
    public static func createFile(atPath: String, data: Data?) {
        self.fileManager.createFile(atPath: atPath, contents: data, attributes: nil)
    }
    
    public static func setDefault(key:String, value: Any?) {
        if value == nil {
            UserDefaults.standard.removeObject(forKey: key)
        }else{
            UserDefaults.standard.set(value, forKey: key)
        }
    }
    
    public static func removeUserDefault(key: String) {
        UserDefaults.standard.removeObject(forKey: key)
    }
    
    public static func getDefault(key: String) -> Any? {
        if let object = UserDefaults.standard.value(forKey: key) {
            return object
        }
        return nil
    }
}
