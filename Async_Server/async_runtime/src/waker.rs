use std::task::{RawWaker, RawWakerVTable};

static VTABLE: RawWakerVTable = RawWakerVTable::new(
    my_clone,
    my_wake,
    my_wake_by_ref,
    my_drop,
);

unsafe fn my_clone(raw_waker: *const ()) -> RawWaker {
    RawWaker::new(raw_waker, &VTABLE)
}

// Our wake and wake_by_ref functions are called when the future should be polled
// again because the future that is being waited on is ready.
unsafe fn my_wake(raw_waker: *const ()) {
    drop(Box::from_raw(raw_waker as *mut u32));
}
unsafe fn my_wake_by_ref(_raw_waker: *const ()) {
}

// When our task has finished or has been canceled, we no longer need to poll our task,
// and our waker is dropped.
unsafe fn my_drop(raw_waker: *const ()) {
    drop(Box::from_raw(raw_waker as *mut u32));
}

pub fn create_raw_waker() -> RawWaker {
    let data = Box::into_raw(Box::new(42u32));
    RawWaker::new(data as *const (), &VTABLE)
}

