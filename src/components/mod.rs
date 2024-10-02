pub mod meeting_view {
    use leptos::{component, view, IntoView};

    #[component]
    pub fn MeetingView(title: String, date: String) -> impl IntoView {
        view! {
            <div class="message is-primary">
                <div class="message-header">
                    <div class="title is-5">{ title }</div>
                    <div class="subtitle is-5">{ date }</div>
                </div>
                <div class="message-body">
                    Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.
                </div>
            </div>
        }
    }
}

pub mod buttons {
    use leptos::*;

    #[component]
    pub fn GeneralButton<F>(on_click: F) -> impl IntoView
    where F: Fn() -> () + 'static
    {
        view! {
            <button class="button" on:click=move |_event| on_click()> Ok </button>
            }
    }

}
