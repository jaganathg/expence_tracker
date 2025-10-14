use dioxus::prelude::*;
use dioxus_router::prelude::*;

mod components;
mod pages;
mod models;
mod services;

use pages::{Home, AddExpense, ExpenseList};

#[derive(Routable, Clone)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/add")]
    AddExpense {},
    #[route("/list")]
    ExpenseList {},
}

#[allow(non_snake_case)]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

fn main() {
    dioxus::launch(App);
}

 