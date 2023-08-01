use opencv::{prelude::*, Error};
opencv::opencv_branch_4! {
    use opencv::{core::*, imgproc, videoio::{VideoCapture, CAP_ANY}};
}

const FPS: u32 = 15;
const HEIGHT: i32 = 360;
const WIDTH: i32 = 640;

fn brightest_frame(imgs: &[Mat]) -> Result<Mat, Error> {
    let mut ret = imgs[0].clone();

    for img in &imgs[1..] {
        let mut tmp = Mat::default();
        max(&ret, img, &mut tmp)?;
        ret = tmp;
    }

    Ok(ret)
}

fn make_diff_list(imgs: &[Mat], mask: &Option<Mat>) -> Result<Vec<Mat>, Error> {
    let mut diff_list = vec![];

    for win in imgs.windows(2) {
        let img1 = &win[1];
        let img2 = &win[0];

        let mut diff = Mat::default();
        if let Some(mask) = mask {
            let mut tmp1 = Mat::default();
            let mut tmp2 = Mat::default();
            bitwise_or(img1, mask, &mut tmp1, &no_array())?;
            bitwise_or(img2, mask, &mut tmp2, &no_array())?;

            subtract(&tmp1, &tmp2, &mut diff, &no_array(), -1)?;
        } else {
            subtract(img1, img2, &mut diff, &no_array(), -1)?;
        }

        diff_list.push(diff);
    }

    Ok(diff_list)
}

fn detect_lines(img: &Mat, min_length: f64) -> Result<Vector<Vec4i>, Error> {
    let mut blur = Mat::default();
    imgproc::gaussian_blur(img, &mut blur, Size::new(3, 3), 0., 0., BORDER_DEFAULT)?;

    let mut canny = Mat::default();
    imgproc::canny(&blur, &mut canny, 33., 66., 3, false)?;

    let mut lines: Vector<Vec4i> = Vector::new();
    imgproc::hough_lines_p(
        &canny,
        &mut lines,
        1.,
        std::f64::consts::PI / 180.,
        10,
        min_length,
        3.,
    )?;

    Ok(lines)
}

pub struct Detector {
    cap: VideoCapture,
    exposure: f32,
    mask: Option<Mat>,
}

impl Detector {
    pub fn new(url: &str, use_mask: bool) -> Result<Self, Error> {
        let cap = VideoCapture::from_file(url, CAP_ANY).unwrap();

        if !cap.is_opened().unwrap() {
            panic!()
        }

        let mask = use_mask.then_some({
            let mut mask = Mat::zeros(HEIGHT, WIDTH, CV_8UC1).unwrap().to_mat()?;

            imgproc::rectangle_points(
                &mut mask,
                Point::new(463, 337),
                Point::new(640, 360),
                Scalar::new(255., 0., 0., 0.),
                -1,
                imgproc::LINE_8,
                0,
            )?;

            mask
        });

        Ok(Self {
            cap,
            exposure: 1.,
            mask,
        })
    }

    // 露出時間を処理
    pub fn detect(&mut self) -> Result<bool, Error> {
        let mut imgs = vec![];

        let frame_range = (self.exposure * FPS as f32) as usize;
        for _ in 0..frame_range {
            let mut frame = Mat::default();
            let has_next = self.cap.read(&mut frame)?;

            if !has_next {}

            let mut gray_frame = Mat::default();
            imgproc::cvt_color(&frame, &mut gray_frame, imgproc::COLOR_BGR2GRAY, 0)?;
            imgs.push(gray_frame);
        }

        if imgs.len() < 3 {}

        let diff_list = make_diff_list(&imgs, &self.mask)?;

        let brightest = brightest_frame(&diff_list)?;

        let detected = detect_lines(&brightest, 10.)?;

        Ok(!detected.is_empty())
    }
}
