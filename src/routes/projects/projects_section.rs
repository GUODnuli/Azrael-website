use crate::components::post::Post;

use crate::server_functions::posts::PostType;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn ProjectsPost() -> impl IntoView {
    view! {
        <Title text="Azrael Projects"/>
        <Meta name="description" content="My Projects Posts."/>
        <Post
            post_type=PostType::Project
            post_description="个人项目.".to_string()
        />
    }
}
