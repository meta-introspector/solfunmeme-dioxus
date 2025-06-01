use dioxus::prelude::*;
use wallet_adapter::Cluster;

use crate::{
    utils::{get_cluster_svg, trunk_cluster_name},
    AdapterCluster, BinSvg, CheckSvg, CloseSvg, ClusterName, ClustersSvg, LinkSvg,
    NotificationInfo, CLUSTER_STORAGE, GLOBAL_MESSAGE,
};

#[component]
pub fn Clusters() -> Element {
    let mut show_add_cluster_modal = use_signal(|| false);

    rsx! {
       div{class:"flex w-full flex-col justify-start p-10 items-center",
        div{class:"flex flex-col w-full items-center justify-center text-4xl",
            span{class:"flex w-[100px]", {ClustersSvg()}}, "Clusters"
            div {class:"text-xl",
                "Manage your Solana endpoints"
            }
            button {
                onclick:move|_|{
                    show_add_cluster_modal.set(true);
                },
                class: "bg-true-blue text-sm text-white px-5 py-2 mt-5 rounded-full hover:bg-cobalt-blue",
                "ADD CLUSTER"
            }
            div { class:"flex flex-wrap w-full items-stretch justify-center gap-4 mt-20",
                ClusterInfo{}
            }
        }
       }

       AddClusterModal{show_add_cluster_modal}
    }
}

#[component]
fn ClusterInfo() -> Element {
    let active = |adapter_cluster: &AdapterCluster| {
        adapter_cluster.name().as_bytes()
            == CLUSTER_STORAGE.read().active_cluster().name().as_bytes()
    };

    rsx! {
        for adapter_cluster in CLUSTER_STORAGE.read().get_clusters() {
            div {
                class:"flex flex-col text-xl p-5 w-[250px] bg-true-blue rounded-xl",
                div {class:"flex w-full",
                    span { class:"w-[25px] mr-2",
                        {get_cluster_svg(adapter_cluster.cluster())()}
                    }
                    {trunk_cluster_name(adapter_cluster.name())}
                }
                div { class: "flex flex-col w-full",
                    div { class: "flex w-full items-start flex-col mt-2.5 mb-5",
                        div { class: "bg-blue-100 text-blue-800 text-sm font-semibold px-2.5 py-0.5 rounded-full dark:bg-blue-200 dark:text-blue-800",
                            {adapter_cluster.cluster().chain()}
                        }
                        div {class:"text-sm mt-2",
                            {adapter_cluster.endpoint()}
                        }
                    }

                    div { class: "flex w-full items-center justify-between",
                        if !active(adapter_cluster) {
                            div { class: "text-3xl font-bold text-gray-900 dark:text-white",
                                {Switch(adapter_cluster.name())}
                            }
                            div {
                                class: " hover:bg-blue-800 rounded-xl dark:hover:bg-blue-700",
                                {Delete(adapter_cluster.clone())}
                            }
                        }else {
                            span { class: "w-5",
                                {CheckSvg()}
                            }
                        }
                    }
                }
            }
        }
    }
}

fn Switch(cluster_name: &str) -> Element {
    let cluster_name = cluster_name.to_string();

    rsx! {
        label {
            onclick:move|_|{
                let cluster_name = cluster_name.clone();

                let find_cluster = CLUSTER_STORAGE.read().get_cluster(&cluster_name).cloned();

                if let Some(active_cluster) = find_cluster{
                    CLUSTER_STORAGE.write().set_active_cluster(active_cluster);
                    GLOBAL_MESSAGE.write().push_back(NotificationInfo::new(String::new() + &cluster_name + " cluster now active!"));

                }else {
                    GLOBAL_MESSAGE.write().push_back(NotificationInfo::new(String::from("Could not find `") + &cluster_name + "` cluster!"));
                }

            },
            title:"Switch",
            class: "inline-flex items-center cursor-pointer",
            input { class: "sr-only peer", r#type: "checkbox", value: "" }
            div { class: "relative w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600 dark:peer-checked:bg-blue-600" }
        }
    }
}

fn Delete(cluster: AdapterCluster) -> Element {
    rsx! {
        div{
            onclick:move|_|{
                if CLUSTER_STORAGE.write().remove_cluster(cluster.name()).is_some(){
                   GLOBAL_MESSAGE.write().push_back(NotificationInfo::new(String::new() + cluster.name() + " cluster has been removed!"));
                }else {
                    GLOBAL_MESSAGE.write().push_back(NotificationInfo::new(String::from("Could not find `") + cluster.name() + "` cluster!"));
                }

            },
            title:"Delete", class:"cursor-pointer w-8", {BinSvg()}
        }
    }
}

#[component]
fn AddClusterModal(mut show_add_cluster_modal: Signal<bool>) -> Element {
    #[derive(Debug, Default)]
    struct AddCluster {
        name: String,
        endpoint: String,
        network: Cluster,
    }

    let mut add_cluster = use_signal(|| AddCluster::default());

    let should_show_button =
        !add_cluster.read().name.is_empty() && !add_cluster.read().endpoint.is_empty();

    if *show_add_cluster_modal.read() {
        rsx! {
            div {
                class: "fixed z-10 flex flex-col w-full h-full bg-[rgba(0,0,0,0.6)] justify-center items-center",
                div { class: "flex flex-col w-[90%] sm:w-[80%] md:w-[70%] min-h-64 max-h-[60%] lg:w-[90%] max-w-screen-sm justify-start items-center bg-gray-200 dark:bg-[#10141f] rounded-3xl",
                    div { class: "flex w-full justify-end items-center p-5",
                        button {
                            onclick: move |_| {
                                show_add_cluster_modal.set(false);
                            },
                            class: "wallet-adapter-modal-button-close w-[25px] items-center justify-center",
                            {CloseSvg()}
                        }
                    }
                    div { class: "flex w-4/5 rounded-xl min-h-[40vh] p-5 mb-10 items-start justify-center flex-col",
                        label {
                            class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                            r#for: "cluster-name",
                            "What would you like to call your cluster?"
                        }
                        div { class: "flex w-full mb-10",
                            span { class: "w-[40px] inline-flex items-center px-3 text-gray-900 bg-gray-200 border rounded-e-0 border-gray-300 border-e-0 rounded-s-md dark:bg-gray-600 dark:text-gray-400 dark:border-gray-600",
                                {ClusterName()}
                            }
                            input {
                                oninput: move |event| {
                                    let data = event.data.value();
                                    add_cluster.write().name = data;
                                },
                                class: "rounded-none rounded-e-lg bg-gray-50 border text-gray-900 focus:ring-blue-500 focus:border-blue-500 block flex-1 min-w-0 w-full text-sm border-gray-300 p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                                id: "cluster-name",
                                placeholder: "Rising Sun",
                                r#type: "text",
                                required: true,
                            }
                        }
                        label {
                            class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                            r#for: "endpoint",
                            "While URL & Custom port will you reach your cluster?"
                        }
                        div { class: "flex w-full",
                            span { class: "w-[40px] inline-flex items-center px-3 text-lg text-gray-900 bg-gray-200 border rounded-e-0 border-gray-300 border-e-0 rounded-s-md dark:bg-gray-600 dark:text-gray-400 dark:border-gray-600",
                                {LinkSvg()}
                            }
                            input {
                                oninput: move |event| {
                                    let data = event.data.value();
                                    if validate_url(&data) {
                                        add_cluster.write().endpoint = data;
                                    }
                                },
                                class: "rounded-none rounded-e-lg bg-gray-50 border text-gray-900 focus:ring-blue-500 focus:border-blue-500 block flex-1 min-w-0 w-full text-sm border-gray-300 p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                                id: "endpoint",
                                placeholder: "URL Endpoint ,eg. http://localhost:8899",
                                r#type: "url",
                                required: true,
                            }
                        }
                        label {
                            class: "block mb-2 text-sm mt-5 font-medium text-gray-900 dark:text-white",
                            r#for: "network",
                            "Network"
                        }
                        div { class: "flex w-full",
                            span { class: "w-[40px] inline-flex items-center px-3 bg-gray-200 border border-gray-300 rounded-s-md dark:bg-gray-600 dark:text-gray-400 dark:border-gray-600",
                                {ClustersSvg()}
                            }
                            select {
                                onchange: move |event| {
                                    let network: Cluster = event.data.value().as_str().try_into().expect(
                                        "This is a fatal error, you provided an invalid cluster"
                                    );
                                    add_cluster.write().network = network;
                                },
                                class: "rounded-none rounded-e-lg bg-gray-50 border text-gray-900 focus:ring-blue-500 focus:border-blue-500 block flex-1 min-w-0 w-full text-sm border-gray-300 p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                                id: "network",
                                name: "network",
                                required: true,
                                for cluster in CLUSTER_STORAGE.read().get_clusters() {
                                    option {
                                        key: "{cluster.name()}",
                                        value: cluster.identifier(),
                                        {cluster.name()}
                                    }
                                }
                            }
                        }
                        div { class: "flex w-full items-center justify-center p-5 mt-5",
                            if should_show_button {
                                button {
                                    onclick: move |_| {
                                        let adapter_cluster = AdapterCluster::new()
                                            .add_name(add_cluster.read().name.as_str())
                                            .add_endpoint(add_cluster.read().endpoint.clone().as_str())
                                            .add_cluster(add_cluster.read().network);

                                        let name = adapter_cluster.name().to_string();

                                        if let Err(error) = CLUSTER_STORAGE.write().add_cluster(adapter_cluster){
                                            show_add_cluster_modal.set(false);

                                            GLOBAL_MESSAGE.write().push_back(NotificationInfo::new(format!("Error Adding Cluster: `{error}`!")));

                                        }else {
                                            GLOBAL_MESSAGE.write().push_back(NotificationInfo::new(format!("Added `{name}` cluster!")));
                                            show_add_cluster_modal.set(false);
                                        }

                                        add_cluster.set(AddCluster::default());

                                    },
                                    class: "bg-true-blue text-sm text-white px-5 py-2 rounded-full hover:bg-cobalt-blue",
                                    "ADD CLUSTER"
                                }
                            } else {}
                        }
                    }
                }
            }
        }
    } else {
        rsx! {}
    }
}

fn validate_url(value: &str) -> bool {
    let scheme_exists = value.starts_with("http://") || value.starts_with("https://");

    scheme_exists && value.len() > 8
}
