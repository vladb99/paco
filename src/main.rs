use std::collections::HashMap;
use std::process::id;
use opencv::{core, highgui};
pub mod detection;
pub mod video;
use detection::*;
use video::*;
use clap::{Arg, App};
use std::thread;
use std::time::Duration;
use opencv::core::{Mat, Rect, Vector, VectorExtern};
use opencv::dnn::print;

const SHOW_GUI: bool = true;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref ARGS: (String, f64) = init_args();
}

pub struct TrackableObject {
    pub object_id: u64,
    pub positions: Vec<Rect>,
    pub frames_for_positions: Vec<f64>,
    pub frames_to_disappear: u64,
    pub was_seen_again: bool,
    pub disappeard: bool,
}

impl TrackableObject {
    pub fn new(id: u64) -> TrackableObject {
        TrackableObject {
            object_id: id,
            positions: Vec::new(),
            frames_for_positions: Vec::new(),
            frames_to_disappear: 3,
            was_seen_again: true,
            disappeard: false,
        }
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
    println!("Path to video: {}", ARGS.0);
    println!("Number of frames: {}", ARGS.1);

    let window = "video";
    if SHOW_GUI {
        highgui::named_window(window, 1).unwrap()
    };
    let mut vid = Video::new(&ARGS.0);
    let mut car_classifier = CascadeClassifier::new("cars.xml");
    println!("total frame count: {}", vid.frame_count);
    let number_of_frames = ARGS.1;
    let down_first_area = core::Rect {
        x: 564,
        y: 830,
        width: 180,
        height: 180,
    };

    let mut frames: Vec<Mat> = Vec::new();
    let mut idx = 0f64;
    loop {
        let mut frame = match vid.get_frame(idx) {
            Ok(x) => x,
            _ => break,
        };
        let gray = vid.get_grayframe(vid.frame_idx).unwrap();
        frames.push(gray);
        if idx >= number_of_frames {
            break;
        }
    }
    println!("{} frames have been read", frames.len());

    //let mut objects: Vec<TrackableObject> = Vec::new();
    // loop {
    //     let mut frame = match vid.get_frame(idx) {
    //         Ok(x) => x,
    //         _ => break,
    //     };
    //     let gray = vid.get_grayframe(vid.frame_idx).unwrap();
    //     video::draw_rectangle_on_frame(down_first_area, &mut frame);
    //     let cars = car_classifier.detect_in_rectangle_on_frame(down_first_area, &gray);
    //     println!("cars: {} @frame: {}", cars.len(), vid.frame_idx);

        // alle frames in einer datenstruktur
        // radius zwischen positionen
        // wofuer braucht man die geschwindigkeit?
        // von mehreren threads in einer liste reinschreiben?




        // let mut count_active_objects = 0;
        // if !objects.is_empty() {
        //     for object in &objects {
        //         if !object.disappeard {
        //             count_active_objects += 1;
        //         }
        //     }
        // }
        //
        // for car in cars {
        //     //println!("{:?}", car);
        //     video::draw_rectangle_on_frame(car, &mut frame);
        //     //println!("len is {}", objects.len());
        //     if objects.is_empty() || count_active_objects == 0 {
        //         println!("Added new trackable object!");
        //         let mut object = TrackableObject::new(0);
        //         object.positions.push(car);
        //         object.frames_for_positions.push(idx);
        //         objects.push(object);
        //     } else {
        //         let mut found_match = false;
        //         for object in objects.iter_mut() {
        //             if !object.disappeard {
        //                 let last_position = object.positions.last().unwrap();
        //                 let pos_diff = (car.y - last_position.y).abs();
        //                 // Wenn differenz groesser als 15 Pixel, dann wahrscheinlich nicht das gleiche auto
        //                 // alternative noch breide und hoehe im betracht nehmen
        //                 if pos_diff > 30 {
        //                     object.was_seen_again = false;
        //                     continue;
        //                 } else {
        //                     found_match = true;
        //                     println!("Added car to existing object!");
        //                     object.positions.push(car);
        //                     object.frames_for_positions.push(idx);
        //                     object.was_seen_again = true;
        //                     object.frames_to_disappear = 5;
        //                     break;
        //                 }
        //             }
        //         }
        //         if !found_match {
        //             println!("Added new trackable object!");
        //             let mut object = TrackableObject::new(0);
        //             object.positions.push(car);
        //             object.frames_for_positions.push(idx);
        //             objects.push(object);
        //         }
        //     }
        // }
        //
        // for object in objects.iter_mut() {
        //     if !object.was_seen_again && !object.disappeard {
        //         object.frames_to_disappear -= 1;
        //         if object.frames_to_disappear == 0 {
        //             object.disappeard = true;
        //             println!("Object disappeard")
        //         }
        //     } else if object.was_seen_again  {
        //         object.was_seen_again = false;
        //     }
        // }

    //     if idx >= number_of_frames {
    //         break;
    //     }
    //     if !SHOW_GUI {
    //         idx += 100f64;
    //         continue;
    //     }
    //     highgui::imshow(window, &frame).unwrap();
    //     let key = highgui::wait_key(10).unwrap();
    //     match key {
    //         83 => idx += 90f64,
    //         81 => idx -= 90f64,
    //         32 => continue,
    //         -1 => idx += 1f64,
    //         _ => {
    //             println!("key pressed: {}", key);
    //             break;
    //         }
    //     };
    // }
}

// fn main() {
//     println!("Path to video: {}", ARGS.0);
//     println!("Number of frames: {}", ARGS.1);
//
//     let window = "video";
//     if SHOW_GUI {
//         highgui::named_window(window, 1).unwrap()
//     };
//
//     let mut vid = Video::new(&ARGS.0);
//     let mut car_classifier = CascadeClassifier::new("cars.xml");
//     println!("total frame count: {}", vid.frame_count);
//
//     let down_first_area = core::Rect {
//         x: 564,
//         y: 830,
//         width: 180,
//         height: 180,
//     };



    // let number_of_frames = ARGS.1;
    // let step: u64 = (number_of_frames / 4.0) as u64;
    // let mut idx = 0f64;
    // let mut main_frames: Vector<f64> = Vector::new();
    // loop {
    //     println!("{}", idx);
    //     if idx >= number_of_frames {
    //         break;
    //     }
    //     let mut frame = match vid.get_frame(idx) {
    //         Ok(x) => x,
    //         _ => break,
    //     };
    //     let gray = vid.get_grayframe(vid.frame_idx).unwrap();
    //     let cars = car_classifier.detect_in_rectangle_on_frame(down_first_area, &gray);
    //     let mut is_empty = true;
    //     for car in cars {
    //         if is_empty == true {
    //             is_empty = false;
    //             main_frames.push(idx);
    //             println!("pushed main frame {}", idx);
    //


    //             thread::spawn(move || {
    //                 let mut thread_index = idx;
    //                 let mut thread_vid = Video::new(&ARGS.0);
    //                 let mut classifier = CascadeClassifier::new("cars.xml");
    //                 loop {
    //                     println!("hello {}", thread_index);
    //                     let mut frame = match thread_vid.get_frame(thread_index) {
    //                         Ok(x) => x,
    //                         _ => break,
    //                     };
    //                     let gray = thread_vid.get_grayframe(thread_vid.frame_idx).unwrap();
    //                     let thread_cars = classifier.detect_in_rectangle_on_frame(down_first_area, &gray);
    //                     for thread_car in thread_cars {
    //                         println!("I saw a car on frame {}", thread_index)
    //                     }
    //                     thread_index += 1f64;
    //                     //thread::sleep(Duration::from_secs(5));
    //                 }
    //             });
    //         }
    //     }
    //     if is_empty {
    //         idx += 1f64;
    //     } else {
    //         idx += step as f64;
    //     }
    //     //highgui::imshow(window, &gray).unwrap();
    // }

    // thread::spawn(|| {
    //     let number_of_frames = ARGS.1;
    //     for i in 1..10 {
    //
    //     }
    // });

    // let down_second_area = core::Rect {
    //     x: 770,
    //     y: 830,
    //     width: 150,
    //     height: 180,
    // };
    //
    // let up_first_area = core::Rect {
    //     x: 1000,
    //     y: 830,
    //     width: 150,
    //     height: 180,
    // };
    //
    // let up_second_area = core::Rect {
    //     x: 1180,
    //     y: 830,
    //     width: 130,
    //     height: 180,
    // };

    // let up_third_area = core:: Rect {
    //     x: 1350,
    //     y: 830,
    //     width: 130,
    //     height: 180,
    // };
    //
    // let mut idx = 0f64;
    // loop {
    //     if idx >= ARGS.1 {
    //         break;
    //     }
    //     let mut frame = match vid.get_frame(idx) {
    //         Ok(x) => x,
    //         _ => break,
    //     };
    //
    //     let gray = vid.get_grayframe(vid.frame_idx).unwrap();
    //
    //     //video::draw_rectangle_on_frame(rect_of_interest, &mut frame);
    //
    //     video::draw_rectangle_on_frame(down_first_area, &mut frame);
    //     video::draw_rectangle_on_frame(down_second_area, &mut frame);
    //
    //     video::draw_rectangle_on_frame(up_first_area, &mut frame);
    //     video::draw_rectangle_on_frame(up_second_area, &mut frame);
    //     video::draw_rectangle_on_frame(up_third_area, &mut frame);
    //
    //     let mut lane_zero = 0;
    //     let mut lane_one = 0;
    //     let mut lane_two = 0;
    //     let mut lane_three = 0;
    //     let mut lane_four = 0;
    //
    //
    //     let mut map_lane_zero: HashMap<u64, Rect> = HashMap::new();
    //
    //     //let mut map_0: HashMap<Rect, u32> = HashMap::new();
    //     // thread cache
    //     // lane cache
    //
    //     let cars0 = car_classifier.detect_in_rectangle_on_frame(down_first_area, &gray);
    //     for car in cars0 {
    //         video::draw_rectangle_on_frame(car, &mut frame);
    //         println!("car at {:?} added on frame {}", car, idx);
    //         map_lane_zero.insert(idx as u64, car);
    //
    //         // car ist das erkannte auto in einem frame in einem rechteck
    //         // es gibt mehrere cars, da sich mehrere autos in einem rechteck befinden koennen
    //         // man sollte pro erkanntem auto ein neues thread erstellen
    //         // dieser thread sollte das auto tracken um zu sehen wann es das rechteck verlaesst
    //         // in einem thread sollte man eine schleife starten, die durch die naechsten frames geht. das thread wird beendet wenn auto das rechteck verlassen hat. Was bringt das?
    //             // thread schaut sich das naechste frame an. Man versucht wieder ein auto in diesem thread zu erkennen.
    //             // man hat jetzt wieder for car in cars:
    //                 // man geht durch alle diese cars und versucht zu erkennen, ob eins davon wirklich das auto ist. (y1 < y2 anschauen oder geschwindigkeit?) Diese neue position dann irgendwo abspeichern
    //             // passt keins der autos? dass naechstes frame vielleicht auch anschauen? oder wieder was mit geschwindigkeit machen?
    //             // wenn das auto wirklich das sichtfeld verlassen hat, dann hat ein thread sich eine bestimme anzahl an frames angeschaut und dabei ein auto gezaehlt. Was bringt das?
    //         //
    //
    //
    //
    //
    //
    //         // if
    //         // // map_0 schon das auto befindet
    //         // // push map_0
    //         // let build_thread = thread::Builder::new().name(format!("{} {:?}", idx, car));
    //         // let handle_thread = build_thread.spawn(|| {
    //         //     lane_one += 1;
    //         //     video::draw_rectangle_on_frame(car, &mut frame);
    //         //     loop {
    //         //
    //         //     }
    //         // // auto rausschmeissem ++1
    //         // }).unwrap();
    //         // println!("{:?}", car);
    //     }
    //
    //     let cars1 = car_classifier.detect_in_rectangle_on_frame(down_second_area, &gray);
    //     for car in cars1 {
    //         video::draw_rectangle_on_frame(car, &mut frame);
    //     }
    //
    //     let cars2 = car_classifier.detect_in_rectangle_on_frame(up_first_area, &gray);
    //     for car in cars2 {
    //         video::draw_rectangle_on_frame(car, &mut frame);
    //     }
    //
    //     let cars3 = car_classifier.detect_in_rectangle_on_frame(up_second_area, &gray);
    //     for car in cars3 {
    //         video::draw_rectangle_on_frame(car, &mut frame);
    //     }
    //
    //     let cars4 = car_classifier.detect_in_rectangle_on_frame(up_third_area, &gray);
    //     for car in cars4 {
    //         video::draw_rectangle_on_frame(car, &mut frame);
    //     }
    //
    //     if idx >= vid.frame_count {
    //         break;
    //     }
    //     if !SHOW_GUI {
    //         idx += 100f64;
    //         continue;
    //     }
    //     highgui::imshow(window, &frame).unwrap();
    //     //idx += 1.0;
    //     let key = highgui::wait_key(10).unwrap();
    //     match key {
    //         83 => idx += 90f64,
    //         81 => idx -= 90f64,
    //         32 => continue,
    //         -1 => idx += 1f64,
    //         _ => {
    //             println!("key pressed: {}", key);
    //             break;
    //         }
    //     };
    // }
//}