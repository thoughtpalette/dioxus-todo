use dioxus::prelude::*;

fn main() {
    dioxus::web::launch(app);
}

#[derive(Debug, PartialEq, Clone)]
struct Todo {
    pub key: u64,
    pub todo: String,
}

fn app(cx: Scope) -> Element {
    // let todos = use_state(&cx, || im_rc::HashMap::<u32, Todo>::default);
    let (todos, set_todos) = use_state(&cx, || vec![]);
    let input_value = use_state(&cx, || String::from(""));
    let mut _count = use_state(&cx, || 1);

    // .map(|item| 0rsx! {
    //     li {
    //         key: "{item}",
    //         div { "{item}" }
    //     }
    // })} 

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
                {todos.clone().iter().map(|item| rsx! {
                    li {
                        key: "{item}",
                        div { "{item}" }
                    }
                }
                )}
            }
            input {
                placeholder: "What are you doing today?",
                value: "{input_value}",
                oninput: move |evt| input_value.set(evt.value.clone()),
                onkeydown: move |evt| {
                    if evt.key == "Enter" && !input_value.is_empty() {
                        // todos.make_mut().insert(*count.current(), &input_value);
                        // input_value.set("".to_string());
                        println!("{}", input_value.get());
                        // count += 1;
                    }
                }
            }
            footer { class: "footer", 
                // button {
                //     onclick: move |_| todos.write().push(&input_value),
                //     "Add"
                // }
            }
        }
    )
}
