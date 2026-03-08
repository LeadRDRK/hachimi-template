pub mod Scene;
pub mod SceneManager;

pub fn init() {
    get_assembly_image_or_return!(image, "UnityEngine.CoreModule.dll");

    Scene::init(image);
    SceneManager::init(image);
}