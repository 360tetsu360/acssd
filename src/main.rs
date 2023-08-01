use std::thread;
mod detect;

fn main() {
    let mut detect_cam1 =
        detect::Detector::new("rtsp://192.168.2.21:8554/video1_unicast", true).unwrap();
    let mut detect_cam2 =
        detect::Detector::new("rtsp://192.168.2.22:8554/video1_unicast", true).unwrap();
    let mut detect_cam3 =
        detect::Detector::new("rtsp://192.168.2.23:8554/video1_unicast", true).unwrap();

    thread::spawn(move || loop {
        if detect_cam1.detect().unwrap() {
            dbg!();
        }
    });

    thread::spawn(move || loop {
        if detect_cam2.detect().unwrap() {
            dbg!();
        }
    });

    loop {
        if detect_cam3.detect().unwrap() {
            dbg!();
        }
    }
}
