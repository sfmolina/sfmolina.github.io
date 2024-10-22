//-------------------------------------------------------------------//
//  AUTHOR:    @sfmolina                                            //
//  Version:   v1                                                  //
//  Modified:  23oc24                                             //
//---------------------------------------------------------------//



use yew::prelude::*;
use yew_router::prelude::*;
use crate::{
    global_ctx::GlobalContextProvider,
    router::{switch, Route},
    components::nav_bar::Navbar,
};



#[function_component(App)]
pub fn app() -> Html {
    html! {
        <HashRouter>
            <GlobalContextProvider>
                <Navbar />
                <main>
                    <Switch<Route> render={switch} />
                </main>
            </GlobalContextProvider>
        </HashRouter>
    }
}
