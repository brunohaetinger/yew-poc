use yew::{function_component, html};

#[function_component(HelloWorld)]
pub fn hello_world() -> Html {
    html! {
        <h1>{ "Hello World from functional component!" }</h1>
    }
}