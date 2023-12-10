#include <stdio.h>
#include "PJToDoCoreLibPAL_C_Bridging_OC.h"

void test_pal_from_Swift() {
    printf("PJToDoCoreLibPAL is ready in iOS!\n");
    swiftSayHi();
}

const char * get_db_path() {
    return getDBPath();
}

const char * get_authorization_str() {
    return getAuthorizationStr();
}

const char * get_access_token_str() {
    return getAccessTokenStr();
}
