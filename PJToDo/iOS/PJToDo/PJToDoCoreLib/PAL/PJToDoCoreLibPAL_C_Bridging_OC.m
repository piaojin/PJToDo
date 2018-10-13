//
//  PJToDoCoreLibPAL_C_Bridging_OC.m
//  PJToDo
//
//  Created by Zoey Weng on 2018/10/11.
//  Copyright © 2018年 piaojin. All rights reserved.
//

#import "PJToDoCoreLibPAL_C_Bridging_OC.h"
#import "PJToDo-Swift.h"

void swiftSayHi() {
    [PJToDoCoreLibPAL swiftSayHi];
}

const char * getDBPath() {
    return [PJToDoConst.dbPath UTF8String];
}
