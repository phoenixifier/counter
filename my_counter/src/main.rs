use yew::prelude::*;

enum Msg {
    AddOne
}
struct Counter {
    count: i64
}

impl Component for Counter{
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg{
            Msg::AddOne => {
                self.count += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html!{
            <div class="container">
                <p>{self.count}</p>
                <button onclick = {link.callback(|_| Msg::AddOne)}>{"+1"}</button>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<Counter>::new().render();
}
