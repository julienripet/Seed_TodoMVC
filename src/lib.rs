// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]
#![allow(dead_code, unused_variables)]

use seed::{prelude::*, *};

use std::collections::BTreeMap;
use std::mem;

use ulid::Ulid;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;

const ENTER_KEY: &str = "Enter";

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model { 
        todos: BTreeMap::new(),
        new_todo_title: String::new(),
        selected_todo: None,
        filter: Filter::All,
        base_url:Url::new(),
    }.add_mock_data()
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    todos: BTreeMap<Ulid, Todo>,
    new_todo_title: String,
    selected_todo: Option<SelectedTodo>,
    filter: Filter,
    base_url: Url 
}

impl Model {
    fn add_mock_data(mut self) -> Self {
        let (id_a, id_b) = (Ulid::new(), Ulid::new());

        self.todos.insert(id_a, Todo {
            id: id_a,
            title: "I'm a todo".to_owned(),
            completed:false
        });
        
        self.todos.insert(id_b, Todo {
            id: id_b,
            title: "I'm b todo".to_owned(),
            completed:true
        });

        self.new_todo_title= "I'm a new todo title".to_owned();

        self.selected_todo = Some(SelectedTodo {
            id:id_b,
            title: "I'm a better B todo".to_owned(),
            input_element: ElRef::new(),
        });
        self
    }
}

struct Todo {
    id: Ulid,
    title: String,
    completed: bool,
}

struct SelectedTodo {
    id: Ulid,
    title: String,
    input_element: ElRef<web_sys::HtmlInputElement>
}
#[derive(Copy, Clone, Eq, PartialEq, EnumIter)]
enum Filter {
    All, 
    Active, 
    Completed
}

// ------ ------
//    Update
// ------ ------

// `Msg` describes the different events you can modify state with.
enum Msg {
    UrlChanged(subs::UrlChanged),
    NewTodoTilteChanged(String),

    CreateTodo,
    ToggleTodo(Ulid),
    RemoveTodo(Ulid),
    
    CheckOrUncheckAll,
    ClearCompleted,
    
    SelectTodo(Option<Ulid>),
    SelectedTodoTitleChanged(String),
    SaveSelectedTodo,

}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        
        Msg::UrlChanged(subs::UrlChanged(url)) =>{
            log!("UrlChanged", url);
        }
        Msg::NewTodoTilteChanged(title) =>{
            log!("NewTodoTilteChanged", title);
            model.new_todo_title = title;
        }

        Msg::CreateTodo => {
            let title = model.new_todo_title.trim();
            if not(title.is_empty()) {
                let id =  Ulid::new();
                model.todos.insert(id, Todo{
                    id,
                    title: title.to_owned(),
                    completed:false
                });
                model.new_todo_title.clear();
            }
            log!("CreateTodo");
        }
        Msg::ToggleTodo(id) => {
            log!("ToggleTodo", id);
            if let Some(todo) = model.todos.get_mut(&id) {
                todo.completed = not(todo.completed)
            }
        }
        Msg::RemoveTodo(id) => {
            model.todos.remove(&id);
            log!("RemoveTodo", id);
        }
        
        Msg::CheckOrUncheckAll => {
            log!("CheckOrUncheckAll");
        }
        Msg::ClearCompleted => {
            log!("ClearCompleted");
        }
        
        Msg::SelectTodo(opt_id) => {
            log!("SelectTodo", opt_id);
        }
        Msg::SelectedTodoTitleChanged(title) =>{
            log!("SelectedTodoTitleChanged", title);
        }
        Msg::SaveSelectedTodo => {
            log!("SaveSelectedTodo");
        }

    }
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Vec<Node<Msg>> {
    // raw![include_str!("../template.html")];
    nodes![
        view_header(&model.new_todo_title),
        IF!(not(model.todos.is_empty()) => vec![
            view_main(&model.todos, model.selected_todo.as_ref()),
            view_footer(&model.todos, model.filter),
        ]),
    ]
}

fn view_header(new_todo_title: &str) -> Node<Msg>{
    header![C!["header"],
        h1!["todos"],
        input![C!["new-todo"],
            attrs!{
                At::Placeholder => "What needs doing?",
                At::AutoFocus=> AtValue::None,
                At::Value => new_todo_title
            },
            input_ev(Ev::Input, Msg::NewTodoTilteChanged),
            keyboard_ev(Ev::KeyDown, |keyboard_event|{
                IF!(keyboard_event.key() == ENTER_KEY => Msg::CreateTodo)
            })
        ]
    ]
}

fn view_main(todos: &BTreeMap<Ulid, Todo>, selected_todo: Option<&SelectedTodo>) -> Node<Msg>{
    section![C!["main"],
        view_toggle_all(todos),
        view_todo_list(todos, selected_todo),
    ]
}

fn view_toggle_all(todos: &BTreeMap<Ulid, Todo>) -> Vec<Node<Msg>>{
    let all_completed = todos.values().all(|todo| todo.completed);
    vec![
        input![C!["toggle-all"],
            attrs!{
                At::Id => "toggle-all", At::Type=> "checkbox", At::Checked => all_completed.as_at_value()
            }
        ],
        label![attrs!{At::For => "toggle-all"}, "Mark all as complete"],
    ]
}

fn view_todo_list(todos: &BTreeMap<Ulid, Todo>, selected_todo: Option<&SelectedTodo>) -> Node<Msg>{
    ul![C!["todo-list"],
        todos.values().map(|todo| {
            let id = todo.id;
            let is_selected = Some(todo.id) == selected_todo.map(|selected_todo| selected_todo.id);

            li![C![IF!(todo.completed => "completed"), IF!(is_selected => "editing")],
                div![C!["view"],
                    input![C!["toggle"],
                        attrs!{At::Type => "checkbox", At::Checked => todo.completed.as_at_value()},
                        ev(Ev::Change, move |_| Msg::ToggleTodo(id))
                    ],
                    label![&todo.title],
                    button![C!["destroy"],
                        ev(Ev::Click, move |_| Msg::RemoveTodo(id))
                    ],
                ],
                IF!(is_selected => {
                    let selected_todo = selected_todo.unwrap();
                    input![C!["edit"],
                        el_ref(&selected_todo.input_element),
                        attrs!{At::Value => selected_todo.title}
                    ]
                }),
            ]
        }) 
    ]
}

fn view_footer(todos: &BTreeMap<Ulid, Todo>, selected_filter: Filter) -> Node<Msg> {
    let completed_count = todos.values().filter(|todo| todo.completed).count();
    let active_count =  todos.len() - completed_count;

    footer![C!["footer"],
        // This should be `0 items left` by default
        span![C!["todo-count"],
            strong![active_count],
            format!(" item{} left", if active_count == 1 {""} else {"s"}),
        ],
        view_filters(selected_filter),
        // Hidden if no completed items are left ↓
        IF!(completed_count > 0 =>
            button![C!["clear-completed"],
                "Clear completed"
            ]
        )
    ]
}

fn view_filters(selected_filter: Filter) -> Node<Msg>{    
        ul![C!["filters"],
            Filter::iter().map(|filter| {
                let (link, title) = match filter {
                    Filter::All => ("#/", "All"),
                    Filter::Active => ("#/", "Active"),
                    Filter::Completed => ("#/", "Completed"),
                };
                li![
                    a![C![IF!(filter == selected_filter => "selected")],
                        attrs!{At::Href => link},
                        title,
                    ],
                ]
            })
        ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();

    let root_element = document()
        .get_elements_by_class_name("todoapp")
        .item(0)
        .expect("element with the class 'todoapp'");

    App::start(root_element, init, update, view);
}
