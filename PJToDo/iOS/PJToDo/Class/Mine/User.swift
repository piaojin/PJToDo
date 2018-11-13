//
//  User.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/11/12.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import Foundation

public class User: Codable {
//    public static func transfromRustPointerData(pointer: UnsafeMutableRawPointer) -> User {
//        /*free Rust pointer here*/
//        return User()
//    }
    
    var login: String = ""
    var id: Int = -1
    var node_id: String = ""
    var avatar_url: String = ""
    var gravatar_id: String = ""
    var url: String = ""
    var html_url: String = ""
    var followers_url: String = ""
    var following_url: String = ""
    var gists_url: String = ""
    var starred_url: String = ""
    var subscriptions_url: String = ""
    var organizations_url: String = ""
    var repos_url: String = ""
    var events_url: String = ""
    var received_events_url: String = ""
    var type: String = ""
    var site_admin: Bool = false
    var name: String = ""
    var company: String = ""
    var blog: String = ""
    var bio: String = ""
    var public_repos: Int = -1
    var public_gists: Int = -1
    var followers: Int = -1
    var following: Int = -1
    var created_at: String = ""
    var updated_at: String = ""
    
    init() {
        
    }
}
