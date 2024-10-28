use std::path::Path;

use slint::Image;


slint::include_modules!();
fn main() -> Result<(), slint::PlatformError> {
    let app = App::new()?;
    let  img=  Image::load_from_path(Path::new("")).unwrap();
    app.set_img(img);
    app.run()
}