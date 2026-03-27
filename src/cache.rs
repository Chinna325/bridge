pub struct Cache {}

impl Cache {
    pub fn new() -> Self {
        Self {}
    }
    pub async fn set(&self) -> Result<(), ()> {
        todo!()
    }

    pub async fn get(&self) -> Result<Vec<u8>, ()> {
        todo!()
    }

    pub async fn clear(&self, _key: String) -> Result<(), ()> {
        todo!()
    }
}
