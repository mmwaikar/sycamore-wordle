use sycamore::prelude::*;

#[component]
pub fn Counter() -> View {
    let mut counter = create_signal(1);
    let doubled = create_memo(move || counter.get() * 2);
    let increment = move |_| counter += 1;
    let decrement = move |_| counter -= 1;

    view! {
        button(on:click=increment) { "Increment" }
        button(on:click=decrement) { "Decrement" }
        p { "Count:" (counter) }
        p { "Doubled: " (doubled) }
    }
}

#[component]
fn App() -> View {
    view! {
        div {
            h1 { "Hello, Sycamore!" }
            Counter()
        }
    }
}

fn main() {
    sycamore::render(App);
}
