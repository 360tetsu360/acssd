pub struct Camera {
}

pub struct CameraConfig {
    app_ver: String,
    product_model: String,
    hostname: String,
    kernel_ver: String,
    atomhack_ver: String,
    digest: Option<()>,
    reboot: bool,
    reboot_schedule: Option<()>,
    recording_local_schedule: bool,
    recording_local_schedule_list: Option<()>,
    rtsp_video0: bool,
    rtsp_audio0: bool,
    rtsp_main_format_hevc: bool,
    rtsp_video1: bool,
    rtsp_audio1: bool,
    rtsp_over_http: bool,
    storage_sdcard: String,
    storage_sdcard_publish: bool,
    storage_sdcard_path: String,
    storage_sdcard_remove: bool,
    storage_sdcard_remove_days: u32,
    storage_cifs: bool,
    storage_cifs_path: String,
    storage_cifs_remove: bool,
    storage_cifs_remove_days: u32,
    storage_cifsserver: Option<String>,
    storage_cifsuser: Option<String>,
    storage_cifspasswd: Option<String>,
    timelapse: bool,
    timelapse_schedule: Option<()>,
    timelapse_path: String,
    timelapse_interval: u32,
    timelapse_count: u32,
    webhook: bool,
    webhook_url: String,
    webhook_alarm_event: bool,
    webhook_alarm_info: bool,
    webhook_alarm_video_finish: bool,
    webhook_alarm_video: bool,
    webhook_alarm_pict_finish: bool,
    webhook_alarm_pict: bool,
    webhook_recore_event: bool,
    webhook_timelapse_start: bool,
    webhook_timelapse_event: bool,
    webhook_timelapse_finish: bool,
    cruise: bool,
    cruise_list: Option<()>,
    minimize_alarm_cycle: bool,
    aws_video_disable: bool,
    custom_zip: bool,
    custom_zip_url: Option<()>,
    healthcheck: bool,
    healthcheck_ping_url: Option<()>,
    locale: String
}

impl CameraConfig {
    pub async fn from_url(url: &str) -> Self {

    }

    pub fn new(res: &str) -> Self {

    }
}

pub struct CameraStatus {
    latestver: String,
    timelapse: String,
    timestamp: String,
}