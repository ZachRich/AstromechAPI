pub mod move_request {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct MoveRequest {
        pub channel: u8,
        pub angle: u16, // angle in degrees, from 0 to 180
    }
}
