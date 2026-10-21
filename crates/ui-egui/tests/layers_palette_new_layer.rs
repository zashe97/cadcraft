//! The Layers palette's New Layer button (AutoCAD for Mac's lower toolbar): it adds `LayerN`
//! with the current layer's properties and leaves the name open for typing.

use cadcraft_engine::Session;
use cadcraft_ui_egui::{CadApp, Services};
use egui::{Key, vec2};
use egui_kittest::Harness;
use egui_kittest::kittest::{Queryable, by};
use serde_json::json;

fn palettes() -> Harness<'static, CadApp> {
    let mut app = CadApp::new(Session::default(), Services::default());
    app.run("layer.new", json!({ "name": "Walls", "color": "red", "current": true })).unwrap();
    let mut h = Harness::builder().with_size(vec2(400.0, 1400.0)).build_ui_state(
        |ui, app: &mut CadApp| {
            app.logic(&ui.ctx().clone());
            cadcraft_ui_egui::palettes::right_palettes(app, ui);
        },
        app,
    );
    h.run_steps(4);
    h
}

fn layer_names(app: &CadApp) -> Vec<String> {
    app.session.doc().unwrap().layers.iter().map(|l| l.name.clone()).collect()
}

#[test]
fn new_layer_inherits_the_current_layer_and_is_renamed_in_place() {
    let mut h = palettes();
    assert!(!h.state().ui.show_layer_list);
    h.get_by_label("New Layer").click();
    h.run_steps(3);
    let app = h.state();
    assert!(app.ui.show_layer_list, "the list opens to show the new layer");
    let d = app.session.doc().unwrap();
    assert_eq!(d.layer("Layer1").unwrap().color, d.layer("Walls").unwrap().color);
    assert_eq!(d.header.str("CLAYER", "0"), "Walls", "the current layer is unchanged");

    // The name field has focus with the default name in it.
    h.key_press_modifiers(egui::Modifiers::COMMAND, Key::A);
    h.run_steps(1);
    h.get(by().role(egui::accesskit::Role::TextInput).value("Layer1")).type_text("Doors");
    h.run_steps(2);
    h.key_press(Key::Enter);
    h.run_steps(3);
    let names = layer_names(h.state());
    assert!(names.contains(&"Doors".to_string()) && !names.contains(&"Layer1".to_string()), "{names:?}");

    // A second one takes the next free name; Escape keeps it.
    h.get_by_label("New Layer").click();
    h.run_steps(3);
    h.key_press(Key::Escape);
    h.run_steps(3);
    assert!(layer_names(h.state()).contains(&"Layer1".to_string()));
}
