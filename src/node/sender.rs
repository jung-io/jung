
pub struct Sender<T> {
    sender: crossbeam::Sender<T>,
}
