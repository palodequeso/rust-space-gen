#[macro_use] extern crate rocket;

mod lib;

use rocket::serde::{json::Json};

use crate::lib::Star;
use crate::lib::Galaxy;
use crate::lib::GalaxyType;
use crate::lib::GalacticPosition;

#[get("/nearby_stars/<x>/<y>")]
fn nearby_stars(x: f64, y: f64) -> Json<Vec<Star>> {
    let galaxy = Galaxy::new(
        "Milky Way".to_string(),
        GalaxyType::Spiral,
        42,
    );

    let position = GalacticPosition::new(x, y);
    let result = galaxy.get_nearby_stars(position);
    Json(result)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![nearby_stars])
}
