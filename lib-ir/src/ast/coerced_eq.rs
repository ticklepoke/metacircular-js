pub trait CoercedEq<Rhs = Self> {
	fn coerced_eq(&self, other: &Rhs) -> bool;
    
	fn coerced_neq(&self, other: &Rhs) -> bool {
		!self.coerced_eq(other)
	}
}