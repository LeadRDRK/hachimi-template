use crate::{core::{Error, Hachimi}, il2cpp::{self, ext::MethodInfoExt, types::*}};

type Il2CppRuntimeInvokeFn = extern "C" fn(
    method: *const MethodInfo, obj: *mut ::std::os::raw::c_void,
    params: *mut *mut ::std::os::raw::c_void, exc: *mut *mut Il2CppException
) -> *mut Il2CppObject;
extern "C" fn il2cpp_runtime_invoke(
    method: *const MethodInfo, obj: *mut ::std::os::raw::c_void,
    params: *mut *mut ::std::os::raw::c_void, exc: *mut *mut Il2CppException
) -> *mut Il2CppObject {
    let res = get_orig_fn!(il2cpp_runtime_invoke, Il2CppRuntimeInvokeFn)(method, obj, params, exc);
    let game_started = unsafe { (*method).name_str()
        .is_ok_and(|name| name.contains("Internal_ActiveSceneChanged"))
    };
    if game_started {
        let hachimi = Hachimi::instance();
        hachimi.interceptor.unhook(il2cpp_runtime_invoke as _);
        hachimi.on_hooking_finished();
    }

    res
}

pub fn init() -> Result<(), Error> {
    let interceptor = &Hachimi::instance().interceptor;
   interceptor.hook(*il2cpp::api::il2cpp_runtime_invoke as _, il2cpp_runtime_invoke as _)?;

   Ok(())
}