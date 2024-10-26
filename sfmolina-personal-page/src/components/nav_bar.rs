//-------------------------------------------------------------------//
//  AUTHOR:    @sfmolina                                            //
//  Version:   v1                                                  //
//  Modified:  23oc24                                             //
//---------------------------------------------------------------//



use yew::prelude::*;
use yew_router::prelude::*;
use crate::{
    router::Route,
    global_ctx::GlobalContext,
    translations::{
        ENGLISH_TEXTS, SPANISH_TEXTS, LanguageVariant
    }
};



#[function_component(Navbar)]
pub fn navbar() -> Html {

    let global_ctx = use_context::<GlobalContext>().expect("GlobalContext not found");
    let language = global_ctx.language;

    let on_language_change = {
        let global_ctx = global_ctx.clone();
        Callback::from(move |lang: LanguageVariant| {
            let new_lang = match lang {
                LanguageVariant::Spanish => &SPANISH_TEXTS,
                LanguageVariant::English => &ENGLISH_TEXTS,
            };
            global_ctx.dispatch(new_lang);
        })
    };

    let current_route: Option<Route> = use_route();

    html! {
        <nav class="navbar navbar-expand-lg navbar-light bg-transparent fixed-top">
            <div class="container-fluid">
                <Link<Route> classes={classes!("navbar-brand")} to={Route::Root}>
                    {language.author_username}
                </Link<Route>>
                <button 
                    class={
                        match current_route {
                            Some(Route::AboutMe) => {
                                "navbar-toggler"
                            }
                            _ => {
                                "navbar-toggler white"
                            }
                        }
                    } 
                    type="button" 
                    data-bs-toggle="collapse" 
                    data-bs-target="#navbarNav" 
                    aria-controls="navbarNav" 
                    aria-expanded="false" 
                    aria-label="Toggle navigation"
                >
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div class="collapse navbar-collapse" id="navbarNav">
                    <ul class="navbar-nav me-auto">
                        <li class="nav-item">
                            <Link<Route> classes={
                                match current_route {
                                    Some(Route::AboutMe) => {
                                        classes!("nav-link", "activated")
                                    }
                                    _ => {
                                        classes!("nav-link")
                                    }
                                }
                            } to={Route::AboutMe}>
                                {language.nav.about_me}
                            </Link<Route>>
                        </li>
                        <li class="nav-item">
                            <Link<Route> classes={
                                match current_route {
                                    Some(Route::Projects) => {
                                        classes!("nav-link", "activated")
                                    }
                                    _ => {
                                        classes!("nav-link")
                                    }
                                }
                            } to={Route::Projects}>
                                {language.nav.projects}
                            </Link<Route>>
                        </li>
                        <li class="nav-item dropdown">
                            <button 
                                class="nav-link dropdown-toggle projects-color"
                                id="navbarDropdown" 
                                data-bs-toggle="dropdown" 
                                aria-expanded="false"
                            >
                                {language.nav.misc}
                            </button>
                            <ul class="dropdown-menu dropdown-menu-end" aria-labelledby="navbarDropdown">
                                <li>
                                    <a 
                                        class={"nav-link"} 
                                        href="https://sfmolina.github.io/supertres/"
                                    >
                                        {language.nav.super_tres}
                                    </a>
                                </li>
                            </ul>
                        </li>
                    </ul>
                    <ul class="navbar-nav">
                        <li class="nav-item dropdown">
                            <button 
                                class={
                                    match current_route {
                                        Some(Route::AboutMe) => {"nav-link dropdown-toggle default-color"},
                                        _ => {"nav-link dropdown-toggle projects-color"}
                                    }
                                } 
                                id="navbarDropdown" 
                                data-bs-toggle="dropdown" 
                                aria-expanded="false"
                            >
                                {language.nav.language}
                            </button>
                            <ul class="dropdown-menu dropdown-menu-end" aria-labelledby="navbarDropdown">
                                <li>
                                    <button 
                                        class="dropdown-item" 
                                        onclick={on_language_change.reform(|_| LanguageVariant::English)}
                                    >
                                        {language.nav.en}
                                    </button>
                                </li>
                                <li>
                                    <button 
                                        class="dropdown-item" 
                                        onclick={on_language_change.reform(|_| LanguageVariant::Spanish)}
                                    >
                                        {language.nav.es}
                                    </button>
                                </li>
                            </ul>
                        </li>
                    </ul>
                </div>
            </div>
        </nav>
    }
}
