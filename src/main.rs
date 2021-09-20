/bin/bash: ligne 1: q : commande introuvable
use std::fs::File;
use std::io::Bytes;
use std::io::prelude::*;
use std::io::BufWriter;
use std::io::BufReader;

use serde::ser::Error;
use serde_json;
use serde;
use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Exame {
    name: String,
    mark: f64,
    weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Subject {
    name: String,
    exames: Vec<Exame>,
    weight: f64,
    average: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UE {
    name: String,
    subjects: Vec<Subject>,
    average: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Marks {
    marks: Vec<UE>,
    average: f64,
}

impl Subject {

    fn add_exame(& mut self, exame: Exame) {
        self.exames.push(exame); 
    }

    fn compute_average(& mut self) {
        if self.exames.len() == 0 {return;}

        self.average = 0 as f64;
        let mut summed_weights: f64 = 0 as f64;

        for exame in self.exames.as_slice() {
            summed_weights += exame.weight;
            self.average += exame.mark * exame.weight;
        }

        self.average = self.average / summed_weights;
    }
}

impl UE {
    fn add_subject(& mut self, subject: Subject) {
        self.subjects.push(subject);
    }

    fn compute_average(& mut self) {
        if self.subjects.len() == 0 {return;}

        self.average = 0 as f64;
        let mut summed_weights: f64 = 0 as f64;

        for subject in self.subjects.as_mut_slice() {
            subject.compute_average();
            summed_weights += subject.weight;
            self.average += subject.average * subject.weight;
        }

        self.average = self.average / summed_weights;
    }
}

impl Marks {
    fn add_ue(& mut self, ue: UE) {
        self.marks.push(ue);
    }

    fn compute_average(& mut self) {
       if self.marks.len() == 0 {return;}
       
       self.average = 0 as f64;
       let mut summed_weights: f64 = 0 as f64;

       for mark in self.marks.as_mut_slice() {
           mark.compute_average();
           summed_weights += 1 as f64;
           self.average += mark.average;
       }

       self.average = self.average / summed_weights;

    }
}

fn load_db(path: &str) -> Result<Marks, serde_json::Error> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => panic!("{}", e),
    };

    let marks: Marks = match serde_json::from_reader(file) {
        Ok(mks) => mks,
        Err(e) => return Err(e),
    };

    Ok(marks)
}

fn save_data(marks: Marks, path: &str) -> Result<(), serde_json::Error> {
    let mut file = match File::create(path) {
       Ok(f) => f,
       Err(e) => panic!("{}", e),
    };

   let data = match serde_json::to_string(&marks) {
       Ok(d) => d,
       Err(e) => return Err(e),
   };

   match file.write_all(data.as_bytes()) {
       Ok(())=> (),
       Err(e) => panic!("{}", e),
   };

   Ok(())
}

fn main() { 
    let path: &str = "data.json";
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => panic!("{}", e),
    };
}
