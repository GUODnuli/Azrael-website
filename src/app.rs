use crate::{
    error_template::{AppError, ErrorTemplate},
    routes::blog::blog_article::RenderBlogPost,
    routes::{
        about::About,
        blog::blog_section::BlogPost,
        books::{books_article::RenderBooksPost, books_section::BooksPost},
        // hire_me::HireMe,
        home::Home,
        projects::{projects_article::RenderProjectsPost, projects_section::ProjectsPost},
    },
    server_functions::posts::get_posts,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let posts = create_resource(|| (), |_| async move { get_posts().await });
    provide_context(posts);

    view! {
        <Html lang="zh-CN"/>
        <Stylesheet id="leptos" href="/pkg/azrael-website.css"/>
        <Link rel="shortcut icon" type_="image/png" href="/Azrael.ico"/>
        <Link rel="preconnect" href="https://fonts.googleapis.com"/>
        <Link rel="preconnect" href="https://fonts.gstatic.com"/>
        <Link
            href="https://fonts.googleapis.com/css2?family=Inter:wght@200;500;700&display=swap"
            rel="stylesheet"
        />
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <Routes>
                <Route
                    path=""
                    view=move || {
                        view! { <Home/> }
                    }
                />

                <Route
                    path="/blog"
                    view=move || {
                        view! { <BlogPost/> }
                    }
                />

                <Route
                    path="/blog/:post"
                    view=move || {
                        view! { ,
                            <Link rel="stylesheet" href="/highlighter/styles/github.min.css"/>
                            <script src="/highlighter/load_highlight.js"></script>
                            <RenderBlogPost/>
                        }
                    }
                />

                <Route
                    path="/about"
                    view=move || {
                        view! { <About/> }
                    }
                />

                // <Route
                //     path="/hire-me"
                //     view=move || {
                //         view! { <HireMe/> }
                //     }
                // />

                <Route
                    path="/books"
                    view=move || {
                        view! { <BooksPost/> }
                    }
                />

                <Route
                    path="/books/:post"
                    view=move || {
                        view! { <RenderBooksPost/> }
                    }
                />

                <Route
                    path="/projects"
                    view=move || {
                        view! { <ProjectsPost/> }
                    }
                />

                <Route
                    path="/projects/:post"
                    view=move || {
                        view! { <RenderProjectsPost/> }
                    }
                />

            </Routes>
            <script src="/preline/preline.js"></script>
        </Router>
    }
}
