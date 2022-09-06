mod app;

use app::App;

fn main() {
    // starts your application and mounts it to the page's <body> tag
    yew::start_app::<App>();
}