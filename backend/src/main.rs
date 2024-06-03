use std::sync::Mutex;

#[macro_use] extern crate rocket;

mod utils;
mod cors;

mod soterius;

mod database;
mod login_info;
mod user;
mod team;

mod project;
mod group;
mod task;


#[get("/")]
pub fn index() -> String {
    "can you understand me?".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::custom(rocket::config::Config::figment().merge(("port", 8002)))
        .manage(Mutex::new(database::Database::load()))
        .mount("/", routes![index])
        .mount("/save", routes![database::save])
        .mount("/load", routes![database::load])
        .mount("/debug", routes![database::debug])

        .mount("/project/fetch", routes![project::fetch])
        .mount("/project/create", routes![project::create])
        .mount("/project/delete", routes![project::delete])
        .mount("/project/edit", routes![project::edit])

        .mount("/group/create", routes![group::create])
        .mount("/group/delete", routes![group::delete])
        .mount("/group/edit", routes![group::edit])

        .mount("/task/create", routes![task::create])
        .mount("/task/delete", routes![task::delete])
        .mount("/task/edit", routes![task::edit])
        .mount("/task/assign", routes![task::assign])
        .mount("/task/toggle_assign", routes![task::toggle_assign])
        .mount("/task/complete", routes![task::complete])
        .mount("/task/toggle_complete", routes![task::toggle_complete])

        .attach(cors::CORS)
}