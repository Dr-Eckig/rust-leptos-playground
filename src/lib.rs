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
            <div>
                <span>{ title }</span>" "<span>{ date }</span>
            </div>
        }
    }).collect::<Vec<_>>();
    view! {

        <h1> Hello World! {counter}</h1>

        <button on:click=move |_event| {
            counter.update(|value| *value = *value + 1);
            log::info!("Button clicked")
        }
        > Ok </button>

        { meeting_views }
    }
}
