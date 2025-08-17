use {
    cached::proc_macro::cached,
    maud::{Markup, html},
    wini_macros::{init_cache, page},
};

#[init_cache]
#[page]
#[cached]
pub async fn render() -> Markup {
    html! {
        button #hello {
            "Say hello!"
        }
    }
}
