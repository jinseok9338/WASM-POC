

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::post::{post::*, post_server::list_post_metadata};


#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        <Router>
            <main>
                <Routes>
                    // We’ll load the home page with out-of-order streaming and <Suspense/>
                    <Route path="" view=|cx| view! { cx, <HomePage/> } ssr=SsrMode::Async/>

                    // We'll load the posts with async rendering, so they can set
                    // the title and metadata *after* loading the data
                    <Route
                        path="/post/:id"
                        view=|cx| view! { cx, <Post/> }
                        ssr=SsrMode::Async
                    />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // load the posts
    let posts =
        create_resource(cx, || (), |_| async { list_post_metadata().await });
    let posts_view = move || {
        posts.with(cx, |posts| posts
            .clone()
            .map(|posts| {
                posts.iter()
                .map(|post| view! { cx, <li><a href=format!("/post/{}", post.id)>{&post.title}</a></li>})
                .collect::<Vec<_>>()
            })
        )
    };


    view! { cx,
        <h1>"My Great Blog"</h1>
        <Suspense   fallback=move || view! { cx, <p>"Loading posts..."</p> }   >
            <ul>{posts_view}</ul>
        </Suspense>
    }
}
