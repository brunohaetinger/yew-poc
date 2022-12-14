use yew::{Component, Context, Html, html};

pub enum Msg {
    AddOne,
    SubOne,
}

pub struct Counter {
    value: i64,
}

impl Component for Counter {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            },
            Msg::SubOne => {
                self.value -= 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div class="counter-container">
                <button onclick={link.callback(|_| Msg::SubOne)}>{ "➖" }</button>
                <p>
                    <b>{ "Current value: " }</b>
                    { self.value }
                </p>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "➕" }</button>
            </div>
        }
    }
}