pub mod particle;
pub mod ue_groups;

pub use crate::particle::*;
pub use crate::ue_groups::*;

use std::{
    // io::{self, BufReader, prelude::*, BufWriter},
    io::{BufReader, BufWriter},
    // sync::{self, atomic::Ordering::Relaxed},
    fs::File
};

// use serde::{
//     Serialize,
//     Deserialize
// };

/*
use actix_web::{
    web::Buf,
    error,
    App, HttpResponse,
    HttpServer,
    Responder,
    HttpRequest,
    dev::Factory
};
*/

struct Course {
    name: String,
    ues: Vec<UEGroup>,
    id: usize,
}

// async fn get_particle(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}!", &name)
// }

// //async fn capture_mark(evt: web::Json<Particle>) -> Result<String> {
// async fn capture_mark(evt: web::Json<Particle>) -> impl Responder {
//     println!("{:?}", evt.average);
//     format!("{:?}", evt.average)
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .route("/particle", web::get().to(get_particle))
//             .route("/particle", web::post().to(capture_mark))
//     })
//     .bind(("127.0.0.1", 8080))?
//     .workers(2)
//     .run()
//     .await
// }

// fn main() {

//     let mut pysics = Particle::from_iter(vec![
//         Particle::new("Exam1".to_string(), 2.0, 15.0),
//         Particle::new("Exam2".to_string(), 1.0, 8.0),
//         Particle::new("Exam3".to_string(), 1.5, 13.0),
//     ]);
//     pysics.name = "Pysiques".to_string();
//     pysics.set_weight(7.0);

//     let mut maths = Particle::from_iter(vec![
//         Particle::new("Exam1".to_string(), 3.0, 12.0),
//         Particle::new("Exam2".to_string(), 1.0, 7.0),
//         Particle::new("Exam3".to_string(), 1.0, 8.0),
//         Particle::new("Exam4".to_string(), 2.0, 14.0),
//     ]);
//     maths.name = "Maths".to_string();
//     maths.set_weight(6.0);

//     let mut spanish = Particle::from_iter(vec![
//         Particle::new("Exam1".to_string(), 3.0, 12.0),
//         Particle::new("Exam2".to_string(), 1.0, 7.0),
//         Particle::new("Exam3".to_string(), 1.0, 8.0),
//         Particle::new("Exam4".to_string(), 2.0, 14.0),
//     ]);
//     spanish.name = "Spanish".to_string();
//     spanish.set_weight(3.0);

//     let mut ue1 = UEGroup {
//         name: "UE1".to_string(),
//         particle: Particle::from_iter(
//             vec![
//                 pysics,
//                 maths,
//                spanish,
//             ]),
//         passed: None,
//         requirement: 10.0,
//     };

//     ue1.determin_if_passed();

//     let f = File::create("data.json").unwrap();
//     let buffer = BufWriter::new(f);
//     serde_json::to_writer(buffer, &ue1).unwrap();
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = File::open("data.json")?;
    let buffer = BufReader::new(f);
    let ue1: UEGroup = serde_json::from_reader(buffer)?;
    println!("{:?}", ue1);
    Ok(())
}

#[cfg(test)]
pub mod test;
