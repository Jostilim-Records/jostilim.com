use leptos::{prelude::*};

fn main() {
    leptos::mount::mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    let pages = (vec!["Home", "About", "Shop"]).iter().map(|s| s.to_string()).collect();
    view! {
        <Header pages=pages></Header>
    }
}

#[component]
fn Header(pages: Vec<String>) -> impl IntoView {
    view! {
        <header>
            <nav>
                <ul>
                    {pages.into_iter()
                        .map(|p| view!{ <a href={p.clone().to_lowercase()}><li>{p.clone()}</li></a> })
                        .collect_view()}
                </ul>
            </nav>
        </header>
        <StaticList length=5></StaticList>
    }
}

#[component]
fn StaticList(length: u8) -> impl IntoView {
    let counters = (1..=length).map(|id| RwSignal::new(id));
    let counter_buttons = counters
        .map(|count| {
            view! {
                <li>
                    <button
                        on:click=move |_| *count.write() += 1
                    >
                        {count}
                    </button>
                </li>
            }
        })
        .collect_view();

    view! {
        <ul>{counter_buttons}</ul>
    }
}