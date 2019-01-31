extern crate systray;
extern crate lifx;
#[macro_use]
extern crate lazy_static;
use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use std::net::SocketAddr;
use lifx::Light::*;
use lifx::{Payload, Power, Bulb, DiscoverOptions};

lazy_static! {
    static ref CLIENT: lifx::Client = lifx::Client::new("0.0.0.0:1234").unwrap();
    static ref DEVICES: HashMap<u64, Bulb<SocketAddr>, RandomState> = CLIENT.devices();
}

fn main() {
    let mut app;
    let _ = CLIENT.listen();
    let _ = CLIENT.discover(1000, DiscoverOptions::GET_ALL);
    thread::sleep(Duration::from_secs(4));
    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create applet window")
    }
    app.set_icon_from_file(&"/usr/share/icons/Adwaita/24x24/status/display-brightness-symbolic.symbolic.png".to_string()).ok();
    for d in DEVICES.values() {
        app.add_menu_item(&d.label().unwrap().to_string(), |_|{}).ok();
        app.add_menu_separator().ok();
        app.add_menu_item(&"Turn On".to_string(), move |_| {
            set_bulb_status(d, Power::Max);
        }).ok();
        app.add_menu_item(&"Turn Off".to_string(), move |_| {
            set_bulb_status(d, Power::Standby);
        }).ok();
        app.add_menu_separator().ok();
    }
    app.add_menu_item(&"Quit".to_string(), |window| {
        window.quit();
    }).ok();
    app.wait_for_message();
}

fn set_bulb_status(bulb: &Bulb<SocketAddr>, power: Power) {
    let _ = bulb.send_msg(
        Payload::Light(SetPower(power, 500)),
        true,
    );
}