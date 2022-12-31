mod rendered;
use yew::prelude::*;

pub use reality::*;
pub use rendered::Rendered;

// Renders a Yew layout.
pub fn render(layout: Html) -> Rendered {
    Rendered::new(layout)
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test as test;

    #[test]
    async fn it_works() -> Result<()> {
        #[function_component]
        fn App() -> Html {
            html! {<div id="matched">{"Hello!"}</div>}
        }

        let rendered = render(html! {<App />});

        let el = rendered.search_text("Hello!").find().await?;

        assert_eq!(el.id(), "matched");

        Ok(())
    }
}
