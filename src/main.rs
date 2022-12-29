use dioxus::{prelude::*, events::{oninput, onkeydown}};

fn main() {
    dioxus::web::launch(app);
}


fn app(cx: Scope) -> Element {
    let initial_todo = vec!["This is the first Todo"];
    let todos = use_state(&cx, || initial_todo);
    let draft = use_state(&cx, || "".to_string());
    let mut count = use_state(&cx, || 1);

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
                {todos.iter().map(|item| rsx! {
                    li {
                        key: "{item}",
                        div { "{item}" }
                    }
                })} 
            }
            input {
                placeholder: "What are you doing today?",
                value: "{draft}",
                oninput: move |evt| draft.set(evt.value.clone()),
                onkeydown: move |evt| {
                    if evt.key == "Enter" && !draft.is_empty() {
                        todos.make_mut().insert(*count.current(), &draft);
                        draft.set("".to_string());
                        count += 1;
                    }
                }
            }
            footer { class: "footer", 
                button {
                    onclick: move |_| todos.with_mut(|v| v.push("yeet")),
                    "Add"
                }
            }
        }
    )
}
