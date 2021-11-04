use opencv::{core, objdetect, prelude::*, types};
pub struct CascadeClassifier {
    classifier: objdetect::CascadeClassifier,
}
impl CascadeClassifier {
    pub fn new(xml_file: &'static str) -> CascadeClassifier {
        let cascade_xml = core::find_file(xml_file, true, false).unwrap();
        CascadeClassifier {
            classifier: objdetect::CascadeClassifier::new(&cascade_xml).unwrap(),
        }
    }
    pub fn detect_on_frame(&mut self, frame: &Mat) -> types::VectorOfRect {
        let mut objs = types::VectorOfRect::new();
        self.classifier
            .detect_multi_scale(
                frame,
                &mut objs,
                1.1,
                2,
                objdetect::CASCADE_SCALE_IMAGE,
                core::Size {
                    width: 30,
                    height: 30,
                },
                core::Size {
                    width: 500,
                    height: 500,
                },
            )
            .unwrap();
        objs
    }
    pub fn detect_in_rectangle_on_frame(
        &mut self,
        rect: core::Rect,
        frame: &Mat,
    ) -> types::VectorOfRect {
        let objs = self.detect_on_frame(frame);
        let mut ret = types::VectorOfRect::new();
        for obj in objs {
            if is_point_in_rectangle(obj.x + obj.width / 2, obj.y + obj.height / 2, rect) {
                ret.push(obj);
            }
        }
        ret
    }
}
pub fn is_point_in_rectangle(x: i32, y: i32, rect: core::Rect) -> bool {
    if x >= rect.x && x <= rect.width + rect.x {
        if y >= rect.y && y <= rect.height + rect.y {
            return true;
        }
    }
    return false;
}
