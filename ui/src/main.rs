use map::map::Map;

use plugin_camera::Camera;
use plugin_ribbon::RibbonPlugin;
use plugin_toolbar::ToolbarPlugin;
use plugin_undo_redo::{Redo, Undo};
use plugins::create_district::CreateDistrict;
use plugins::delete_district::DeleteDistrict;
use plugins::delete_street::DeleteStreet;
use plugins::load::Load;
use plugins::map_render::MapRender;
use plugins::new::New;
use plugins::reference_image::ReferenceImage;
use plugins::save::Save;
use plugins::settings::Settings;
use plugins::sync::Sync;
use plugin_ui_components::ComponentsPlugin;

use plugins::create_freeform_street::CreateFreeformStreet;
use rust_editor::ui::app::x_launch;

mod algorithm;
mod map;
mod plugins;

fn main() {
    let mut editor = x_launch::<Map>();

    editor.add_plugin(ComponentsPlugin::default());
    editor.add_plugin(ToolbarPlugin::default());
    editor.add_plugin(New::default());
    editor.add_plugin(Save::default());
    editor.add_plugin(Load::default());
    editor.add_plugin(Settings::default());
    editor.add_plugin(Camera::default());
    editor.add_plugin(Undo::<Map>::default());
    editor.add_plugin(Redo::<Map>::default());
    editor.add_plugin(CreateFreeformStreet::default());
    editor.add_plugin(DeleteStreet::default());
    editor.add_plugin(CreateDistrict::default());
    editor.add_plugin(DeleteDistrict::default());
    editor.add_plugin(MapRender::default());
    //editor.add_plugin(TestData::default());
    editor.add_plugin(ReferenceImage::default());
    editor.add_plugin(RibbonPlugin::default());
    editor.add_plugin(Sync::default());
    
}

#[cfg(test)]
mod tests {
    use crate::plugins::create_freeform_street::CreateFreeformStreet;
    use crate::plugins::new::New;
    use crate::plugins::settings::Settings;
    use crate::Map;
    use plugin_camera::Camera;
    use plugin_undo_redo::{Undo, Redo};
    use rust_editor::plugin::Plugin;
    use rust_editor::ui::app::{launch, EditorError};
    use rust_macro::editor_plugin;
    use wasm_bindgen_test::*;

    use crate::plugins::create_district::CreateDistrict;
    use crate::plugins::delete_district::DeleteDistrict;
    use crate::plugins::delete_street::DeleteStreet;
    use crate::plugins::load::Load;
    use crate::plugins::map_render::MapRender;
    use crate::plugins::save::Save;

    wasm_bindgen_test_configure!(run_in_browser);

    #[editor_plugin(skip, specific_to=Map)]
    pub struct HeadlessTestPlugin {}

    impl Plugin<Map> for HeadlessTestPlugin {
        fn startup(&mut self, _: &mut App<Map>) -> Result<(), EditorError> {
            let window = web_sys::window().expect("no global `window` exists");
            let document = window.document().expect("should have a document on window");

            assert_eq!(document.get_elements_by_tag_name("canvas").length(), 1);

            Ok(())
        }
    }

    #[wasm_bindgen_test]
    fn pass() {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");
        let val = document.create_element("div").unwrap();
        val.set_id("test_container");
        body.append_child(&val).unwrap();

        let mut editor = launch::<Map>("test_container");

        editor.add_plugin(New::default());
        editor.add_plugin(Save::default());
        editor.add_plugin(Load::default());
        editor.add_plugin(Settings::default());
        editor.add_plugin(Camera::default());
        editor.add_plugin(Undo::<Map>::default());
        editor.add_plugin(Redo::<Map>::default());
        editor.add_plugin(MapRender::default());
        editor.add_plugin(DeleteStreet::default());
        editor.add_plugin(CreateDistrict::default());
        editor.add_plugin(DeleteDistrict::default());
        editor.add_plugin(CreateFreeformStreet::default());
        editor.add_plugin(crate::plugins::debug::Debug::default());

        // The headless test plugin contains the asserts. We chose this approach to avoid hardcoding timeouts
        // or other workarounds.
        editor.add_plugin(HeadlessTestPlugin::default());
    }
}
