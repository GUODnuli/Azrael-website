use crate::components::footer::GoBack;
use crate::components::footer::HomeFooter;
use crate::error_template::AppError;
use crate::error_template::ErrorTemplate;
use crate::server_functions::posts::Post;
use crate::server_functions::posts::PostContent;
use crate::server_functions::posts::PostMetadata;
use crate::server_functions::posts::PostType;
use leptos::*;
use leptos_meta::*;
use leptos_router::use_params_map;
use std::collections::HashMap;

#[component]
pub fn Post(post_type: PostType, post_description: String) -> impl IntoView {
    let posts =
        use_context::<Resource<(), Result<HashMap<PostType, Vec<Post>>, ServerFnError>>>()
            .expect("unable to find context");
    let title_string = match post_type {
        PostType::Blog => "文章",
        PostType::Project => "项目",
        PostType::Book => "阅读书单",
    };
    view! {
        <Body class="flex flex-col min-h-screen bg-[#000000]"/>

        <div class="max-w-[85rem] px-4 py-10 sm:px-6 lg:px-8 lg:py-14 mx-auto">
            <div class="max-w-2xl mx-auto text-center mb-10 lg:mb-14">
                <h2 class="text-2xl font-bold md:text-4xl md:leading-tight text-[#F8F9FA]">
                    {title_string.to_string()}
                </h2>
                <p class="mt-1  text-[#CED4DA]">{post_description}</p>
            </div>
            <div class="grid sm:grid-cols-2 lg:grid-cols-3 gap-8">
                <Transition fallback=move || {
                    view! { <p>"加载中..."</p> }
                }>
                    {move || {
                        posts
                            .get()
                            .map(|posts| match posts {
                                Ok(posts) => {
                                    posts
                                        .get(&post_type)
                                        .expect("Unable to read the right post_type")
                                        .iter()
                                        .map(|post| {
                                            if post_type == PostType::Project {
                                                view! {
                                                    <ProjectPostCard
                                                        post_metadata=post.post_metadata.clone()
                                                        href=post.post_metadata.project_link.clone()
                                                    />
                                                }
                                            } else {

                                                view! {
                                                    <PostCard
                                                        post_metadata=post.post_metadata.clone()
                                                        path=post_type.to_string()
                                                    />
                                                }
                                            }
                                        })
                                        .collect_view()
                                }
                                Err(e) => {
                                    view! {
                                        <pre class="error">"Server Error: " {e.to_string()}</pre>
                                    }
                                        .into_view()
                                }
                            })
                    }}

                </Transition>
            </div>
        </div>
        <GoBack content="Back to Home".to_string()/>
        <HomeFooter father_name="Post".to_string()/>
    }
}

#[component]
pub fn PostCard(post_metadata: PostMetadata, path: String) -> impl IntoView {
    view! {
        <a
            class="group flex flex-col h-full border transition-all duration-300 rounded-xl p-5 border-gray-700 hover:border-transparent hover:shadow-black/[.4]"
            href=format!("/{}/{}", path, post_metadata.create_href())
        >
            <div class="aspect-w-16 aspect-h-11">
                <img class="w-full object-cover rounded-xl" src=post_metadata.image_path/>
            </div>
            <div class="my-6">
                <h3 class="text-xl font-semibold  text-gray-300 group-hover:text-[#F8F9FA]">
                    {post_metadata.title}
                </h3>
                <h2 class="mt-5 text-gray-400">{post_metadata.date}</h2>
                <p class="mt-5 text-[#CED4DA]">{post_metadata.description}</p>
            </div>
            <div class="mt-auto flex items-center gap-x-3">
                <img class="w-8 h-8 rounded-full" src="/Azrael.ico"/>
                <h5 class="text-sm text-gray-200">"By Azrael."</h5>
            </div>
        </a>
    }
}

#[component]
pub fn ProjectPostCard(post_metadata: PostMetadata, href: String) -> impl IntoView {
    view! {
        <a
            class="group flex flex-col h-full border transition-all duration-300 rounded-xl p-5 border-gray-700 hover:border-transparent hover:shadow-black/[.4]"
            href=href
        >
            <div class="aspect-w-16 aspect-h-11">
                <img class="w-full object-cover rounded-xl" src=post_metadata.image_path/>
            </div>
            <div class="my-6">
                <h3 class="text-xl font-semibold  text-gray-300 group-hover:text-[#F8F9FA]">
                    {post_metadata.title}
                </h3>
                <h2 class="mt-5 text-gray-400">{post_metadata.date}</h2>
                <p class="mt-5 text-[#CED4DA]">{post_metadata.description}</p>
            </div>
            <div class="mt-auto flex items-center gap-x-3">
                <img class="w-8 h-8 rounded-full" src="/Azrael.ico"/>
                <h5 class="text-sm text-gray-200">"By Azrael."</h5>
            </div>
        </a>
    }
}
#[component]
pub fn RenderPost(post_type: PostType) -> impl IntoView {
    let posts =
        use_context::<Resource<(), Result<HashMap<PostType, Vec<Post>>, ServerFnError>>>()
            .expect("unable to find context");
    let params = use_params_map();
    let post_query = move || params.with(|params| params.get("post").cloned().unwrap_or_default());

    view! {
        <Suspense fallback=move || {
            view! { <p>"加载中..."</p> }
        }>
            {move || {
                posts
                    .get()
                    .map(|posts| match posts {
                        Ok(posts) => {
                            let post = posts
                                .get(&post_type)
                                .expect("Unable to read the right post_type")
                                .iter()
                                .find(|&p| p.post_metadata.create_href() == post_query());
                            if let Some(post) = post {
                                view! {
                                    <Title text=post.post_metadata.title.clone()/>
                                    <Meta
                                        name="description"
                                        content=post.post_metadata.description.clone()
                                    />
                                    <PostLayout content=post.post_content.clone() post_type=post_type/>
                                }
                                .into_view()
                            } else {
                                let mut outside_errors = Errors::default();
                                outside_errors.insert_with_default_key(AppError::NotFound);

                                view! { <ErrorTemplate outside_errors/> }
                                    .into_view()
                            }
                        }
                        Err(e) => {
                            view! { <pre class="error">"Server Error: " {e.to_string()}</pre> }
                                .into_view()
                        }
                    })
            }}
        </Suspense>
    }
}

#[component]
pub fn PostLayout(content: PostContent, post_type: PostType) -> impl IntoView {
    let mut back_string = String::from("Back to Home");
    match post_type {
        PostType::Blog => {
            back_string = "Back to Posts".to_string();
        },
        PostType::Book => {
            back_string = "Back to Books".to_string();
        },
        _ => {}
    }
    view! {
        <div class="bg-[#000000] min-h-screen flex flex-col">
            <div class="max-w-3xl px-4 pt-6 lg:pt-10 pb-12 sm:px-6 lg:px-8 mx-auto flex-grow">
                <div class="max-w-3xl">
                    <div
                        class="prose prose-blog mx-auto md:prose-lg prose-pre:m-0 prose-pre:rounded-none"
                        inner_html=content
                    ></div>

                </div>
            </div>
            <GoBack content=back_string/>
            <HomeFooter father_name="PostLayout".to_string()/>
        </div>
    }
}
