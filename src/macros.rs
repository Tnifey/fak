macro_rules! from_range {
    ($x:expr) => {
        rand::thread_rng().gen_range($x)
    }
}
