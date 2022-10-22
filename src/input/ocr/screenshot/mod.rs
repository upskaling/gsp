mod gnome_screenshot;
mod xfce4_screenshooter;

use self::xfce4_screenshooter::xfce4_screenshooter_region;
use which::which;

pub struct Screenshot {
    pub path: String,
}

impl Screenshot {
    pub fn new() -> Self {
        Screenshot {
            path: "/dev/shm/screenshot.png".to_string(),
        }
    }

    pub fn capture(&self) -> String {
        if which("xfce4-screenshooter").is_ok() {
            xfce4_screenshooter_region(&self.path);
        } else if which("gnome-screenshot").is_ok() {
            gnome_screenshot::gnome_screenshot_area(&self.path);
        } else {
            panic!("No screenshot tool found");
        }

        self.path.clone()
    }
}
