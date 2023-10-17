use super::use_tabs;
use crate::utils::mount_style::mount_style;
use leptos::*;

#[derive(Clone)]
pub(crate) struct TabOption {
    pub key: String,
    pub label: String,
}

#[component]
pub fn Tab(
    #[prop(into)] key: String,
    #[prop(into)] label: String,
    children: Children,
) -> impl IntoView {
    mount_style("tab", include_str!("./tab.css"));
    let tabs = use_tabs();
    tabs.push_tab_options(TabOption {
        key: key.clone(),
        label,
    });
    view! {
        <div class="melt-tab" class=("melt-tab--hidden", move || key != tabs.get_key())>
            {children()}
        </div>
    }
}