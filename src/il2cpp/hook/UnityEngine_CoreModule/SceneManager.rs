use crate::il2cpp::{ext::Il2CppStringExt, hook::UnityEngine_CoreModule::Scene, symbols::get_method_addr, types::*, unity_types::*};

type InternalActiveSceneChangedFn = extern "C" fn(
    prev_scene: UnityEngine_SceneManagement_Scene,
    new_scene: UnityEngine_SceneManagement_Scene
);
extern "C" fn Internal_ActiveSceneChanged(
    prev_scene: UnityEngine_SceneManagement_Scene,
    new_scene: UnityEngine_SceneManagement_Scene
) {
    let scene_name_ptr = Scene::GetNameInternal(new_scene.m_Handle);
    if !scene_name_ptr.is_null() {
        let scene_name = unsafe { (*Scene::GetNameInternal(new_scene.m_Handle)).as_utf16str() };
        info!("Scene changed: {}", scene_name);
    }
    get_orig_fn!(Internal_ActiveSceneChanged, InternalActiveSceneChangedFn)(prev_scene, new_scene);
}

pub fn init(UnityEngine_CoreModule: *const Il2CppImage) {
    get_class_or_return!(UnityEngine_CoreModule, "UnityEngine.SceneManagement", SceneManager);

    let Internal_ActiveSceneChanged_addr = get_method_addr(SceneManager, c"Internal_ActiveSceneChanged", 2);

    new_hook!(Internal_ActiveSceneChanged_addr, Internal_ActiveSceneChanged);
}