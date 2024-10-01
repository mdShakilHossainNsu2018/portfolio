use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
           
            <h1 class="underline">{ "Hello World!" }</h1>
            <p>{ "Welcome to the Yew App! test" }</p>
        </main>
    }
}
