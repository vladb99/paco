use opencv::{core, imgproc, prelude::*, videoio};
pub struct Video {
    pub file: &'static str,
    pub frame_count: f64,
    pub frame_idx: f64,
    fd_video: videoio::VideoCapture,
}
impl Video {
    pub fn new(file: &'static str) -> Video {
        let vid = videoio::VideoCapture::from_file(file, videoio::CAP_ANY).unwrap();
        Video {
            file: file,
            frame_count: vid.get(videoio::CAP_PROP_FRAME_COUNT).unwrap(),
            frame_idx: 0f64,
            fd_video: vid,
        }
    }
    pub fn get_frame(&mut self, idx: f64) -> Result<Mat, String> {
        match idx {
            i if i > -1f64 && i < self.frame_count => {
                self.fd_video.set(videoio::CAP_PROP_POS_FRAMES, i).unwrap();
                self.frame_idx = idx;
            }
            i if i < 0f64 => {
                self.fd_video
                    .set(videoio::CAP_PROP_POS_FRAMES, self.frame_idx)
                    .unwrap();
                self.frame_idx += 1f64;
            }
            _ => return Err("end of video".to_string()),
        };
        let mut frame = Mat::default();
        self.fd_video.read(&mut frame).unwrap();
        Ok(frame)
    }
    pub fn get_grayframe(&mut self, idx: f64) -> Result<Mat, String> {
        let frame = self.get_frame(idx).unwrap();
        let mut gray = Mat::default();
        imgproc::cvt_color(&frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0).unwrap();
        Ok(gray)
    }
    
}
pub fn draw_rectangle_on_frame(rect: core::Rect, frame: &mut Mat) {
    imgproc::rectangle(
        frame,
        rect,
        core::Scalar::new(1f64, -1f64, -1f64, -1f64),
        2,
        8,
        0,
    )
    .unwrap();
}
