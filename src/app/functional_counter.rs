use yew::{Callback, function_component, html, use_state};

#[function_component(FunctionalCounter)]
pub fn functionalCounter() -> Html {
    let counter = use_state(|| 0);
    let on_sub_click = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter - 1))
    };
    let on_add_click = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };


    html! {
        <div class="counter-container">
            <button onclick={on_sub_click}>{ "➖" }</button>
            <p>
                <b>{ "Current value: " }</b>
                { *counter }
            </p>
            <button onclick={on_add_click}>{ "➕" }</button>
        </div>
    }
}