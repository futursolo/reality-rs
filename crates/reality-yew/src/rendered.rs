use std::ops::Deref;

use reality::screen;
use reality::Screen;
use yew::html::ChildrenProps;
use yew::prelude::*;
use yew::AppHandle;

#[function_component]
pub fn Wrapper(props: &ChildrenProps) -> Html {
    html! {<>{props.children.clone()}</>}
}

/// The rendered result of a Yew layout.
///
/// This is an RAII guard, dropping this type will result in the layout being unmounted.
#[derive(Debug)]
pub struct Rendered {
    handle: Option<AppHandle<Wrapper>>,
    screen: Screen,
}

impl Rendered {
    pub(crate) fn new(layout: Html) -> Self {
        let screen = screen();
        let handle = yew::Renderer::with_props(ChildrenProps {
            children: Children::new(vec![layout]),
        })
        .render();

        Self {
            handle: Some(handle),
            screen,
        }
    }
}

impl Deref for Rendered {
    type Target = Screen;

    fn deref(&self) -> &Self::Target {
        &self.screen
    }
}

impl Drop for Rendered {
    fn drop(&mut self) {
        if let Some(m) = self.handle.take() {
            m.destroy();
        }
    }
}
