mod app;
mod home;
mod about;

use app::App;
use leptos::*;

fn main() {
    mount_to_body(App)
}
