use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let counter = create_rw_signal(0);

    view! {

        <h1> Hello World! {counter}</h1>

        <button on:click=move |_event| {
            counter.update(|value| *value = *value + 1);
            log::info!("Button clicked")
        }
        > Ok </button>

    }
}
