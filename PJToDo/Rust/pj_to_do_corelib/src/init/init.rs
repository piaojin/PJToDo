#[no_mangle]
pub unsafe extern "C" fn init_hello_piaojin() {
    println!("******here will be the rust lib init start******");
    println!("hello piaojin!");
    println!("******here will be the rust lib init end******");
}
