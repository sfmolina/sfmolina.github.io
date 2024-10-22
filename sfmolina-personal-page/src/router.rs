//-------------------------------------------------------------------//
//  AUTHOR:    @sfmolina                                            //
//  Version:   v1                                                  //
//  Modified:  23oc24                                             //
//---------------------------------------------------------------//



use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::{
    echart_demo::EchartDemo,
    about_me::AboutMe,
    projects::Projects,
    super_tres::SuperTresComponent,
};



#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Root,
    #[at("/echart-demo")]
    EchartDemo,
    #[at("/about-me")]
    AboutMe,
    #[at("/projects")]
    Projects,
    #[at("/super-tres")]
    SuperTres,
    
}


pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Root => html! { <Redirect<Route> to={Route::AboutMe}/> },
        Route::EchartDemo => html! { <EchartDemo /> },
        Route::AboutMe => html! { <AboutMe /> },
        Route::Projects => html! { <Projects /> },
        Route::SuperTres => html! { <SuperTresComponent /> },
    }
}
