use std::net::Ipv6Addr;

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
        <DynamicList initial_length=5></DynamicList>
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

#[component]
fn DynamicList(initial_length: u8) -> impl IntoView {
    let mut next_counter_id =initial_length;
    let initial_counters = (0..initial_length)
        .map(|id| (id, ArcRwSignal::new(id+1)))
        .collect::<Vec<_>>();
    let (counters, set_counters) = signal(initial_counters);
    let add_counter = move |_| {
        let sig = ArcRwSignal::new(next_counter_id + 1);
        set_counters.update(move |counters| {
            counters.push((next_counter_id, sig));
        });
        next_counter_id += 1;
    };
    
    view! {
        <div>
            <button on:click=add_counter>
                "Add counter"
            </button>
            <ul>
                <For
                    each=move || counters.get()
                    key=|counter| counter.0
                    children=move |(id, count)| {
                        let count = RwSignal::from(count);
                        view! {
                            <li>
                                <button
                                    on:click=move |_| *count.write() += 1
                                >
                                    {count}
                                </button>
                                <button
                                    on:click=move |_| {
                                        set_counters
                                            .write()
                                            .retain(|(counter_id, _)| {
                                                counter_id != &id
                                            });
                                    }
                                >
                                    "Remove"
                                </button>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}