#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;

mod m;
mod tableau_data;
mod tableau;

use tableau_data::TableauData;
use tableau::Tableau;
use rocket_contrib::json::Json;

#[post("/solve", format = "json", data = "<tableau>")]
fn solve(tableau: Json<TableauData>) -> Json<TableauData> {
    let mut t = Tableau::new(tableau.0);
    t.setup();
    t.solve();
    if t.has_artificial_vars && t.big_M_solve_algorithm.as_str() == "two-phase" {
        t.two_phase_second_phase();
    }
    if t.solved {
        t.get_solution();
        Json(TableauData::new(t))
    } else if t.error {
        Json(TableauData::new(Tableau::error_tableau(t)))
    } else {
        t.error = true; 
        t.error_message = String::from("Stuck in a cycle, terminted solution process.");
        Json(TableauData::new(t))
    }
}

fn main() {
    rocket::ignite().mount("/", routes![solve]).launch();
}
