use yew::prelude::*;
use crate::models::applicant::Applicant;
use crate::services::api;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub applicant: Applicant,
    pub on_update: Callback<Applicant>,
    pub is_mobile: bool,
}

#[function_component]
pub fn ApplicantCard(props: &Props) -> Html {
    let applicant = &props.applicant;
    let on_update = props.on_update.clone();
    
    let toggle_selection = {
        let mut applicant = applicant.clone();
        let on_update = on_update.clone();
        
        Callback::from(move |_| {
            applicant.is_selected = !applicant.is_selected;
            on_update.emit(applicant.clone());
        })
    };
    
    // Responsive classes
    let card_classes = if props.is_mobile {
        "p-3 mb-3"
    } else {
        "p-6 mb-4 hover:shadow-md"
    };
    
    let selected_classes = if applicant.is_selected {
        "bg-blue-50 dark:bg-blue-900 border-blue-200 dark:border-blue-700"
    } else {
        "bg-white dark:bg-gray-800 border-gray-200 dark:border-gray-700"
    };

    html! {
        <div class={classes!(
            "border rounded transition-all", 
            card_classes,
            selected_classes
        )}>
            <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
                { &applicant.name }
            </h3>
            <p class="text-gray-600 dark:text-gray-300">{ &applicant.email }</p>
            <p class="text-gray-600 dark:text-gray-300">{ &applicant.department }</p>
            <p class="text-gray-500 dark:text-gray-400 text-sm">{ &applicant.date }</p>
            <button 
                onclick={toggle_selection}
                class={classes!(
                    "mt-2 px-3 py-1 rounded text-sm",
                    if applicant.is_selected {
                        "bg-red-100 dark:bg-red-900 text-red-800 dark:text-red-200"
                    } else {
                        "bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200"
                    }
                )}
            >
                { if applicant.is_selected { "Deselect" } else { "Select" } }
            </button>
        </div>
    }
}