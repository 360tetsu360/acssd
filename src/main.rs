use std::collections::HashMap;
use std::{path::PathBuf, thread};

use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Result};
use tokio::sync::mpsc;
use tokio_stream::{StreamExt, StreamMap};

use crate::detect::{detection_loop, DetectionStatusMsg, Detector};
mod detect;
//mod cam;
//mod web;

#[tokio::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let out_dir_tmp = PathBuf::from("A!");

    let detector1 = Detector::new(
        "rtsp://192.168.2.21:8554/video1_unicast",
        true,
        out_dir_tmp.clone(),
    )
    .unwrap();
    let detector2 = Detector::new(
        "rtsp://192.168.2.22:8554/video1_unicast",
        true,
        out_dir_tmp.clone(),
    )
    .unwrap();
    let detector3 =
        Detector::new("rtsp://192.168.2.23:8554/video1_unicast", true, out_dir_tmp).unwrap();

    let (s1, r1) = mpsc::channel(10);
    let (s2, r2) = mpsc::channel(10);
    let (s3, r3) = mpsc::channel(10);

    let mut thread_map = HashMap::new();
    thread_map.insert("cam1", thread::spawn(move || detection_loop(detector1, s1)));
    thread_map.insert("cam2", thread::spawn(move || detection_loop(detector2, s2)));
    thread_map.insert("cam3", thread::spawn(move || detection_loop(detector3, s3)));

    //let meteor_list1 = Arc::new(Mutex::new(vec![]));
    //let meteor_list2 = Arc::new(Mutex::new(vec![]));
    //let meteor_list3 = Arc::new(Mutex::new(vec![]));

    let r1 = tokio_stream::wrappers::ReceiverStream::new(r1);
    let r2 = tokio_stream::wrappers::ReceiverStream::new(r2);
    let r3 = tokio_stream::wrappers::ReceiverStream::new(r3);

    let mut stream_map = StreamMap::new();
    stream_map.insert("cam1", r1);
    stream_map.insert("cam2", r2);
    stream_map.insert("cam3", r3);

    //let meteor_list_cp1 = meteor_list1.clone();
    //let meteor_list_cp2 = meteor_list2.clone();
    //let meteor_list_cp3 = meteor_list3.clone();
    tokio::spawn(async move {
        loop {
            let res = stream_map.next().await;

            if res.is_none() {
                break;
            }

            let (key, val) = res.unwrap();

            match val {
                DetectionStatusMsg::Detected(detected) => {
                    log::info!(
                        "Meteor detected {:?} {}",
                        &detected.img_path.as_os_str(),
                        detected.time
                    );

                    dbg!();
                }
                DetectionStatusMsg::Error(e) => {
                    log::warn!("{} err {}", key, e);
                    stream_map.remove(key);
                    thread_map.remove(key);
                }
            }
        }
    });

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(app_config)
    })
    .bind(("0.0.0.0", 8080))
    .unwrap()
    .run()
    .await
    .unwrap();
}

fn app_config(config: &mut web::ServiceConfig) {
    config.service(web::scope("").service(index));
}

#[get("/")]
async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../web/index.html")))
}
