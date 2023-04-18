use yew::prelude::*;

fn main() {
    // println!("Hello, world!");
    yew::Renderer::<App>::new().render();
}

#[function_component(App)] // compoment name is App: creates a struct called App and with all athings needed
fn app() -> Html {
    html! {
        <h1>{"Hello World"}</h1>
    }
}
