use leptos::prelude::*;
use leptos_meta::{Stylesheet, Title, provide_meta_context};
use leptos_router::StaticSegment;
use leptos_router::components::{Route, Router, Routes};

use crate::components::navbar::Navbar;
use crate::routing::page_home::HomePage;
use crate::routing::page_test::TestPage;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/starter_leptos_ssr_workspace.css" />
        <Title text="Workspace template" />

        <Router>
            <Navbar />
            <div class="min-h-screen">
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("/") view=HomePage />
                    <Route path=StaticSegment("/test") view=TestPage />
                </Routes>
            </div>
        </Router>
    }
}
