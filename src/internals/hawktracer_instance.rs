include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[allow(unused_imports)]
use super::hawktracer_listener::*;
use std::path::PathBuf;

pub struct HawktracerInstance {}

pub enum HawktracerListenerType {
    ToFile {
        file_path: PathBuf,
        buffer_size: usize,
    },
    TCP {
        port: u32,
        buffer_size: usize,
    },
}

impl HawktracerInstance {
    pub fn new() -> HawktracerInstance {
        use std::os::raw::c_char;
        let p: *mut *mut c_char = std::ptr::null_mut();
        unsafe {
            ht_init(0, p);
        }
        HawktracerInstance {}
    }

    pub fn create_listener<'a>(
        &'a self,
        listener_type: HawktracerListenerType,
    ) -> Box<dyn HawktracerListener<'a>> {
        use super::hawktracer_listener_file::HawktracerListenerFile;
        use super::hawktracer_listener_tcp::HawktracerListenerTCP;

        let listener: Box<dyn HawktracerListener> = match listener_type {
            HawktracerListenerType::ToFile {
                file_path,
                buffer_size,
            } => Box::new(HawktracerListenerFile::new(file_path, buffer_size)),
            HawktracerListenerType::TCP { port, buffer_size } => {
                Box::new(HawktracerListenerTCP::new(port, buffer_size))
            }
        };

        listener
    }
}

impl Drop for HawktracerInstance {
    fn drop(&mut self) {
        unsafe {
            ht_deinit();
        }
    }
}
