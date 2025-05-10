use yew::prelude::*;
use crate::services::api;
use crate::components::ApplicantCard;
use crate::auth::context::AuthContextHandle;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub auth: AuthContextHandle,
}

#[function_component]
pub fn ApplicantList(props: &Props) -> Html {
    let applicants = use_state(|| Vec::new());
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let is_mobile = use_mobile_view();
    
    // Fetch data on mount
    {
        let applicants = applicants.clone();
        let loading = loading.clone();
        let error = error.clone();
        let auth = props.auth.clone();
        
        use_effect_with_deps(move |_| {
            if !auth.is_authenticated() {
                return || ();
            }
            
            wasm_bindgen_futures::spawn_local(async move {
                match api::fetch_applicants(auth.token().unwrap()).await {
                    Ok(data) => {
                        applicants.set(data);
                        loading.set(false);
                    }
                    Err(e) => {
                        error.set(Some(e));
                        loading.set(false);
                    }
                }
            });
            || ()
        }, ());
    }
    
    let on_update = {
        let applicants = applicants.clone();
        let auth = props.auth.clone();
        
        Callback::from(move |updated: Applicant| {
            let applicants = applicants.clone();
            let auth = auth.clone();
            
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(updated) = api::update_applicant(&updated, auth.token().unwrap()).await {
                    let mut current = (*applicants).clone();
                    if let Some(index) = current.iter().position(|a| a.id == updated.id) {
                        current[index] = updated;
                        applicants.set(current);
                    }
                }
            });
        })
    };
    
    // Responsive grid
    let grid_classes = if is_mobile {
        "grid-cols-1 gap-3"
    } else {
        "grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4"
    };

    html! {
        <div class={classes!("grid", grid_classes)}>
            {if *loading {
                html! { 
                    <div class="col-span-full flex justify-center items-center h-64">
                        <div class="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-primary"></div>
                    </div>
                }
            } else if let Some(err) = &*error {
                html! { 
                    <div class="col-span-full bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded">
                        <strong class="font-bold">{ "Error: " }</strong>
                        <span class="block sm:inline">{ err }</span>
                    </div>
                }
            } else {
                html! {
                    <>
                        {for applicants.iter().map(|applicant| {
                            html! {
                                <ApplicantCard 
                                    applicant={applicant.clone()} 
                                    on_update={on_update.clone()}
                                    is_mobile={is_mobile}
                                />
                            }
                        })}
                    </>
                }
            }}
        </div>
    }
}

fn use_mobile_view() -> bool {
    let is_mobile = use_state(|| false);
    
    use_effect_with_deps(move |_| {
        let window = web_sys::window().unwrap();
        let check_mobile = move || {
            let width = window.inner_width().unwrap().as_f64().unwrap();
            is_mobile.set(width < 768.0);
        };
        
        check_mobile();
        
        let closure = Closure::wrap(Box::new(check_mobile) as Box<dyn FnMut()>);
        window.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
        
        || ()
    }, ());
    
    *is_mobile
}