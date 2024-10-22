//-------------------------------------------------------------------//
//  AUTHOR:    @sfmolina                                            //
//  Version:   v1                                                  //
//  Modified:  23oc24                                             //
//---------------------------------------------------------------//



use std::rc::Rc;
use yew::prelude::*;
use crate::translations::{
    Texts, ENGLISH_TEXTS, SPANISH_TEXTS
};



#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Context {
    pub language: &'static Texts,
}


impl Reducible for Context {
    type Action = &'static Texts;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        // Save the language preference to local storage
        if let Ok(Some(storage)) = web_sys::window().unwrap().local_storage() {
            let _ = storage.set_item("language", action.lang);
        }
        Context { language: action }.into()
    }
}


pub type GlobalContext = UseReducerHandle<Context>;


#[derive(Properties, Debug, PartialEq)]
pub struct GlobalContextProviderProps {
    #[prop_or_default]
    pub children: Html,
}


#[function_component]
pub fn GlobalContextProvider(props: &GlobalContextProviderProps) -> Html {
    let language = use_reducer(|| {
        // Try to get the language preference from local storage
        let default_language = if let Ok(Some(storage)) = web_sys::window().unwrap().local_storage() {
            if let Ok(Some(lang)) = storage.get_item("language") {
                match lang.as_str() {
                    "es" => &SPANISH_TEXTS,
                    "en" => &ENGLISH_TEXTS,
                    _ => &ENGLISH_TEXTS,
                }
            } else {
                &ENGLISH_TEXTS
            }
        } else {
            &ENGLISH_TEXTS
        };

        Context {
            language: default_language,
        }
    });

    html! {
        <ContextProvider<GlobalContext> context={language}>
            {props.children.clone()}
        </ContextProvider<GlobalContext>>
    }
}
