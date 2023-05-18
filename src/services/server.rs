pub struct Server {}

impl Server {
    pub fn new() -> Self {
        Server {}
    }
    pub fn get_invite_link(&self) -> String {
        String::from("")
    }
    pub fn join(&self, link: String) -> Result<(), ()> {
        Ok(())
    }
}
