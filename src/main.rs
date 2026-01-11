use leptos::{prelude::*};

fn main() {
    leptos::mount::mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    let pages = (vec!["Home", "About", "Shop"]).iter().map(|s| s.to_string()).collect();;
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
                        .map(|p| view!{ <a href="{p}"><li>{p}</li></a> })
                        .collect::<Vec<_>>()}
                </ul>
            </nav>
        </header>
    }
}