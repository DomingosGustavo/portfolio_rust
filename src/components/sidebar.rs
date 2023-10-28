// src/components/navbar.rs
use yew::prelude::*;

#[function_component(SideBar)]
pub fn view() -> Html {
    let is_open = use_state(|| true);

    let toggle_sidebar = { 
        let is_open_clone = is_open.clone();
        Callback::from(move |_| is_open_clone.set(!*is_open_clone)) 
    };
    html! {
    <>
        {
            if *is_open {
                html! {
                    <div class="h-screen bg-black w-64 fixed left-0 top-0 overflow-auto shadow-xl shadow-fuchsia-600">
                        <div class="flex flex-row p-6">
                            <h1 class="text-7xl mb-5 flex-1 font-boogieeee text-green-500">{"My App"}</h1>
                            <button
                                class="p-3 bg-blue-500 text-white right-0 rounded flex-2 h-3"
                                onclick={toggle_sidebar}>
                            </button>
                        </div>
                        <ul class="list-none p-4">
                            <li class="mb-3 flex items-center hover:bg-gray-100 p-2 rounded">
                                <span class="text-blue-500 mr-3">
                                    <i class="fas fa-home"></i>
                                </span>
                                <a href="#" class="text-gray-700 hover:text-gray-900">{"Home"}</a>
                            </li>
                            <li class="mb-3 flex items-center hover:bg-gray-100 p-2 rounded">
                                <span class="text-blue-500 mr-3">
                                    <i class="fas fa-info-circle"></i>
                                </span>
                                <a href="#" class="text-gray-700 hover:text-gray-900">{"About"}</a>
                            </li>
                            <li class="mb-3 flex items-center hover:bg-gray-100 p-2 rounded">
                                <span class="text-blue-500 mr-3">
                                    <i class="fas fa-envelope"></i>
                                </span>
                                <a href="#" class="text-gray-700 hover:text-gray-900">{"Contact"}</a>
                            </li>
                        </ ul>
                    </div>
                }
            } else {
                html! {
                    <div class="h-screen bg-white w-15 fixed left-0 top-0 overflow-auto shadow-lg flex p-6 transition-all -translate-x-5" >
                        <button
                            class="p-3 bg-blue-500 text-white h-2 rounded flex-1"
                            onclick={toggle_sidebar}>
                        </button>
                    </div>
                }
            }
        }
        </>
    }
}
