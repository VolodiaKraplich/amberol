mod application;
mod audio;
mod cover_picture;
mod drag_overlay;
mod i18n;
mod marquee;
mod playback_control;
mod playlist_view;
mod queue_row;
mod search;
mod song_cover;
mod song_details;
mod sort;
mod utils;
mod volume_control;
mod waveform_view;
mod window;

use std::env;

const APPLICATION_ID: &str = env!("APPLICATION_ID");
const GETTEXT_PACKAGE: &str = env!("GETTEXT_PACKAGE");

use gettextrs::{LocaleCategory, bind_textdomain_codeset, setlocale, textdomain};
use gtk::{gio, glib, prelude::*};

use self::application::Application;

/// Sets up the application environment
fn setup_environment() {
  unsafe {
    env::set_var("RUST_LOG", "info");
  }
}

/// Calls the C function `bindtextdomain` from `gettext-sys` to bind to GResource.
fn bind_textdomain_to_gresource(domain: &str) {
  use std::ffi::CString;
  use std::ptr;

  let domain_c = CString::new(domain).unwrap();

  unsafe {
    gettext_sys::bindtextdomain(domain_c.as_ptr(), ptr::null());
  }
}

/// Sets up internationalization support
fn setup_i18n() {
  setlocale(LocaleCategory::LcAll, "");
  bind_textdomain_to_gresource(GETTEXT_PACKAGE);
  bind_textdomain_codeset(GETTEXT_PACKAGE, "UTF-8").ok();
  textdomain(GETTEXT_PACKAGE).ok();
}

/// Configures PulseAudio environment variables
fn setup_audio_environment() {
  let app_id = APPLICATION_ID.trim_end_matches(".Devel");
  unsafe {
    env::set_var("PULSE_PROP_application.icon_name", app_id);
    env::set_var("PULSE_PROP_application.name", "Amberol");
    env::set_var("PULSE_PROP_media.role", "music");
  }
}

/// Loads application resources
fn setup_resources() -> Result<()> {
  gio::resources_register_include!("amberol.gresource")
    .expect("Failed to register GResource bundle.");
  Ok(())
}

/// Sets up GTK and GStreamer
fn setup_frameworks() -> Result<()> {
  glib::set_application_name("Amberol");
  glib::set_program_name(Some("amberol"));
  gst::init()?;
  Ok(())
}

fn main() -> glib::ExitCode {
  setup_environment();
  setup_i18n();
  setup_audio_environment();

  if setup_resources().is_err() {
    return glib::ExitCode::FAILURE;
  }

  if setup_frameworks().is_err() {
    return glib::ExitCode::FAILURE;
  }

  let ctx = glib::MainContext::default();
  let _guard = ctx
    .acquire()
    .unwrap_or_else(|_| panic!("Failed to acquire main context"));

  let app = Application::new();

  app.run()
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
