//
//  PJToDoCoreLibPAL_C_Bridging_OC.m
//  PJToDo
//
//  Created by Zoey Weng on 2018/10/11.
//  Copyright © 2018年 piaojin. All rights reserved.
//

#import "PJToDoCoreLibPAL_C_Bridging_OC.h"
#import "PJToDo-Swift.h"
#import "app_bindings.h"

void swiftSayHi() {
    [PJToDoCoreLibPAL swiftSayHi];
}

const char * getDBPath() {
    return [PJToDoConst.DBPath UTF8String];
}

const char * getDBGZipPath(void) {
    return [PJToDoConst.DBGZipPath UTF8String];
}

const char * getDBUnCompressesPath() {
    return [PJToDoConst.DBUnCompressesPath UTF8String];
}

const char * getDBTypeSQLFilePath() {
    return [PJToDoConst.DBTypeSQLFilePath UTF8String];
}

const char * getDBTagSQLFilePath() {
    return [PJToDoConst.DBTagSQLFilePath UTF8String];
}

const char * getDBToDoSQLFilePath() {
    return [PJToDoConst.DBToDoSQLFilePath UTF8String];
}

const char * getDBToDoSettingsSQLFilePath() {
    return [PJToDoConst.DBToDoSettingsSQLFilePath UTF8String];
}

const char * getAuthorizationStr() {
    if (PJUserInfoManagerOC.isLogin) {
        NSError *error;
        NSString *authorizationStr = [PJKeychainManagerOC readSensitiveStringWithService: PJKeyCenterOC.KeychainUserInfoService sensitiveKey: PJUserInfoManagerOC.userAccount accessGroup:nil error: &error];
        if (authorizationStr) {
            NSString *str = [NSString stringWithFormat:@"%@:%@", PJUserInfoManagerOC.userAccount, authorizationStr];
            const char *cstr = ConvertStrToBase64Str([str UTF8String]);
            NSString *base64Str = [NSString stringWithFormat:@"Basic %s", cstr];
            return [base64Str UTF8String];
        } else {
            NSLog(@"getAuthorizationStr error: %@", error);
        }
        return [@"" UTF8String];
    }
    return [@"" UTF8String];
}
