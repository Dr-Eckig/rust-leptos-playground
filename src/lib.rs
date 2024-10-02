use leptos::*;

struct Meeting {
    title: String,
    date: String
}

impl Meeting {

    fn new(title: &str, date: &str) -> Self {
        Self {
            title: String::from(title),
            date: String::from(date)
        }
    }
}
#[component]
pub fn App() -> impl IntoView {
    let counter = create_rw_signal(0);
    let meetings = vec![
        Meeting::new("Meeting 1", "01.01.204"),
        Meeting::new("Meeting 2", "01.01.2028"),
        Meeting::new("Meeting 3", "24.10.2023")
    ];
    let meeting_views = meetings.into_iter().map(|meeting| {
        let title = String::from(meeting.title);
        let date = String::from(meeting.date);
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
    }).collect::<Vec<_>>();
    view! {

        <h1> Hello World! {counter}</h1>

        <button class="button" on:click=move |_event| {
            counter.update(|value| *value = *value + 1);
            log::info!("Button clicked")
        }
        > Ok </button>

        { meeting_views }
    }
}
