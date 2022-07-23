use stry_common::models::{story::Story, Existing};
use windswept::{rsx, Render};

pub fn index(stories: &[Existing<Story>]) -> impl Render + '_ {
    crate::templates::base(rsx! {
        <>
        {for story in stories {
            rsx! {
                {crate::templates::partials::story(story)}

                <div class="hidden sm:block sm:px-6 lg:px-8 text-sm" aria-hidden="true">
                    <div class="border-t border-gray-700"></div>
                </div>
            }
        }}
        </>
    })
}
