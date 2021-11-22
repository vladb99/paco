use std::cmp::Ordering;
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
    let number_of_frames = ARGS.1;

    // Open files
    let car_classifier = CascadeClassifier::new("cars.xml");

    // Define the areas for the 5 lanes
    let down_first_area = core::Rect {
        x: 564,
        y: 830,
        width: 180,
        height: 180,
    };
    let down_second_area = core::Rect {
        x: 770,
        y: 830,
        width: 150,
        height: 180,
    };

    let up_first_area = core::Rect {
        x: 1000,
        y: 830,
        width: 150,
        height: 180,
    };

    let up_second_area = core::Rect {
        x: 1180,
        y: 830,
        width: 130,
        height: 180,
    };

    let up_third_area = core::Rect {
        x: 1400,
        y: 830,
        width: 130,
        height: 180,
    };

    let first_area = core::Rect {
        x: 564,
        y: 829,
        width: 356,
        height: 180,
    };

    let second_area = core::Rect {
        x: 977,
        y: 830,
        width: 603,
        height: 185,
    };

    // Create multiple threads and distribute the frames evenly
    let step = 325f64;
    let mut batches: Vec<(f64, f64)> = Vec::new();
    let mut idx = 0f64;
    loop {
        let start = idx;
        idx += step;
        let mut end = idx;
        if end > number_of_frames {
            end = number_of_frames;
            if start == end {
                break;
            }
            let batch = (start, end);
            batches.push(batch);
            break;
        }
        let batch = (start, end);
        batches.push(batch);
    }

    let shared_classifier = Arc::new(Mutex::new(car_classifier));

    let static_batch: &Vec<(f64, f64)> = Box::leak(Box::from(batches));

    // let detected_objects_lane_one = spawn_thread_for_lane(down_first_area, &shared_classifier, static_batch);
    // let detected_objects_lane_two = spawn_thread_for_lane(down_second_area, &shared_classifier, static_batch);
    // let detected_objects_lane_three = spawn_thread_for_lane(up_first_area, &shared_classifier, static_batch);
    // let detected_objects_lane_four = spawn_thread_for_lane(up_second_area, &shared_classifier, static_batch);
    // let detected_objects_lane_five = spawn_thread_for_lane(up_third_area, &shared_classifier, static_batch);

    let mut detected_objects_lane_one = spawn_thread_for_lane(first_area, &shared_classifier, static_batch);
    let mut detected_objects_lane_two = spawn_thread_for_lane(second_area, &shared_classifier, static_batch);

    detected_objects_lane_one.append(&mut detected_objects_lane_two);

    let counted_objects = count_objects(detected_objects_lane_one);



    // let cars_count_lane_one = count_objects(detected_objects_lane_one);
    // let cars_count_lane_two = count_objects(detected_objects_lane_two);
    // let cars_count_lane_three = count_objects(detected_objects_lane_three);
    // let cars_count_lane_four = count_objects(detected_objects_lane_four);
    // let cars_count_lane_five = count_objects(detected_objects_lane_five);

    println!("{} {} {} {} {}", counted_objects.0, counted_objects.1, counted_objects.2, counted_objects.3, counted_objects.4);
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