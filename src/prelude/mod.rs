pub trait Dot {
    type Output;

    fn dot(self, rhs: Self) -> Self::Output;
}

pub trait Cross {
    type Output;

    fn cross(self, rhs: Self) -> Self::Output;
}
