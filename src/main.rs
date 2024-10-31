use i_slint_backend_winit::WinitWindowAccessor;

slint::include_modules!();
fn main() -> Result<(), slint::PlatformError> {
    let app = App::new()?;

    let app_weak = app.as_weak();
    app.on_mouse_move(move || {
        let app_weak = app_weak.unwrap();
        app_weak.window().with_winit_window(|win| {
            let _ = win.drag_window();
        });
    });
    let app_weak = app.as_weak();


    app.on_search(move |text| {
        println!("{:?}",text);
    });
    println!("asd ");
    app.run()
}