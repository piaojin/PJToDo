use db::dao::to_do_type_dao::PJToDoTypeDAO;
use db::dao::dao_impl::to_do_type_dao_impl::PJToDoTypeDAOImpl;
use service::to_do_type_service::PJToDoTypeService;
use to_do_type::to_do_type::ToDoTypeInsert;
use service::service_delegate::to_do_tag_service_delegate::PJToDoTypeServiceDelegate;
use service::service_view_model::to_do_type_service_view_model::{createToDoTypeServiceViewModel, ToDoTypeServiceViewModel};

lazy_static! {
    pub static ref StaticPJToDoTypeDAO: PJToDoTypeDAOImpl = {
        PJToDoTypeDAOImpl{}
    };
}

#[repr(C)]
pub struct PJToDoTypeServiceImpl {
    pub delegate: PJToDoTypeServiceDelegate,
    pub view_model: ToDoTypeServiceViewModel,
}

impl PJToDoTypeService for PJToDoTypeServiceImpl {
    /**
     * 添加分类
     */
    fn insert_to_do_type(to_do_type: ToDoTypeInsert) -> Result<usize, String> {
        StaticPJToDoTypeDAO.insert_to_do_type(to_do_type)
    }
}

/*** extern "C" ***/

#[no_mangle]
pub extern "C" fn createPJToDoTypeService(delegate: PJToDoTypeServiceDelegate) -> *mut PJToDoTypeServiceImpl {
    let view_model = ToDoTypeServiceViewModel::new();
    let service = PJToDoTypeServiceImpl {
        delegate: delegate,
        view_model: view_model,
    };
    Box::into_raw(Box::new(service))
}

//析构对象
#[no_mangle]
pub unsafe extern fn freePJToDoTypeServiceImpl(ptr: *mut PJToDoTypeServiceImpl) {
    if ptr.is_null() { return };
    Box::from_raw(ptr);//unsafe
}
