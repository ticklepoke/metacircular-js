pub trait Additive<Rhs = Self> {
    fn add(&self, other: &Rhs) -> Rhs;

    fn sub(&self, other: &Rhs) -> Rhs;
}

pub trait Multiplicative<Rhs = Self> {
    fn mul(&self, other: &Rhs) -> Rhs;

    fn div(&self, other: &Rhs) -> Rhs;

    fn modulo(&self, other: &Rhs) -> Rhs;
}

pub trait BitwiseBinary<Rhs = Self> {
    fn bitwise_and(&self, other: &Rhs) -> Rhs;

    fn bitwise_or(&self, other: &Rhs) -> Rhs;

    fn bitwise_xor(&self, other: &Rhs) -> Rhs;
}

pub trait BitwiseShift<Rhs = Self> {
    fn left_shift(&self, other: &Rhs) -> Rhs;

    fn signed_right_shift(&self, other: &Rhs) -> Rhs;

    fn unsigned_right_shift(&self, other: &Rhs) -> Rhs;
}
