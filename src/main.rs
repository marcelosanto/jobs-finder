#![allow(non_snake_case)]

use api::Label;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

mod api;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    let jobs = use_resource(move || async move { api::get_jobs().await });

    rsx! {match &*jobs.read() {
        Some(Ok(resp)) => {

                rsx! {
                    Link { to: Route::Home {}, "Go to counter" },
                p {"Blog post {&resp:?}"}
            }

            //println!("{:?}", resp.len())
        },
        Some(Err(e)) => {

                rsx! {Link { to: Route::Home {}, "Go to counter" }
                p {"Blog post {e}"}}

        }
        None => {
            rsx! {
                p {"Loading() "}
            }
        },
    }}
}

#[component]
fn Home() -> Element {
    let jobs = use_resource(move || async move { api::get_jobs().await });

    rsx! {
           Link {
                to: Route::Blog {
                    id: 22
                },
                "Go to blog"
            }
            style {{include_str!("../assets/tailwind.css")}}
           body { class: "bg-gray-50 p-8 mt-10",
            div { class:"max-w-md mx-auto",
            //<!-- Header -->
            div { class:"bg-purple-100 p-6 rounded-lg text-center mb-8",
                h1 { class:"text-xl font-semibold", "Discover your ideal career right here!"}
                p {class:"text-sm text-gray-600 mt-2","Explore opportunities that suit your interests to achieve the career you want."}
                div {class:"mt-4",
                    input {r#type:"text", placeholder:"Search Jobs...", class:"w-full p-3 border border-gray-300 rounded-lg shadow-sm focus:outline-none focus:ring-2 focus:ring-purple-500"}
           }
           }

           // <!-- Job Listings -->
            div {class:"grid grid-cols-1 sm:grid-cols-2 gap-6 sm:grid-cols-3",
              //  <!-- Job Card -->



                match &*jobs.read() {
                    Some(Ok(resp)) => {

                            rsx! {
                              for i in 0..resp.len(){
                              Card {
                                title: "{resp[i].title}",
                                role: "UI/UX Designer",
                                rate: "$7000/hr",
                                location: "Jakarta, IND",
                                experience: "5 years",
                                skills: "Figma, Sketch, Adobe XD",
                                description: "Looking for a creative UI/UX Designer to join our team.",
                                site: "{resp[i].html_url}",
                                labels: resp[i].labels.clone()
                            }}
                        }

                        //println!("{:?}", resp.len())
                    },
                    Some(Err(e)) => {

                            rsx! {Link { to: Route::Home {}, "Go to counter" }
                            p {"Blog post {e}"}}

                    }
                    None => {
                        rsx! {
                            p {"Loading() "}
                        }
                    },
                }



                //<!-- Repita para outros cartões de empregos -->
               // <!-- Exemplo de outro cartão -->



               // <!-- Adicione mais cartões conforme necessário -->



            }
        }
    }}
}

#[component]
fn Card(
    title: String,
    role: String,
    rate: String,
    location: String,
    experience: String,
    skills: String,
    description: String,
    site: String,
    labels: Vec<Label>
) -> Element {
    let mut is_expanded = use_signal(|| false);

    rsx! {
            div {
                class: "bg-white p-6 rounded-lg shadow hover:shadow-md transition",
            div {class:"flex items-center justify-between",
                div {class:"flex items-center",
                    img {src:"https://w7.pngwing.com/pngs/786/126/png-transparent-logo-contracting-photography-logo-symbol.png", alt:"Vista Logo", class:"w-10 h-10 mr-3"}
                    div {
                        h2 { class: "text-xl font-bold", "{title}" }
                        for i in 0..labels.len() {
                            span { class:"bg-blue-100 text-blue-800 text-xs font-medium me-2 px-2.5 py-0.5 rounded dark:bg-blue-900 dark:text-blue-300","{labels[i].name}"}

                        }
                    }
        }
                a {r#type:"link",class:"text-blue-500 text-xs font-semibold",href:"{site}","Onsite"
            }
    }
            div {class:"mt-4",
                span {class:"text-green-500 font-semibold","$7000/hr"}
                span {class:"text-gray-500 text-sm ml-2","{location}"}
    }
            div {class:"mt-4",
                button {class:"bg-green-100 text-green-600 px-4 py-2 rounded-lg text-sm",onclick: move |_| is_expanded.set(!is_expanded()),
                "Details"}
            }
            if is_expanded() {

                div {
                    class: "mt-4",
                    p { b { "Experience: " } "{experience}" }
                    p { b { "Skills: " } "{skills}" }
                    p { b { "Description: " } "{description}" }
                }

        }
    }
        }
}
