use crate::{core::{Error, Hachimi}, il2cpp};

type Il2CppInitFn = extern "C" fn(domain_name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
extern "C" fn il2cpp_init(domain_name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int {
    let res = get_orig_fn!(il2cpp_init, Il2CppInitFn)(domain_name);
    let hachimi = Hachimi::instance();
    hachimi.interceptor.unhook(il2cpp_init as _);
    hachimi.on_hooking_finished();

    res
}

pub fn init() -> Result<(), Error> {
    let interceptor = &Hachimi::instance().interceptor;
    interceptor.hook(*il2cpp::api::il2cpp_init as _, il2cpp_init as _)?;

    Ok(())
}