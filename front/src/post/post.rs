use leptos_router::IntoParam;
use leptos_meta::MetaProps;
use leptos_meta::Meta;
use leptos_meta::TitleProps;
use leptos_meta::Title;
use leptos_router::use_params;
use leptos::*;
use leptos_router::Params;

use crate::post::post_server::PostError;
use crate::post::post_server::get_post;

#[derive(Params, Copy, Clone, Debug, PartialEq, Eq)]
pub struct PostParams {
    id: usize,
}

#[component]
pub fn Post(cx: Scope) -> impl IntoView {
    let query = use_params::<PostParams>(cx);
    let id = move || {
        query.with(|q| {
            q.as_ref().map(|q| q.id).map_err(|_| PostError::InvalidId)
        })
    };
    let post = create_resource(cx, id, |id| async move {
        match id {
            Err(e) => Err(e),
            Ok(id) => get_post(id)
                .await
                .map(|data| data.ok_or(PostError::PostNotFound))
                .map_err(|_| PostError::ServerError)
                .flatten(),
        }
    });

    let post_view = move || {
        post.with(cx, |post| {
            post.clone().map(|post| {
                view! { cx,
                    // render content
                    <h1>{&post.title}</h1>
                    <p>{&post.content}</p>

                    // since we're using async rendering for this page,
                    // this metadata should be included in the actual HTML <head>
                    // when it's first served
                    <Title text=post.title/>
                    <Meta name="description" content=post.content/>
                }
            })
        })
    };


    view! { cx,
        <Suspense fallback=move || view! { cx, <p>"Loading post..."</p> }>
            <ErrorBoundary fallback=|cx, errors| {
                view! { cx,
                    <div class="error">
                        <h1>"Something went wrong."</h1>
                        <ul>
                        {move || errors.get()
                            .into_iter()
                            .map(|(_, error)| view! { cx, <li>{error.to_string()} </li> })
                            .collect::<Vec<_>>()
                        }
                        </ul>
                    </div>
                }
            }>
                {post_view}
            </ErrorBoundary>
        </Suspense>
    }
}
