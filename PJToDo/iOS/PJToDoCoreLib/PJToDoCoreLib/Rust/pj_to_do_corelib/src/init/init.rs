use crate::common::pj_logger::PJLogger;
use crate::db::pj_db_connection_util::pj_db_connection_util::{pj_init_data_base, pj_init_tables};

#[no_mangle]
pub unsafe extern "C" fn pj_init_hello_piaojin() {
    println!("******here will be the rust lib init start******");
    println!("hello piaojin!");
    println!("******here will be the rust lib init end******");
}

#[no_mangle]
pub unsafe extern "C" fn pj_init_corelib() {
    pj_init_hello_piaojin();
    pj_init_data_base();
    pj_init_tables();
    //使用log之前需要初始化，并且只需要初始化一次
    let _ = PJLogger::pj_init_logger();
}
