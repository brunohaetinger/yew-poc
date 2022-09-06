
mod hello_world;
mod counter;

use yew::prelude::*;
use hello_world::HelloWorld;
use counter::Counter;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "App!" }</h1>
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
            <HelloWorld/>
            <h1>{ "Counter:" }</h1>
            <Counter/>
        </main>
    }
}