#include <stdio.h>
#include "PJToDoCoreLibPAL_C_Bridging_OC.h"

void test_pal_from_Swift(void) {
    printf("PJToDoCoreLibPAL is ready in iOS!\n");
    swiftSayHi();
}

const char * get_db_path(void) {
    return getDBPath();
}

const char * get_authorization_str(void) {
    return getAuthorizationStr();
}

const char * get_access_token_str(void) {
    return getAccessTokenStr();
}
