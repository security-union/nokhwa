use core::ops::AddAssign;

pub fn min_max_range<N: Copy + PartialOrd + AddAssign<N> + Sized>(
    min: N,
    max: N,
    step: N,
) -> Vec<N> {
    let mut counter = min;
    let mut nums = vec![min];

    loop {
        counter += step;

        if counter > max {
            break;
        }

        nums.push(counter);
    }

    nums
}

pub trait Distance<T>
where
    T: PartialEq,
{
    fn distance_from(&self, other: &Self) -> T;
}
