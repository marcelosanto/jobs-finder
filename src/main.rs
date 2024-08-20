#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

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
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        div {
            class: "container mx-auto mt-10 flex justify-center",

        }
            // Link {
            //     to: Route::Blog {
            //         id: count()
            //     },
            //     "Go to blog"
            // }
            style {{include_str!("../assets/tailwind.css")}}
           body { class: "bg-gray-50 p-8 mt-10",
            div { class:"max-w-4xl mx-auto",
            //<!-- Header -->
            div { class:"bg-purple-100 p-6 rounded-lg text-center mb-8",
                h1 { class:"text-xl font-semibold", "Discover your ideal career right here!"}
                p {class:"text-sm text-gray-600 mt-2","Explore opportunities that suit your interests to achieve the career you want."}
                div {class:"mt-4",
                    input {r#type:"text", placeholder:"Search Jobs...", class:"w-full p-3 border border-gray-300 rounded-lg shadow-sm focus:outline-none focus:ring-2 focus:ring-purple-500"}
           }
           }

           // <!-- Job Listings -->
            div {class:"grid grid-cols-1 sm:grid-cols-2 gap-6",
              //  <!-- Job Card -->

              for i in 0..10 {
                Card {
                    title: "Vista {i}",
                    role: "UI/UX Designer",
                    rate: "$7000/hr",
                    location: "Jakarta, IND",
                    experience: "5 years",
                    skills: "Figma, Sketch, Adobe XD",
                    description: "Looking for a creative UI/UX Designer to join our team."
                }
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
                    p { "{role}" }
                    }
        }
                span {class:"text-blue-500 text-xs font-semibold","Onsite"}
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
