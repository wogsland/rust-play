fn main() {
    let x = std::sync::Mutex::new(1);
    let t1 = std::thread::scoped(|| {
        *x.lock().unwrap() += 8;
    });
    let t2 = std::thread::scoped(|| {
        *x.lock().unwrap() += 41;
    });
    t1.join();
    t2.join();
    assert_eq!(*x.lock().unwrap(), 50);
}
