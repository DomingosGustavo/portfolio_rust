use yew::prelude::*;
use yew_router::prelude::*;

pub mod about;
pub mod home;

use crate::components::{nav::Nav, sidebar::SideBar};
use about::About;
use home::Home;

/// App routes
#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum AppRoute {
    #[at("/about")]
    About,
    #[not_found]
    #[at("/page-not-found")]
    PageNotFound,
    #[at("/")]
    Home,
}

/// Switch app routes
pub fn switch(routes: AppRoute) -> Html {
    match routes.clone() {
        AppRoute::Home => html! { <Home /> },
        AppRoute::About => html! { <About /> },
        AppRoute::PageNotFound => html! { "Page not found" },
    }
}

/// Root app component


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="bg-black bg-grid-pattern bg-40 color-white">
        <HashRouter>
        <SideBar />
            <div class="flex min-h-screen flex-col ml-64 p-8">
                <Nav />
                <Switch<AppRoute> render={switch} />
            </div>
        </HashRouter>
        </div>
    }
}
