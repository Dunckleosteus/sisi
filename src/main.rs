use gloo::console::log;
use yew::prelude::*;
fn main() {
    yew::Renderer::<App>::new().render();
}

// creating first component
#[function_component(App)]
fn app() -> Html {
    let message: Option<String> = Some(String::from("Hello"));
    let tasks = vec!["Say Hello to Blob", "Sisi stab 100 people", "War crimes"];
    log!("Hello world");
    html! {
        <> // fragment start
            <h1>{"Hello world"}</h1>
            <p>{"Hello sisi"}</p>
            // if statement in html
            if true {
                <p>{"True"}</p>
            }
            // this is a let some message
            if let Some(value) = message{
                <p>{"There is a message"}</p>
                <p>{value}</p>
            }
            // loops
            <ul>
            {tasks.iter().map(|x| html! {<li>{x}</li>}).collect::<Html>()}
            </ul>
        </> // fragment end
    }
}
// components allow for things to be seperated into parts
