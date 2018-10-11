pub struct PJToDoPal;

#[link(name = "PJToDoCoreLibPAL")]
// #[link(name="hello", kind="static")]
extern "C" { 
    fn test_pal_from_Swift(); 
}

impl PJToDoPal {
    pub fn sqlite_url<'a>() -> &'a str {
        // from pal
        "/Users/zoey.weng/Desktop/Study/PJToDo/ToDo.db"
    }

    pub unsafe fn say_hi_from_rust() {
        println!("PJToDoPal is ready in Rust CoreLib!");
        test_pal_from_Swift();
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_pal_from_rust() {
    PJToDoPal::say_hi_from_rust();
}
