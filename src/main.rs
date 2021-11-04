use opencv::{core, highgui};
pub mod detection;
pub mod video;
use detection::*;
use video::*;

const SHOW_GUI: bool = true;

fn main() {
    let window = "video";
    if SHOW_GUI {
        highgui::named_window(window, 1).unwrap()
    };
    let mut vid = Video::new("Cars.mp4");
    let mut car_classifier = CascadeClassifier::new("cars.xml");
    println!("total frame count: {}", vid.frame_count);
    let rect_of_interest = core::Rect {
        x: 710,
        y: 650,
        width: 210,
        height: 150,
    };
    let mut idx = 0f64;
    loop {
        let mut frame = match vid.get_frame(idx) {
            Ok(x) => x,
            _ => break,
        };
        let gray = vid.get_grayframe(vid.frame_idx).unwrap();
        video::draw_rectangle_on_frame(rect_of_interest, &mut frame);
        let cars = car_classifier.detect_in_rectangle_on_frame(rect_of_interest, &gray);
        println!("cars: {} @frame: {}", cars.len(), vid.frame_idx);
        for car in cars {
            video::draw_rectangle_on_frame(car, &mut frame);
        }
        if idx >= vid.frame_count {
            break;
        }
        if !SHOW_GUI {
            idx += 50f64;
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
