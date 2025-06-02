use jack::{Client, ClientOptions, ClientStatus};

pub struct AudioBuffer {
    client: Client,
    client_status: ClientStatus,
}

impl AudioBuffer {
    pub fn new(name: &str) -> Self {
        let (client, client_status) = Client::new(name, ClientOptions::default()).expect("Error initializing jack client");

        Self { client, client_status }
    }
}
