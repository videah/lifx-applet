extern crate systray;

fn main() {
    let mut app;
    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create applet window")
    }
    app.set_icon_from_file(&"/usr/share/gxkb/flags/ua.png".to_string()).ok();
    app.add_menu_item(&"Toggle Light".to_string(), |_| {
        println!("Clicked");
    }).ok();
    app.wait_for_message();
}
