// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]
#![allow(dead_code, unused_variables)]

use seed::{prelude::*, *};


// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model { 
        welcome_text: "Hello world".to_owned(),
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    welcome_text: String,
}


// ------ ------
//    Update
// ------ ------

// `Msg` describes the different events you can modify state with.
enum Msg {
    NoMsg,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::NoMsg => model.welcome_text = "".to_owned(),
    }
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Vec<Node<Msg>> {
    nodes![
        view_header(&model.welcome_text),
    ]
}

fn view_header(welcome_text: &str) -> Node<Msg>{
    header![C!["header"],
        h1![
            welcome_text
        ]
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
        .get_elements_by_class_name("root")
        .item(0)
        .expect("element with the class 'root'");

    App::start(root_element, init, update, view);
}
