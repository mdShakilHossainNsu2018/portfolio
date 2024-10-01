use yew::prelude::*;

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self{}
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="container">
            <div>
             // { self.view_info_title(_ctx) }
            </div>
            <h1 class="underline">{ "Hello World! check" }</h1>
            <p>{ "Welcome to the Yew App! check" }</p>
            </div>
        }
    }
}
// impl Home {
//     fn view_info_title(&self, _ctx: &Context<Self>) -> Html {
//         html! {
//             <h1 class="underline">{ "Hello World!" }</h1>
//         }
//     }
// }
//
// #[function_component(App)]
// pub fn home() -> Html {
//     html! {
//         <main>
//
//             <h1 class="underline">{ "Hello World!" }</h1>
//             <p>{ "Welcome to the Yew App! check" }</p>
//         </main>
//     }
// }
