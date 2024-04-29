use crate::components::post::Post;

use crate::server_functions::posts::PostType;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn BooksPost() -> impl IntoView {
    view! {
        <Title text="Azrael Books"/>
        <Meta name="description" content="My Books Posts."/>
        <Post post_type=PostType::Book post_description="墨香余韵.".to_string()/>
    }
}
