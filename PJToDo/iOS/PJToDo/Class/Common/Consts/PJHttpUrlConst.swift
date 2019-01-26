//
//  PJHttpUrlConst.swift
//  PJToDo
//
//  Created by piaojin on 2019/1/24.
//  Copyright © 2019 piaojin. All rights reserved.
//

import UIKit

public class PJHttpUrlConst: NSObject {
    
    public static var BaseReposUrl: String {
        if let url = PJReposManager.shared.repos?.url {
            return url
        } else if let loginName = PJUserInfoManager.shared.userInfo?.login {
            return "https://api.github.com/repos/\(loginName)/\(PJToDoConst.PJToDoWebDataBase)"
        }
        return ""
    }
    
    public static let CreateReposUrl = "https://api.github.com/user/repos"
    
    public static var DeleteReposUrl: String {
        return self.BaseReposUrl
    }
    
    public static var GetReposUrl: String {
        return self.BaseReposUrl
    }
    
    public static var BaseReposFileUrl: String {
        if let contentUrl = PJReposManager.shared.repos?.contents_url {
            return contentUrl
        } else if let loginName = PJUserInfoManager.shared.userInfo?.login {
            return "https://api.github.com/repos/\(loginName)/\(PJToDoConst.PJToDoWebDataBase)/contents/\(GitHubReposDBFilePath)"
        }
        return ""
    }
    
    //eg: https://api.github.com/repos/piaojin/PJToDoWebDataBase/contents/examples/test.md
    public static var CreateReposFileUrl: String {
        return self.BaseReposFileUrl
    }
    
    //eg: https://api.github.com/repos/piaojin/PJToDoWebDataBase/contents/examples/test.md
    public static var GetReposFileUrl: String {
        return self.BaseReposFileUrl
    }
    
    //eg: https://api.github.com/repos/piaojin/PJToDoWebDataBase/contents/examples/test.md
    public static var DeleteReposFileUrl: String {
        return self.BaseReposFileUrl
    }
    
    //eg: https://api.github.com/repos/piaojin/PJToDoWebDataBase/contents/examples/test.md
    public static var UpdateReposFileUrl: String {
        return self.BaseReposFileUrl
    }
    
    public static let GitHubLoginUrl = "https://api.github.com/user"
    
    public static let GitHubReposDBFilePath = "PJToDo/Data/pj_to_db.db"
}
