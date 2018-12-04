
pub struct Receiver<T> {
    sender: crossbeam::Receiver<T>,
}