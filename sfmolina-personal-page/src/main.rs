//-------------------------------------------------------------------//
//  AUTHOR:    @sfmolina                                            //
//  Version:   v1                                                  //
//  Modified:  23oc24                                             //
//---------------------------------------------------------------//



mod app;
mod router;
mod components;
mod global_ctx;
mod translations;



use app::App;



fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
