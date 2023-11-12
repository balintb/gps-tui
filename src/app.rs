use std::error;
use unbounded_gpsd::{types::TpvResponse, *};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
// #[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,

    /// latitude
    pub lat: f64,

    /// logintude
    pub lon: f64,

    /// gps connection
    pub gps_connection: GpsdConnection,
}

impl Default for App {
    fn default() -> Self {
        let mut conn = GpsdConnection::new("127.0.0.1:2947").unwrap();
        conn.watch(true).unwrap();

        Self {
            running: true,
            lat: 0.0,
            lon: 0.0,
            gps_connection: conn,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        // read stream
        let resp = self.gps_connection.get_response();
        match resp {
            Ok(response) => match response {
                unbounded_gpsd::types::Response::Tpv(tpv) => match tpv {
                    TpvResponse::Fix3D { lat, lon, .. } => {
                        self.update_position(lat, lon);
                    }
                    _ => {}
                },
                _ => {}
            },
            Err(_) => {}
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn update_position(&mut self, lat: f64, lon: f64) {
        self.lat = lat;
        self.lon = lon;
    }
}
