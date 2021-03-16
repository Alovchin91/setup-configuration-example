use com::{
    IID,
    interfaces::IUnknown,
    sys::{CLSCTX_INPROC_SERVER, FAILED, HRESULT}
};
use std::ffi::c_void;

com::interfaces! {
    #[uuid("42843719-db4c-46c2-8e7c-64f1816efd5b")]
    unsafe interface ISetupConfiguration : IUnknown {
        fn EnumInstances(&self, ppEnumInstances: *mut *mut c_void) -> HRESULT;
        fn GetInstanceForCurrentProcess(&self, ppInstance: *mut *mut c_void) -> HRESULT;
        fn GetInstanceForPath(&self, wzPath: *const c_void, ppInstance: *mut *mut c_void) -> HRESULT;
    }
}

#[allow(non_upper_case_globals)]
const CLSID_SetupConfiguration: IID = IID {
    data1: 0x177f0c4a,
    data2: 0x1cd3,
    data3: 0x4de7,
    data4: [ 0xa3, 0x2c, 0x71, 0xdb, 0xbb, 0x9f, 0xa3, 0x6d ],
};

// TODO: Uncomment and replace CLSCTX_INPROC_SERVER on line 42
// const CLSCTX_LOCAL_SERVER: u32 = 0x4;

fn main() {
    com::runtime::init_runtime().expect("Failed to initialize COM library.");

    let result = create_setup_configuration_instance::<ISetupConfiguration>();
    println!("{:x?}", result);
}

fn create_setup_configuration_instance<T: com::Interface>() -> Result<T, HRESULT> {
    unsafe {
        let mut instance = None;

        let hr = com::sys::CoCreateInstance(
            &CLSID_SetupConfiguration,
            std::ptr::null_mut(),
            CLSCTX_INPROC_SERVER, // TODO: Replace this with CLSCTX_LOCAL_SERVER
            &T::IID as *const IID,
            &mut instance as *mut _ as _,
        );

        if FAILED(hr) {
            return Err(hr);
        }
    
        Ok(instance.unwrap())
    }
}
