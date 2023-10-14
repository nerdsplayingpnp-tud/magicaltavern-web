use yew::prelude::*;

pub fn main() {
    yew::Renderer::<App>::new().render();
}

pub struct App {
    value: i64,
}

pub enum Message {
    AddOne,
    RemoveOne,
    Reset,
}

impl Component for App {
    type Message = Message;

    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        App { value: 0 }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                {self.value}
                <button onclick={ctx.link().callback(|_| Message::AddOne)}>{"+1"}</button>
                <button onclick={ctx.link().callback(|_| Message::RemoveOne)}>{"-1"}</button>
                <button onclick={ctx.link().callback(|_| Message::Reset)}>{"Reset"}</button>
            </>
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::AddOne => self.value += 1,
            Message::RemoveOne => self.value -= 1,
            Message::Reset => self.value = 0,
        }
        true
    }
}
