use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

use crate::component::{blog_previews::BlogPreviews, edit_post::EditPost, view_post::ViewPost};

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <div class="dark:bg-gray=800 text-white p-4">
            <div class="container mx-auto flex justify-between items-center">
                <a href="/" class="text-2xl font-bold">Moonbound</a>
                <nav>
                    <ul class="flex space-x-4">
                        <li><a href="/" class="hover:text-blue-400">Blog</a></li>
                        <li><a href="/edit" class="hover:text-blue-400">Create</a></li>
                    </ul>
                </nav>
            </div>
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/moonbound.css"/>
        <Title text="Welcome to Leptos 2"/>

        <Navbar />

        <Router>
            <main class="bg-gray-700 text-gray-200 p-8 h-full">
                <Routes fallback=|| "Not found">
                    <Route path=path!("") view=BlogPreviews />
                    <Route path=path!("/edit/:post_id?") view=EditPost />
                    <Route path=path!("/view/:post_id?") view=ViewPost />
                </Routes>
            </main>
        </Router>
    }
}
