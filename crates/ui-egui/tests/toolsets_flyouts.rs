//! Tool Sets buttons with several methods (Arc, Circle): the small triangle lists them, picking
//! one starts it, and the button then runs the method picked last.

use cadcraft_engine::Session;
use cadcraft_ui_egui::palettes::FLYOUTS;
use cadcraft_ui_egui::{CadApp, Services};
use egui::vec2;
use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;

fn harness() -> Harness<'static, CadApp> {
    let mut app = CadApp::new(Session::default(), Services::default());
    app.ui.in_window_menu = false;
    app.integrated_titlebar = true;
    let mut harness = Harness::builder().with_size(vec2(1280.0, 760.0)).build_ui_state(
        |ui, app: &mut CadApp| {
            app.logic(&ui.ctx().clone());
            app.ui(ui);
        },
        app,
    );
    harness.run_steps(5);
    harness
}

#[test]
fn every_method_is_a_command() {
    for (tool, methods) in FLYOUTS {
        assert!(cadcraft_engine::find_command(tool).is_some(), "{tool}");
        assert_eq!(methods.first().map(|(m, _)| *m), Some(*tool), "{tool}: the first method is the tool's own command");
        for (m, _) in methods.iter() {
            assert!(cadcraft_engine::find_command(m).is_some(), "{tool}: {m} is not a command");
        }
    }
}

#[test]
fn the_triangle_lists_the_arc_methods_and_the_button_remembers_the_pick() {
    let mut harness = harness();
    assert!(harness.state().ui.flyout_choice.is_empty());
    harness.get_by_label("Arc methods").click();
    harness.run_steps(5);
    harness.get_by_label("Start, Center, End").click();
    harness.run_steps(5);
    assert_eq!(harness.state().ui.flyout_choice, vec![("arc".to_string(), "arc.sce".to_string())], "the pick is remembered");
    assert!(harness.query_by_label("Arc (Start, Center, End)").is_some(), "the button now names the method it runs");
}
