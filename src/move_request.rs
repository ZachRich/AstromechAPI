pub mod move_request {
    use serde::Deserialize;

    #[derive(Deserialize, Clone)]
    pub struct MoveRequest {
        pub channel: u8,
        pub position: String,
    }
}