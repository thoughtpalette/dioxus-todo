use dioxus::prelude::*;

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    let mut list_items = vec!["Todo1", "Todo2", "Todo3"];

    rsx!(cx, main {
            style { 
                    [
                        include_str!("../src/reset.css"),
                        include_str!("../src/style.css")
                    ]}
            header { class: "header", 
                h1 { "Todays Todos" }
            }
            ul { class: "todo-list",

                {list_items.iter().map(|item| rsx! {
                    li {
                        key: "{item}",
                        div { "{item}" }
                    }
                })} 
            }
            footer { class: "footer"}
        }
    )
}
