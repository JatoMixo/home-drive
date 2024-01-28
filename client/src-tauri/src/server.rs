#[derive(serde::Deserialize)]
pub struct Server {
    ip: String,
    port: u16,
}

impl Server {
    fn get_server_direction(&self) -> String {
        format!("http://{}:{}", self.ip, self.port)
    }

    pub fn get_upload_route(&self) -> String {
        format!("{}/upload", self.get_server_direction())
    }

    pub fn get_explorer_route_for_path(&self, path: &str) -> String {
        format!("{}/explorer/{}", self.get_server_direction(), path)
    }
}
