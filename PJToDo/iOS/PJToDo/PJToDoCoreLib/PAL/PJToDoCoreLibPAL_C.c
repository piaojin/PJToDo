#include <stdio.h>
#include "PJToDoCoreLibPAL_C_Bridging_OC.h"

void test_pal_from_Swift() {
    printf("PJToDoCoreLibPAL is ready in iOS!\n");
    swiftSayHi();
}

const char * get_db_path() {
    return getDBPath();
}

const char * get_db_gzip_path() {
    return getDBGZipPath();
}

const char * get_db_uncompresses_path() {
    return getDBUnCompressesPath();
}

const char * get_db_data_sql_file_path() {
    return getDBDataSQLFilePath();
}

const char * get_authorization_str() {
    return getAuthorizationStr();
}
