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
const PROFILE: &str = env!("PROFILE");

use gettextrs::{LocaleCategory, bind_textdomain_codeset, setlocale, textdomain};
use gtk::{gio, glib, prelude::*};
use log::{LevelFilter, debug};

use self::application::Application;

/// Calls the C function `bindtextdomain` from `gettext-sys` to bind to GResource.
fn bind_textdomain_to_gresource(domain: &str) {
  use std::ffi::CString;
  use std::ptr;

  let domain_c = CString::new(domain).unwrap();

  unsafe {
    gettext_sys::bindtextdomain(domain_c.as_ptr(), ptr::null());
  }
}

fn main() -> glib::ExitCode {
  let mut builder = pretty_env_logger::formatted_builder();
  if PROFILE == "development" {
    builder.filter(Some("amberol"), LevelFilter::Debug);
  } else {
    builder.filter(Some("amberol"), LevelFilter::Info);
  }
  builder.init();

  // Register embedded resources
  debug!("Loading embedded resources");
  gio::resources_register_include!("amberol.gresource")
    .expect("Failed to register GResource bundle.");

  // Set up gettext to use embedded translations
  debug!("Setting up locale data");
  setlocale(LocaleCategory::LcAll, "");

  // Use our helper function to bind to GResource
  bind_textdomain_to_gresource(GETTEXT_PACKAGE);

  bind_textdomain_codeset(GETTEXT_PACKAGE, "UTF-8")
    .expect("Unable to set the text domain encoding");
  textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");

  // Set up environment variables for PulseAudio
  debug!("Setting up pulseaudio environment");
  let app_id = APPLICATION_ID.trim_end_matches(".Devel");
  unsafe {
    env::set_var("PULSE_PROP_application.icon_name", app_id);
    env::set_var("PULSE_PROP_application.name", "Amberol");
    env::set_var("PULSE_PROP_media.role", "music");
  }

  // Initialize application metadata
  debug!("Setting up application (profile: {})", &PROFILE);
  glib::set_application_name("Amberol");
  glib::set_program_name(Some("amberol"));

  // Initialize GStreamer
  gst::init().expect("Failed to initialize GStreamer");

  // Create the main GTK context and run the application
  let ctx = glib::MainContext::default();
  let _guard = ctx.acquire().unwrap();

  Application::new().run()
}
