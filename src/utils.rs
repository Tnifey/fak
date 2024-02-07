macro_rules! from_range {
    ($x:expr) => {
        rand::thread_rng().gen_range($x)
    }
}

macro_rules! probability {
    () => {
        rand::thread_rng().gen_bool(0.5)
    };

    ($x:expr) => {
        rand::thread_rng().gen_bool($x)
    };
}

pub fn code_point(c: char) -> Option<u16> {
    let mut b = [0; 2];
    return c.encode_utf16(&mut b).first().copied();
}
