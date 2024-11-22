use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use crate::component::blog_post::BlogPost;

use crate::model::blog_post::Post;

#[derive(Params, PartialEq, Clone, Serialize, Deserialize, Debug)]
struct EditPostParams {
    post_id: Option<String>,
}

#[component]
pub fn EditPost() -> impl IntoView {
    let params = use_params::<EditPostParams>();
    let (post, set_post) = create_signal(Post::new_empty());
    view! {
        <div class="flex h-screen">
            <div class="min-w-[50%] max-h-[90%] text-gray-200 dark:bg-gray-800 bg-gray-100 p-10 rounded-md">
                <form>
                    <label class="block mb-4">
                        <span>Title</span>
                        <input class="mt-1 p-2 w-full" type="text" id="title" name="title"
                            on:input=move |ev| {
                                set_post.update(|post| post.title = event_target_value(&ev))
                            }
                            prop:value={move || post.get().title} />
                    </label>
                    <label class="block mb-4">
                        <span>Entry</span>
                        <textarea class="mt-1 p-2 w-full" id="text" name="text"
                            on:input=move |ev| {
                                set_post.update(|post| post.text = event_target_value(&ev))
                            }
                        />
                    </label>
                </form>
            </div>
            <div>
            { move || view ! {<BlogPost post=post.get() />}}
            </div>
        </div>
    }
}