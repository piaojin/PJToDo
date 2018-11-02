#[allow(unused_imports)]
use common::pj_logger::PJLogger;
use db::pj_db_connection_util::pj_db_connection_util::{init_database, init_tables};

#[no_mangle]
pub unsafe extern "C" fn init_hello_piaojin() {
    println!("******here will be the rust lib init start******");
    println!("hello piaojin!");
    println!("******here will be the rust lib init end******");
}

#[no_mangle]
pub unsafe extern "C" fn init_core_lib() {
    init_hello_piaojin();
    init_database();
    init_tables();
    //使用log之前需要初始化，并且只需要初始化一次
    let _ = PJLogger::pj_init_logger();
}
