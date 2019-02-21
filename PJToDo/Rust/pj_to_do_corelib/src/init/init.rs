#[allow(unused_imports)]
use common::pj_logger::PJLogger;
use db::pj_db_connection_util::pj_db_connection_util::{initDataBase, initTables};

#[no_mangle]
pub unsafe extern "C" fn init_hello_piaojin() {
    println!("******here will be the rust lib init start******");
    println!("hello piaojin!");
    println!("******here will be the rust lib init end******");
}

#[no_mangle]
pub unsafe extern "C" fn initCoreLib() {
    init_hello_piaojin();
    initDataBase();
    initTables();
    //使用log之前需要初始化，并且只需要初始化一次
    let _ = PJLogger::pj_init_logger();
}
