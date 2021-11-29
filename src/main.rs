use rayon::prelude::*;
use std::sync::{Arc, mpsc, Mutex};

pub mod detection;
pub mod video;

use detection::*;
use video::*;
use clap::{Arg, App};

#[macro_use]
extern crate lazy_static;


lazy_static! {
    pub static ref ARGS: (String, f64) = init_args();
}

#[derive(Debug, Copy, Clone)]
pub struct Object {
    pub frame: i32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Object {
    pub fn new(frame: i32, x: i32, y: i32, width: i32, height: i32) -> Object {
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

    let (tx, rx) = mpsc::channel();

    let skipping = 4;
    let mut vid = Video::new(&ARGS.0);
    let mut vid_container = Vec::new();
    for fidx in (0..number_of_frames as i32).step_by(skipping) {
        vid_container.push((fidx, vid.get_grayframe(fidx as f64).unwrap()));
    }

    let first_rect_x_range = 564..744;
    let rect_y_range = 830..1010;
    let second_rect_x_range = 770..920;
    let third_rect_x_range = 1000..1150;
    let fourth_rect_x_range = 1180..1310;
    let fifth_rect_x_range = 1350..1480;
    let car_classifier = Arc::new(Mutex::new(CascadeClassifier::new("cars.xml")));
    vid_container.into_par_iter().for_each_with(tx, |s, x| {
        let mut temp_classifer = car_classifier.lock().unwrap();
        let cars = temp_classifer.detect_on_frame(&x.1);
        for car in cars {
            if first_rect_x_range.contains(&car.x) {
                if rect_y_range.contains(&car.y) {
                    let object = Object::new(x.0, car.x, car.y, car.width, car.height);
                    s.send((0, object)).unwrap();
                }
            } else if second_rect_x_range.contains(&car.x) {
                if rect_y_range.contains(&car.y) {
                    let object = Object::new(x.0, car.x, car.y, car.width, car.height);
                    s.send((1, object)).unwrap();
                }
            } else if third_rect_x_range.contains(&car.x) {
                if rect_y_range.contains(&car.y) {
                    let object = Object::new(x.0, car.x, car.y, car.width, car.height);
                    s.send((2, object)).unwrap();
                }
            } else if fourth_rect_x_range.contains(&car.x) {
                if rect_y_range.contains(&car.y) {
                    let object = Object::new(x.0, car.x, car.y, car.width, car.height);
                    s.send((3, object)).unwrap();
                }
            } else if fifth_rect_x_range.contains(&car.x) {
                if rect_y_range.contains(&car.y) {
                    let object = Object::new(x.0, car.x, car.y, car.width, car.height);
                    s.send((4, object)).unwrap();
                }
            }
        }
    });

    let mut firstlane_obj: Vec<Object> = vec![];
    let mut secondlane_obj: Vec<Object> = vec![];
    let mut thirdlane_obj: Vec<Object> = vec![];
    let mut fourlane_obj: Vec<Object> = vec![];
    let mut fivelane_obj: Vec<Object> = vec![];

    for (i, obj) in rx {
        match i {
            0 => firstlane_obj.push(obj),
            1 => secondlane_obj.push(obj),
            2 => thirdlane_obj.push(obj),
            3 => fourlane_obj.push(obj),
            4 => fivelane_obj.push(obj),
            _ => {}
        }
    }

    firstlane_obj.sort_by(|a, b| a.frame.partial_cmp(&b.frame).unwrap());
    secondlane_obj.sort_by(|a, b| a.frame.partial_cmp(&b.frame).unwrap());
    thirdlane_obj.sort_by(|a, b| a.frame.partial_cmp(&b.frame).unwrap());
    fourlane_obj.sort_by(|a, b| a.frame.partial_cmp(&b.frame).unwrap());
    fivelane_obj.sort_by(|a, b| a.frame.partial_cmp(&b.frame).unwrap());
    let cars_count_lane_one = count_objects(&firstlane_obj);
    let cars_count_lane_two = count_objects(&secondlane_obj);
    let cars_count_lane_three = count_objects(&thirdlane_obj);
    let cars_count_lane_four = count_objects(&fourlane_obj);
    let cars_count_lane_five = count_objects(&fivelane_obj);
    println!("{} {} {} {} {}", cars_count_lane_one, cars_count_lane_two, cars_count_lane_three, cars_count_lane_four, cars_count_lane_five);
}

fn count_objects(detected_objects: &Vec<Object>) -> i32 {
    let mut objects_count = 0;
    let threshold_pixel_y = 75;
    let threshold_frames: i32 = 10;
    let mut taken_objects: Vec<i32> = Vec::new();
    for (i, object) in detected_objects.iter().enumerate() {
        if taken_objects.contains(&(i as i32)) {
            continue;
        }
        taken_objects.push(i as i32);
        objects_count += 1;
        let mut reference_object = Object::new(object.frame, object.x, object.y, object.width, object.height);
        for (j, possible_neighbor) in detected_objects.iter().enumerate() {
            if possible_neighbor.cmp(&object) {
                continue;
            }
            if taken_objects.contains(&(j as i32)) {
                continue;
            }
            let pos_diff = (possible_neighbor.y - reference_object.y).abs();
            let frame_diff = possible_neighbor.frame - reference_object.frame;
            if pos_diff <= threshold_pixel_y && frame_diff <= threshold_frames {
                taken_objects.push(j as i32);
                reference_object = Object::new(possible_neighbor.frame, possible_neighbor.x, possible_neighbor.y, possible_neighbor.width, possible_neighbor.height);
            }
        }
    }
    //println!("{:?}", taken_objects);
    objects_count
}