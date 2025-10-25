use {
    crate::shared::wini::err::ServerResult,
    maud::{html, Markup},
    wini_macros::layout,
};

#[layout]
pub async fn render(s: Markup) -> ServerResult<Markup> {
    Ok(html! {
        header {
            "Welcome to Wini!"
        }
        (s)
    })
}
