use opencv::{core, highgui};
pub mod detection;
pub mod video;
use detection::*;
use video::*;
use clap::{Arg, App};

const SHOW_GUI: bool = true;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref ARGS: (String, u64) = test();
}

fn test() -> (String, u64) {
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

//fn get_str_ref() -> &'static str {
//    &ARGS.0
//}

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

    let down_first_area = core::Rect {
        x: 564,
        y: 829,
        width: 192,
        height: 183,
    };

    let down_second_area = core::Rect {
        x: 756,
        y: 829,
        width: 175,
        height: 183,
    };

    let up_first_area = core::Rect {
        x: 977,
        y: 829,
        width: 200,
        height: 185,
    };

    let up_second_area = core::Rect {
        x: 1177,
        y: 829,
        width: 155,
        height: 185,
    };

    let up_third_area = core:: Rect {
        x: 1350,
        y: 829,
        width: 200,
        height: 185,
    };

    // let rect_of_interest = core::Rect {
    //     x: 710,
    //     y: 650,
    //     width: 210,
    //     height: 150,
    // };
    let mut idx = 0f64;
    loop {
        let mut frame = match vid.get_frame(idx) {
            Ok(x) => x,
            _ => break,
        };

        let gray = vid.get_grayframe(vid.frame_idx).unwrap();

        //video::draw_rectangle_on_frame(rect_of_interest, &mut frame);

        video::draw_rectangle_on_frame(down_first_area, &mut frame);
        video::draw_rectangle_on_frame(down_second_area, &mut frame);

        video::draw_rectangle_on_frame(up_first_area, &mut frame);
        video::draw_rectangle_on_frame(up_second_area, &mut frame);
        video::draw_rectangle_on_frame(up_third_area, &mut frame);

        // let cars = car_classifier.detect_in_rectangle_on_frame(rect_of_interest, &gray);
        // println!("cars: {} @frame: {}", cars.len(), vid.frame_idx);
        // for car in cars {
        //     video::draw_rectangle_on_frame(car, &mut frame);
        // }

        let cars0 = car_classifier.detect_in_rectangle_on_frame(down_first_area, &gray);
        for car in cars0 {
           video::draw_rectangle_on_frame(car, &mut frame);
        }

        let cars1 = car_classifier.detect_in_rectangle_on_frame(down_second_area, &gray);
        for car in cars1 {
            video::draw_rectangle_on_frame(car, &mut frame);
        }

        let cars2 = car_classifier.detect_in_rectangle_on_frame(up_first_area, &gray);
        for car in cars2 {
            video::draw_rectangle_on_frame(car, &mut frame);
        }

        let cars3 = car_classifier.detect_in_rectangle_on_frame(up_second_area, &gray);
        for car in cars3 {
            video::draw_rectangle_on_frame(car, &mut frame);
        }

        let cars4 = car_classifier.detect_in_rectangle_on_frame(up_third_area, &gray);
        for car in cars4 {
            video::draw_rectangle_on_frame(car, &mut frame);
        }



        if idx >= vid.frame_count {
            break;
        }
        if !SHOW_GUI {
            idx += 100f64;
            continue;
        }
        highgui::imshow(window, &frame).unwrap();
        let key = highgui::wait_key(10).unwrap();
        match key {
            83 => idx += 90f64,
            81 => idx -= 90f64,
            32 => continue,
            -1 => idx += 10f64,
            _ => {
                println!("key pressed: {}", key);
                break;
            }
        };
    }
}
