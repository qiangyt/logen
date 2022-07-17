use rand::Rng;

pub fn choose_arr<T>(arr: &Vec<T>) -> &T
where
    T: Sized,
{
    let max = arr.len();
    if arr.len() == 0 {
        panic!("arr should configure at least 1 element");
    }

    let mut k = 0;
    let mut rng = rand::thread_rng();

    while k < 10 {
        let i = rng.gen_range(0..max * 2);
        if i < max {
            return &arr[i];
        }

        k = k + 1;
    }

    return &arr[0];
}
