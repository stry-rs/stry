use stry_common::models::{
    story::{Rating, State, Story, StoryTag, TagKind, TagLevel},
    Existing,
};

use windswept::{rsx, Render};

pub fn media_object<L, T, S, M>(tile: L, title: T, sub: S, meta: M) -> impl Render
where
    L: Render,
    T: Render,
    S: Render,
    M: Render,
{
    rsx! {
        <div class="flex">
            <div>
                {tile}
            </div>
            <div class="flex-1 flex flex-col">
                <div class="text-base">
                    {title}
                </div>
                <div class="text-sm">
                    {sub}
                </div>
            </div>
            <div>
                {meta}
            </div>
        </div>
    }
}

pub fn tag<'r, T, S>(tag: &'r T, url: &'r str, slot: S) -> impl Render + 'r
where
    T: StoryTag,
    S: Render + 'r,
{
    macro_rules! class {
        ($ext:expr) => {
            concat!("inline-block text-sm mr-1.5 mb-1.5 px-2 py-0.5 transition-colors duration-75 rounded focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-900 focus:ring-blue-400", " ", $ext)
        };
    }

    let class = match (tag.kind(), tag.level()) {
        (TagKind::Warning, _) => class!("bg-red-400 bg-red-500"),
        (TagKind::General, _) => class!("bg-gray-400 bg-gray-500"),
        (TagKind::Pairing, TagLevel::Major) => class!("bg-yellow-400 bg-yellow-500"),
        (TagKind::Pairing, TagLevel::Minor) => {
            class!("gradient-stripes-yellow-400 gradient-stripes-yellow-500")
        }
        (TagKind::Character, TagLevel::Major) => class!("bg-blue-400 bg-blue-500"),
        (TagKind::Character, TagLevel::Minor) => {
            class!("gradient-stripes-blue-400 gradient-stripes-blue-500")
        }
    };

    rsx! {
        <a href={url} class={class}>{slot}</a>
    }
}

pub fn nav() -> impl Render {
    rsx! {
        <nav class="flex flex-wrap">
            <a class="order-1 inline-block flex-1 py-3 px-2 font-mono font-bold text-zinc-200 md:flex-initial" href="#">"stry"</a>
            <ul class="order-3 flex flex-1 basis-full overflow-x-auto md:order-2 md:basis-auto md:overflow-x-hidden">
                <li><a class="inline-block cursor-pointer py-3 px-2 text-zinc-400 transition-colors duration-75 ease-in-out hover:text-zinc-50" href="#">"authors"</a></li>
                <li><a class="inline-block cursor-pointer py-3 px-2 text-zinc-400 transition-colors duration-75 ease-in-out hover:text-zinc-50" href="#">"origins"</a></li>
                <li><a class="inline-block cursor-pointer py-3 px-2 text-zinc-400 transition-colors duration-75 ease-in-out hover:text-zinc-50" href="#">"warnings"</a></li>
                <li><a class="inline-block cursor-pointer py-3 px-2 text-zinc-400 transition-colors duration-75 ease-in-out hover:text-zinc-50" href="#">"pairings"</a></li>
                <li><a class="inline-block cursor-pointer py-3 px-2 text-zinc-400 transition-colors duration-75 ease-in-out hover:text-zinc-50" href="#">"characters"</a></li>
                <li><a class="inline-block cursor-pointer py-3 px-2 text-zinc-400 transition-colors duration-75 ease-in-out hover:text-zinc-50" href="#">"tags"</a></li>
            </ul>
            <ul class="order-2 flex md:order-3">
                <li><a class="inline-block cursor-pointer py-3 px-2 text-zinc-400 transition-colors duration-75 ease-in-out hover:text-zinc-50" href="#">"register"</a></li>
                <li><a class="inline-block cursor-pointer py-3 px-2 text-zinc-400 transition-colors duration-75 ease-in-out hover:text-zinc-50" href="#">"sign-in"</a></li>
            </ul>
        </nav>
    }
}

pub fn story_tile(rating: Rating, warning: bool, state: State) -> impl Render {
    rsx! {
        <a href="/" class="w-9 h-9 mt-1.5 mr-3 flex flex-col rounded focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-900 focus:ring-blue-400">
            <div class="flex">
                <div class={match rating {
                        Rating::Explicit => "block w-4 h-4 mr-1 rounded bg-red-400",
                        Rating::Mature => "block w-4 h-4 mr-1 rounded bg-yellow-400",
                        Rating::Teen => "block w-4 h-4 mr-1 rounded bg-blue-400",
                        Rating::General => "block w-4 h-4 mr-1 rounded bg-gray-100",
                }}>
                    <span class="sr-only">"Rating"</span>
                </div>
                <div class={match warning {
                    true => "block w-4 h-4 rounded bg-red-400",
                    false => "block w-4 h-4 rounded bg-gray-100",
                }}>
                    <span class="sr-only">"Warning"</span>
                </div>
            </div>
            <div class="flex justify-center">
                <div class={match state {
                    State::Completed => "block w-4 h-4 mt-1 rounded bg-green-400",
                    State::InProgress => "block w-4 h-4 mt-1 rounded bg-blue-400",
                    State::Hiatus => "block w-4 h-4 mt-1 rounded bg-yellow-400",
                    State::Abandoned => "block w-4 h-4 mt-1 rounded bg-red-400",
                }}>
                    <span class="sr-only">"State"</span>
                </div>
            </div>
        </a>
    }
}

pub fn story(story: &Existing<Story>) -> impl Render + '_ {
    rsx! {
        <div class="px-3 sm:px-6 lg:px-8 my-2">
            {media_object(
                story_tile(story.rating, !story.warnings.is_empty(), story.state),
                rsx! {
                    <p class="text-base"></p>
                },
                rsx! {
                    <p class="text-sm"></p>
                },
                rsx! {
                    <p class="text-sm text-opacity-60 text-white"></p>
                },
            )}
            <div class="pt-2 text-sm text-opacity-60 text-white"></div>
            <div class="text-sm">
                <ul class="flex flex-wrap">
                    {for warning in &story.warnings {
                        rsx! {
                            <li>{tag(warning, "", &warning.content)}</li>
                        }
                    }}
                    {for pairing in &story.pairings {
                        rsx! {
                            <li>{tag(pairing, "", "")}</li>
                        }
                    }}
                    {for character in &story.characters {
                        rsx! {
                            <li>{tag(character, "", &character.content)}</li>
                        }
                    }}
                    {for general in &story.tags {
                        rsx! {
                            <li>{tag(general, "", &general.content)}</li>
                        }
                    }}
                </ul>
            </div>
            <div class="text-sm text-opacity-60 text-white flex">
                <p class="flex-grow"></p>
                <p></p>
            </div>
        </div>
    }
}
