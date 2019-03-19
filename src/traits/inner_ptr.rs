pub trait InnerPtr<T> {
    fn inner_ptr(&self) -> *const T;
    fn inner_ptr_mut(&mut self) -> *mut T;
}
