//-------------------------------------------------------------------//
//  AUTHOR:    @sfmolina                                            //
//  Version:   v1                                                  //
//  Modified:  19de24                                             //
//---------------------------------------------------------------//



use yew::prelude::*;
use yew_router::prelude::*;
use crate::{
    global_ctx::GlobalContext, router::Route, translations::{
        LanguageVariant, ENGLISH_TEXTS, SPANISH_TEXTS
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

    html! {
        <div class="nav-style">
        <nav class="navbar navbar-expand-lg bg-body-tertiary fixed-top">
            <div class="container-fluid">
                // Navbar brand
                <Link<Route> classes={classes!("navbar-brand")} to={Route::Root}>{language.author_username}</Link<Route>>

                // Navbar toggler (for small screens)
                <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent" 
                        aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                </button>

                // Navbar itself
                <div class="collapse navbar-collapse" id="navbarSupportedContent">
                    // Navbar left side
                    <LefttNav />

                    // Navbar right side
                    <RightNav on_language_change={on_language_change} />
                </div>
            </div>
        </nav>
        </div>
    }
}


#[function_component(LefttNav)]
fn left_nav() -> Html {

    let global_ctx = use_context::<GlobalContext>().expect("GlobalContext not found");
    let language = global_ctx.language;
    let current_route: Option<Route> = use_route();

    html! {
        <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                        
            // Link to AboutMe
            <li class="nav-item">
                <Link<Route> classes={classes!(
                    "nav-link",
                    (current_route == Some(Route::AboutMe)).then_some("active")
                )} to={Route::AboutMe}>{language.nav.about_me}</Link<Route>>
            </li>
            
            // Link to Projects
            <li class="nav-item">
                <Link<Route> classes={classes!(
                    "nav-link",
                    (current_route == Some(Route::Projects)).then_some("active")
                )} to={Route::Projects}>{language.nav.projects}</Link<Route>>
            </li>
            
            // Links to other projects
            <li class="nav-item dropdown">
                <a class="nav-link dropdown-toggle" href="#" role="button" data-bs-toggle="dropdown" aria-expanded="false">
                    {language.nav.misc}
                </a>
                <ul class="dropdown-menu">
                    <li><a class="dropdown-item" href="https://sfmolina.github.io/supertres/">{language.nav.super_tres}</a></li>
                    
                    <li><hr class="dropdown-divider"/></li>
                    
                    <li><a class="dropdown-item" href="https://sfmolina.github.io/mod-comp/">{language.nav.mod_comp}</a></li>
                </ul>
            </li>
    
        </ul>
    }

}


#[derive(Properties, PartialEq)]
struct RightNavProps {
    on_language_change: Callback<LanguageVariant>,
}
#[function_component(RightNav)]
fn right_nav(RightNavProps { on_language_change }: &RightNavProps) -> Html {

    let global_ctx = use_context::<GlobalContext>().expect("GlobalContext not found");
    let language = global_ctx.language;

    html! {
        <ul class="navbar-nav">
            <li class="nav-item dropdown">
                <a class="nav-link dropdown-toggle bold" href="#" role="button" data-bs-toggle="dropdown" aria-expanded="false">
                    {language.nav.language}
                </a>
                <ul class="dropdown-menu dropdown-menu-end">
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
                    //<li><hr class="dropdown-divider"/></li>
                    //<li><a class="dropdown-item" href="#">{"Something else here"}</a></li>
                </ul>
            </li>
        </ul>
    }

}
