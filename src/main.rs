slint::include_modules!();
fn main() -> Result<(), slint::PlatformError> {
    let app = App::new()?;

    let app_weak = app.as_weak();
    app.on_mouse_move(move |delta_x, delta_y| {
        let app_weak = app_weak.unwrap();
        let logical_pos = app_weak.window().position().to_logical(app_weak.window().scale_factor());
        app_weak.window().set_position(slint::LogicalPosition::new(logical_pos.x + delta_x, logical_pos.y + delta_y));
        // app_weak.drag_window();
        app_weak.window().with_winit_window(|win| {
            let _ = win.drag_window();
        });
    });
    app.run()
}