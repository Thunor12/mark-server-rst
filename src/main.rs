use std::io;
use std::io::BufReader;
use std::sync;
use std::sync::atomic::Ordering::Relaxed;

use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;

use actix_web::dev::Factory;
use actix_web::web::Buf;
use actix_web::{error, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde;
use serde::__private::de::IdentifierDeserializer;
use serde_json;
use serde::Serialize;
use serde::Deserialize;
use atomic_float::{self, AtomicF64};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Particle {
    name: String,
    weight: Option<f64>,
    average: Option<f64>,
    particles: Vec<Particle>,
}

impl Default for Particle {
    fn default() -> Self {
        Particle {
            name: "".to_string(),
            weight: Some(1 as f64),
            average: None,
            particles: Vec::new(),
        }
    }
}

impl FromIterator<Particle> for Particle {
    fn from_iter<I: IntoIterator<Item=Particle>>(iter: I) -> Self {
        let mut u = Self::default();

        u.particles.extend(iter);
        u.update();
        u
    }
}

impl Particle {
    fn set_weight(&mut self, v: f64) {
        self.weight = Some(v);
    }

    fn get_weight(& self) -> f64 {
        match self.weight {
            Some(w) => w,
            None => 1 as f64,
        }
    }

    fn get_average(& self) -> Result<f64, Box<dyn std::error::Error>> {
        let e: Box<dyn std::error::Error> = From::from("No average available".to_string());
        self.average.ok_or(e)
    }

    fn new(n: String, weight: f64, average: f64) -> Self {
        Particle {
            name: n,
            average: Some(average),
            weight: Some(weight),
            particles: vec![],
        }
    }

    // TODO add error handeling
    fn update(&mut self) {
        if self.particles.len() == 0 {
            return;
        }

        let mut numerator = AtomicF64::new(0.0);
        let mut summed_weights = AtomicF64::new(0.0);

        self.particles
            .as_mut_slice()
            .into_par_iter()
            .for_each(|p| {p.update();});

        self.particles
            .as_mut_slice()
            .into_par_iter()
            .filter(|p| {p.average.is_some()})
            .for_each(|particle| {
                let w = particle.get_weight();
                numerator.fetch_add(particle.get_average().unwrap() * w, Relaxed);
                summed_weights.fetch_add(w, Relaxed);
            });

        self.average = Some(*numerator.get_mut() / *summed_weights.get_mut());
    }

}


#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExameGroup {
    name: String,
    particle: Particle,
}

impl Default for ExameGroup {
    fn default() -> Self {
        ExameGroup {
            name: "".to_string(),
            particle: Particle::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UEGroup {
    name: String,
    particle: Particle,
    passed: Option<bool>,
    requirement: f64,
}

impl Default for UEGroup {
    fn default() -> Self {
        UEGroup {
            name: "".to_string(),
            particle: Particle::default(),
            passed: None,
            requirement: 10 as f64,
        }
    }
}

impl UEGroup {
    fn determin_if_passed(&mut self) -> Option<bool> {
        self.particle.update();
        match self.particle.average {
            Some(a) => {
                self.passed = Some(a >= self.requirement);
                return self.passed;
            },
            None => None,
        }

    }
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
mod tests {
    use std::vec;
    use crate::Particle;
    use crate::UEGroup;

    #[test]
    #[should_panic]
    fn test_panick_if_average_is_none() {
        let p = Particle::default();
        let a = p.get_average().unwrap();
    }

    #[test]
    fn test_can_access_average_if_some() {
        let p = Particle::new("".to_string(), 1.0, 1.0);
        assert_eq!(p.get_average().unwrap(), 1.0)
    }

    #[test]
    fn test_average_update_on_one_particle() {
        let p = Particle::from_iter(
            vec![
                Particle::new("".to_string(), 1.0, 1.0),
                Particle::new("".to_string(), 2.0, 4.0),
            ]
        );

        assert_eq!(p.average.unwrap(), 3.0);
    }

    #[test]
    fn test_average_update_cascade_particle() {
        let mut p1 = Particle::from_iter(
            vec![
                Particle::new("".to_string(), 1.0, 1.0),
                Particle::new("".to_string(), 2.0, 1.0),
            ]
        );
        p1.weight = Some(1.0);

        let mut p2 = Particle::new("".to_string(), 2.0, 4.0);

        let p = Particle::from_iter(vec![p1, p2]);

        assert_eq!(p.average.unwrap(), 3.0);
    }

    #[test]
    fn test_uegroup_passes_with_requirement() {
        let mut ue1 = UEGroup {
            name: "Sciences".to_string(),
            particle: Particle::from_iter(
                vec![
                    Particle::new("".to_string(), 1.0, 10.0),
                    Particle::new("".to_string(), 2.0, 10.5),
                ]),
            passed: None,
            requirement: 10.0,
        };

        assert_eq!(ue1.determin_if_passed().unwrap(), true);

    }

    #[test]
    #[should_panic]
    fn test_uegroup_panic_if_none() {
        let mut ue1 = UEGroup {
            name: "Sciences".to_string(),
            particle: Particle::from_iter(vec![]),
            passed: None,
            requirement: 10.0,
        };

        ue1.determin_if_passed().unwrap();

    }

}
