use yew::prelude::*;
use wasm_bindgen::prelude::*;
// Define the component's state
#[derive(Clone, PartialEq)]
pub struct Counter {
    count: i32,
}

// Define messages that can be sent to the component
pub enum Msg {
    Increment,
    Decrement,
    Reset,
}

// Implement the Component trait
impl Component for Counter {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { count: 0 }
    }

    // Handle messages and update state
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Increment => {
                self.count += 1;
                true // re-render
            }
            Msg::Decrement => {
                self.count -= 1;
                true // re-render
            }
            Msg::Reset => {
                self.count = 0;
                true // re-render
            }
        }
    }

    // Render the component
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <div class="counter">
                <h1>{ "Counter App" }</h1>
                <div class="count-display">
                    <span class="count">{ self.count }</span>
                </div>
                <div class="buttons">
                    <button onclick={link.callback(|_| Msg::Decrement)}>
                        { "-" }
                    </button>
                    <button onclick={link.callback(|_| Msg::Reset)}>
                        { "Reset" }
                    </button>
                    <button onclick={link.callback(|_| Msg::Increment)}>
                        { "+" }
                    </button>
                </div>
                <div class="info">
                    <p>{ format!("Current count: {}", self.count) }</p>
                    <p>{ 
                        if self.count > 0 { 
                            "Positive!" 
                        } else if self.count < 0 { 
                            "Negative!" 
                        } else { 
                            "Zero!" 
                        }
                    }</p>
                </div>
            </div>
        }
    }
}

// Main app component
#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <Counter />
        </div>
    }
}

// Entry point
#[wasm_bindgen(start)]
pub fn main() {
    yew::Renderer::<App>::new().render();
}