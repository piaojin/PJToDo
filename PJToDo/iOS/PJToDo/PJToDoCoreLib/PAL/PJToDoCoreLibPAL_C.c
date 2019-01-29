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

const char * get_db_type_sql_file_path() {
    return getDBTypeSQLFilePath();
}

const char * get_db_tag_sql_file_path() {
    return getDBTagSQLFilePath();
}

const char * get_db_todo_sql_file_path() {
    return getDBToDoSQLFilePath();
}

const char * get_db_todo_settings_sql_file_path() {
    return getDBToDoSettingsSQLFilePath();
}

const char * get_authorization_str() {
    return getAuthorizationStr();
}
