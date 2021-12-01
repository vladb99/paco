use std::sync::{Arc, Mutex};
use opencv::{types};
use rayon::prelude::*;
use detection::*;
use video::*;
use opencv::core::{Mat, Rect, Vector};
use std::env;
//use std::env::set_var;

pub mod detection;
pub mod video;

//////////////////////////////////////////////////////////////////////
// FileName:            main.rs
// FileType:            Rust - Source file
// Author:              Vlad Bratulescu
// Task:                Aufgabe 2
// Created On:          07.11.2021
// Last Modified On :   01.12.2021 23.50
//////////////////////////////////////////////////////////////////////

fn main() {
    // Get Args
    let mut file_name:&str = "";
    let mut number_of_frames = 2690;

    let mut next_f = false;
    let mut next_n = false;
    for argument in env::args() {
        if argument == "-f" {
            next_f = true;
            continue
        }
        if argument == "-n" {
            next_n = true;
            continue
        }
        if next_f {
            file_name = Box::leak(Box::from(&*argument));
            next_f = false;
            continue;
        }
        if next_n {
            number_of_frames = argument.parse().unwrap();
            next_n = false;
            continue;
        }
    }

    // Setting the threads number that is spawned by rayon
    //set_var("RAYON_NUM_THREADS", "1");

    // Open files
    //let mut car_classifier = Arc::new(Mutex::new(CascadeClassifier::new("cars.xml")));
    let skipping = 20;
    let vid_container: Vec<(i32, Mat)> = (0..number_of_frames as i32).into_par_iter()
        .filter(|frame_index| frame_index % skipping == 0)
        .map(|frame_index| {
            let mut vid = Video::new(file_name);
            (frame_index, vid.get_grayframe(frame_index as f64).unwrap())
        })
        .collect();

    let cars: Vec<(i32, Vector<Rect>)> = vid_container.into_par_iter()
        .map(|tuple| {
            //let mut temp_classifer = car_classifier.lock().unwrap();
            let mut car_classifier = CascadeClassifier::new("cars.xml");
            let objects = car_classifier.detect_on_frame(&tuple.1);
            let mut filtered_objects: Vector<Rect> = types::VectorOfRect::new();
            for object in objects {
                if is_in_area(object.x, object.y) {
                    filtered_objects.push(object);
                }
            }
            (tuple.0, filtered_objects)
        })
        .collect();

    // Sort cars by frame index
    //cars.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let second_list = Arc::new(Mutex::new(cars.clone()));
    let counted_objects: Vec<(i32, i32, i32, i32, i32)> = (cars.clone()).into_par_iter()
        .map(|tuple| {
            let mut count_first_lane = 0;
            let mut count_second_lane = 0;
            let mut count_third_lane = 0;
            let mut count_fourth_lane = 0;
            let mut count_fifth_lane = 0;

            for car in &tuple.1 {
                let ref_center_x = car.x + car.width / 2;
                let ref_lane = get_lane(ref_center_x);
                let mut already_counted = false;
                for next_frame in second_list.lock().unwrap().iter() {
                    // find next frame, which has the difference of skipping constant
                    if next_frame.0 == tuple.0 || next_frame.0 - tuple.0 != skipping { continue; }
                    for possible_same_car in &next_frame.1 {
                        let center_x = possible_same_car.x + possible_same_car.width / 2;
                        let lane = get_lane(center_x);
                        if ref_lane != lane { continue; }
                        if lane == 1 || lane == 2 {
                            if !(car.y < possible_same_car.y) {
                                continue;
                            }
                        } else {
                            if !(car.y > possible_same_car.y) {
                                continue;
                            }
                        }
                        already_counted = true;
                        match lane {
                            0 => count_first_lane += 1,
                            1 => count_second_lane += 1,
                            2 => count_third_lane += 1,
                            3 => count_fourth_lane += 1,
                            4 => count_fifth_lane += 1,
                            _ => {}
                        }
                    }
                    // break to speed up, because there is not other frame has the same difference
                    break;
                }
                if already_counted {
                    continue;
                }
                // if no car was identified in next frame, that is the same car, then count this only car
                match ref_lane {
                    0 => count_first_lane += 1,
                    1 => count_second_lane += 1,
                    2 => count_third_lane += 1,
                    3 => count_fourth_lane += 1,
                    4 => count_fifth_lane += 1,
                    _ => {}
                }
            }
            (count_first_lane, count_second_lane, count_third_lane, count_fourth_lane, count_fifth_lane)
        })
        .collect();

    let mut count_first_lane = 0;
    let mut count_second_lane = 0;
    let mut count_third_lane = 0;
    let mut count_fourth_lane = 0;
    let mut count_fifth_lane = 0;

    for tuple in counted_objects {
        count_first_lane += tuple.0;
        count_second_lane += tuple.1;
        count_third_lane += tuple.2;
        count_fourth_lane += tuple.3;
        count_fifth_lane += tuple.4;
    }

    println!("{} {} {} {} {}", count_first_lane, count_second_lane, count_third_lane, count_fourth_lane, count_fifth_lane);
}

fn is_in_area(x: i32, y: i32) -> bool {
    if x >= 564 && x <= 1580 && y >= 785 && y <= 1010 {
        return true;
    }
    return false;
}

fn get_lane(x: i32) -> i32 {
    if x >= 564 && x <= 744 {
        0
    } else if x >= 770 && x <= 950 {
        1
    } else if x >= 1000 && x <= 1155 {
        2
    } else if x >= 1170 && x <= 1300 {
        3
    } else  if x >= 1320 && x <= 1470 {
        4
    } else {
        -1
    }
}