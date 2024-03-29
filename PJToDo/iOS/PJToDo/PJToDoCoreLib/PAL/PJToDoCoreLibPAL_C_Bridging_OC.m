//
//  PJToDoCoreLibPAL_C_Bridging_OC.m
//  PJToDo
//
//  Created by Zoey Weng on 2018/10/11.
//  Copyright © 2018年 piaojin. All rights reserved.
//

#import "PJToDoCoreLibPAL_C_Bridging_OC.h"
#import "PJToDo-Swift.h"
#import <PJToDoCoreLib/PJToDoCoreLib.h>

void swiftSayHi(void) {
    [PJToDoCoreLibPAL swiftSayHi];
}

const char * getDBPath(void) {
    return [PJToDoConst.DBPath UTF8String];
}

const char * getAuthorizationStr(void) {
    if (PJUserInfoManagerOC.isLogin) {
        NSError *error;
        NSString *authorizationStr = [PJKeychainManagerOC readSensitiveStringWithService: PJKeyCenterOC.KeychainUserInfoService sensitiveKey: PJUserInfoManagerOC.userAccount accessGroup:nil error: &error];
        if (authorizationStr) {
            NSString *str = [NSString stringWithFormat:@"%@:%@", PJUserInfoManagerOC.userAccount, authorizationStr];
            const char *cstr = pj_convert_str_to_base64str([str UTF8String]);
            NSString *base64Str = [NSString stringWithFormat:@"Basic %s", cstr];
            return [base64Str UTF8String];
        } else {
            NSLog(@"getAuthorizationStr error: %@", error);
        }
        return [@"" UTF8String];
    }
    return [@"" UTF8String];
}

const char * getAccessTokenStr(void) {
    NSError *error;
    NSString *accessTokenStr = [PJKeychainManagerOC readSensitiveStringWithService: PJKeyCenterOC.KeychainAuthorizationService sensitiveKey: PJKeyCenterOC.KeychainAccessTokenKey accessGroup:nil error: &error];
    if (accessTokenStr) {
        return [accessTokenStr UTF8String];
    } else {
        NSLog(@"getAccessTokenStr error: %@", error);
        return [@"" UTF8String];
    }
}
