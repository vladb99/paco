use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use opencv::{core};
use rayon::prelude::*;
use detection::*;
use video::*;
use clap::{Arg, App};
use std::thread;

pub mod detection;
pub mod video;

//////////////////////////////////////////////////////////////////////
// FileName:            main.rs
// FileType:            Rust - Source file
// Author:              Vlad Bratulescu, Kevin Castorina, Marco Mollo
// Task:                Aufgabe 2
// Created On:          07.11.2021
// Last Modified On :   11.11.2021 23.50
//////////////////////////////////////////////////////////////////////


#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref ARGS: (String, f64) = init_args();
}

#[derive(Debug, Copy, Clone)]
pub struct Object {
    pub frame: f64,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Object {
    pub fn new(frame: f64, x: i32, y: i32, width: i32, height: i32) -> Object {
        Object {
            frame,
            x,
            y,
            width,
            height,
        }
    }

    pub fn cmp(&self, object: &Object) -> bool {
        return self.frame == object.frame &&
            self.x == object.x &&
            self.y == object.y &&
            self.width == object.width &&
            self.height == object.height;
    }
}

fn init_args() -> (String, f64) {
    let matches = App::new("Car Tracking Program")
        .arg(Arg::new("number")
            .short('n')
            .takes_value(true))
        .arg(Arg::new("path")
            .short('f')
            .takes_value(true)
            .required(true))
        .get_matches();
    (matches.value_of("path").unwrap().to_string(), matches.value_of("number").unwrap_or("2690").parse().unwrap())
}

fn main() {
    // Get Args
    let file_name = &ARGS.0;
    let number_of_frames = ARGS.1;

    // Open files
    let mut car_classifier = CascadeClassifier::new("cars.xml");

    let skipping = 10;
    let mut vid = Video::new(file_name);
    let mut vid_container = Vec::new();
    for fidx in (0..vid.frame_count as i32).step_by(skipping) {
        vid_container.push((fidx, vid.get_grayframe(fidx as f64).unwrap()));
    }

    let mut cars = HashMap::new();
    for (fidx, gray) in vid_container {
        let c = car_classifier.detect_on_frame(&gray);
        cars.insert(fidx, c);
    }

    println!("frames in map {}", cars.len());
}

fn count_objects(detected_objects: Vec<Object>) -> (i32, i32, i32, i32, i32) {
    let mut objects_count_1 = 0;
    let mut objects_count_2 = 0;
    let mut objects_count_3 = 0;
    let mut objects_count_4 = 0;
    let mut objects_count_5 = 0;

    let threshold_pixel_y = 90;
    let threshold_frames: f64 = 10f64;
    let mut taken_objects: Vec<f64> = Vec::new();
    for (i, object) in detected_objects.iter().enumerate() {
        if taken_objects.contains(&(i as f64)) {
            continue;
        }
        taken_objects.push(i as f64);

        let lane = get_lane(object.x, object.y);
        if lane == 0 {
            objects_count_1 += 1;
        } else if lane == 1 {
            objects_count_2 += 1;
        } else if lane == 2 {
            objects_count_3 += 1;
        } else if lane == 3 {
            objects_count_4 += 1;
        } else {
            objects_count_5 += 1;
        }
        let mut reference_object = Object::new(object.frame, object.x, object.y, object.width, object.height);
        for (j, possible_neighbor) in detected_objects.iter().enumerate() {
            if possible_neighbor.cmp(&object) {
                continue;
            }
            let lane_neighbor = get_lane(possible_neighbor.x, possible_neighbor.y);
            if lane != lane_neighbor {
                continue;
            }
            if taken_objects.contains(&(j as f64)) {
                continue;
            }
            let pos_diff = (possible_neighbor.y - reference_object.y).abs();
            let frame_diff = possible_neighbor.frame - reference_object.frame;
            if pos_diff <= threshold_pixel_y && frame_diff <= threshold_frames {
                taken_objects.push(j as f64);
                reference_object = Object::new(possible_neighbor.frame, possible_neighbor.x, possible_neighbor.y, possible_neighbor.width, possible_neighbor.height);
            }
        }
    }
    //objects_count
    (objects_count_1, objects_count_2, objects_count_3, objects_count_4, objects_count_5)
}

fn get_lane(x: i32, y: i32) -> i32 {
    if x >= 564 && x <=  744 {
        0
    } else if x >= 770 && x <= 950 {
        1
    } else if x >= 1000 && x <= 1150 {
        2
    } else if x >= 1180 && x <= 1310 {
        3
    } else {
        4
    }
}

fn spawn_thread_for_lane(area: core::Rect, shared_classifier: &Arc<Mutex<CascadeClassifier>>, batches: &'static Vec<(f64, f64)>) -> Vec<Object> {
    let mut threads_lane_one = vec![];
    let detected_objects: Arc<Mutex<Vec<Object>>> = Arc::new(Mutex::new(Vec::new()));
    for batch in batches {
        let cloned_classifier = shared_classifier.clone();
        let cloned_objects = Arc::clone(&detected_objects);
        let mut frame_index = batch.0;
        threads_lane_one.push(thread::spawn(move || {
            let mut vid = Video::new(&ARGS.0);
            let mut temp_objects = cloned_objects.lock().unwrap();
            loop {
                let frame = vid.get_grayframe(frame_index).unwrap();
                let mut temp_classifier = cloned_classifier.lock().unwrap();
                let cars = temp_classifier.detect_in_rectangle_on_frame(area, &frame);
                for car in cars {
                    let object = Object::new(frame_index, car.x, car.y, car.width, car.height);
                    temp_objects.push(object);
                }
                frame_index += 1f64;
                if frame_index == batch.1 {
                    break;
                }
            }
        }));
    }
    for thread in threads_lane_one {
        let _ = thread.join();
    }

    // Sort list_of_objects by frame
    let list_of_objects = &mut *detected_objects.lock().unwrap();
    list_of_objects.par_sort_by(|a, b| a.frame.partial_cmp(&b.frame).unwrap_or(Ordering::Equal));
    list_of_objects.to_owned()
}