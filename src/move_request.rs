pub mod move_request {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct MoveRequest {
        pub servo_name: String, // Use servo name instead of channel
        pub angle: u16,         // Angle in degrees
    }
}
