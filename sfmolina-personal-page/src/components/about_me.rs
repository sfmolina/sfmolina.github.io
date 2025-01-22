//-------------------------------------------------------------------//
//  AUTHOR:    @sfmolina                                            //
//  Version:   v1                                                  //
//  Modified:  23oc24                                             //
//---------------------------------------------------------------//



use yew::prelude::*;
use crate::{
    global_ctx::GlobalContext, 
    translations::Texts
};



#[function_component(AboutMe)]
pub fn about_me() -> Html {
    
    let global_ctx = use_context::<GlobalContext>().expect("GlobalContext not found");
    let language = global_ctx.language;

    html! {
        <div class="about-me">
            <div class="container-fluid h-100">
                <div class="row top-section">
                    <TextColumn language={language} />
                    <PhotoColumn language={language} />
                </div>
            </div>
        </div>
    }
}


#[derive(Properties, PartialEq)]
struct LanguageProps {
    language: &'static Texts,
}


#[function_component(TextColumn)]
fn text_column(props: &LanguageProps) -> Html {
    html! {
        <div class="col-lg-6 d-flex align-items-center justify-content-center text-column">
            <div class="presentation">
                <h1>{ props.language.about_me.greeting }</h1>
                <h2>
                    { props.language.about_me.my_name }
                    <span class="highlighted">{ props.language.author_large }</span>
                </h2>
                <h2>{ props.language.about_me.my_profession }</h2>
                <p>{ props.language.about_me.introduction1 }</p>
                <p>{ props.language.about_me.introduction2 }</p>
                { social_buttons() }
            </div>
        </div>
    }
}


#[function_component(PhotoColumn)]
fn photo_column(props: &LanguageProps) -> Html {
    html! {
        <div class="col-lg-6 d-flex align-items-center justify-content-center photo-column">
            <div class="profile-picture">
                <div class="sphere-container">
                    // Profile picture from public/
                    <img src="public/profile_picture.jpg" alt={ props.language.about_me.prof_pict_alt }/>
                </div>
                <div class="sphere sphere1"></div>
                <div class="sphere sphere2"></div>
                <div class="sphere sphere3"></div>
                <div class="sphere sphere4"></div>
            </div>
        </div>
    }
}


/// Pure component: social_buttons
fn social_buttons() -> Html {
    html! {
        <div class="social-buttons">
            <button class="lnk-btn">
                <a href="https://linkedin.com/in/sfmolina" target="_blank" class="social-url">
                    {"LinkedIn"}
                </a>
            </button>
            <button class="gh-btn">
                <a href="https://github.com/sfmolina" target="_blank" class="social-url">
                    {"GitHub"}
                </a>
            </button>
        </div>
    }
}
