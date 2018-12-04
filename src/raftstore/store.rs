
pub struct Store<T>{
    store_id: u64,
}

impl<T: Transport> Store<T>{

}

impl<T> Store<T> {
    pub fn store_id(&self)-> u64 {
        self.store_id
    }
}