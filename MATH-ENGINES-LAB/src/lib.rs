//! Experimental, pure math kernel. No Host authority, registry or graph storage.
//! No implicit conversion between exact and approximate domains.
#![forbid(unsafe_code)]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MathError {
    Overflow,
    DivisionByZero,
    NonFinite,
    Domain,
    ShapeMismatch,
    EmptyInput,
    BudgetExceeded,
    Unbound,
    BindingMismatch,
    StaleBinding,
}

pub type MathResult<T> = Result<T, MathError>;

/// Checked scalar operations. Implementors must document their arithmetic domain;
/// implementing this trait does not certify mathematical laws or error bounds.
pub trait Scalar: Copy {
    fn zero() -> Self;
    fn add(self, rhs: Self) -> MathResult<Self>;
    fn mul(self, rhs: Self) -> MathResult<Self>;
}

impl Scalar for i128 {
    fn zero() -> Self { 0 }
    fn add(self, rhs: Self) -> MathResult<Self> {
        self.checked_add(rhs).ok_or(MathError::Overflow)
    }
    fn mul(self, rhs: Self) -> MathResult<Self> {
        self.checked_mul(rhs).ok_or(MathError::Overflow)
    }
}

/// Reduced rational with a strictly positive denominator. This bounded reference
/// backend may reject an operation when an intermediate i128 overflows, even
/// when the mathematical result would fit. No silent fallback to floating point.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rational { numerator: i128, denominator: i128 }

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 { let r = a % b; a = b; b = r; }
    a
}

impl Rational {
    pub fn new(n: i128, d: i128) -> MathResult<Self> {
        if d == 0 { return Err(MathError::DivisionByZero); }
        if n == 0 { return Ok(Self { numerator: 0, denominator: 1 }); }
        let g = gcd(n.unsigned_abs(), d.unsigned_abs());
        // Only MIN/MIN has a gcd of 2^127 for nonzero signed inputs.
        if g > i128::MAX as u128 {
            return Ok(Self { numerator: 1, denominator: 1 });
        }
        let g = g as i128;
        let (mut n, mut d) = (n / g, d / g);
        if d < 0 {
            n = n.checked_neg().ok_or(MathError::Overflow)?;
            d = d.checked_neg().ok_or(MathError::Overflow)?;
        }
        Ok(Self { numerator: n, denominator: d })
    }
    pub fn numerator(self) -> i128 { self.numerator }
    pub fn denominator(self) -> i128 { self.denominator }
    pub fn checked_div(self, rhs: Self) -> MathResult<Self> {
        if rhs.numerator == 0 { return Err(MathError::DivisionByZero); }
        if self.numerator == 0 { return Ok(Self::zero()); }
        // Cross-reduce before multiplication, without constructing a reciprocal
        // whose denominator might not fit the representation.
        let a = gcd(self.numerator.unsigned_abs(), rhs.numerator.unsigned_abs());
        let b = gcd(self.denominator as u128, rhs.denominator as u128) as i128;
        let (n1, n2) = if a > i128::MAX as u128 { (-1, -1) }
            else { (self.numerator / a as i128, rhs.numerator / a as i128) };
        let n = n1.checked_mul(rhs.denominator / b).ok_or(MathError::Overflow)?;
        let d = n2.checked_mul(self.denominator / b).ok_or(MathError::Overflow)?;
        Self::new(n, d)
    }
}

impl Scalar for Rational {
    fn zero() -> Self { Self { numerator: 0, denominator: 1 } }
    fn add(self, rhs: Self) -> MathResult<Self> {
        let g = gcd(self.denominator as u128, rhs.denominator as u128) as i128;
        let l = self.denominator / g;
        let r = rhs.denominator / g;
        let n = self.numerator.checked_mul(r).ok_or(MathError::Overflow)?
            .checked_add(rhs.numerator.checked_mul(l).ok_or(MathError::Overflow)?)
            .ok_or(MathError::Overflow)?;
        if n == 0 { return Ok(Self::zero()); }
        let h = gcd(n.unsigned_abs(), g as u128) as i128;
        let d = l.checked_mul(rhs.denominator / h).ok_or(MathError::Overflow)?;
        Self::new(n / h, d)
    }
    fn mul(self, rhs: Self) -> MathResult<Self> {
        let a = gcd(self.numerator.unsigned_abs(), rhs.denominator as u128) as i128;
        let b = gcd(rhs.numerator.unsigned_abs(), self.denominator as u128) as i128;
        Self::new(
            (self.numerator / a).checked_mul(rhs.numerator / b).ok_or(MathError::Overflow)?,
            (self.denominator / b).checked_mul(rhs.denominator / a).ok_or(MathError::Overflow)?,
        )
    }
}

/// Finite binary64, NOT exact or certified. Underflow/rounding may occur;
/// signed zero is normalized for consistent equality. No NaN/Inf accepted.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FiniteF64(f64);
impl FiniteF64 {
    pub fn new(x: f64) -> MathResult<Self> {
        if !x.is_finite() { return Err(MathError::NonFinite); }
        Ok(Self(if x == 0.0 { 0.0 } else { x }))
    }
    pub fn get(self) -> f64 { self.0 }
    pub fn checked_div(self, rhs: Self) -> MathResult<Self> {
        if rhs.0 == 0.0 { return Err(MathError::DivisionByZero); }
        Self::new(self.0 / rhs.0)
    }
}
impl Scalar for FiniteF64 {
    fn zero() -> Self { Self(0.0) }
    fn add(self, rhs: Self) -> MathResult<Self> { Self::new(self.0 + rhs.0) }
    fn mul(self, rhs: Self) -> MathResult<Self> { Self::new(self.0 * rhs.0) }
}

/// A bounded borrowed vector, independent of storage/backend.
#[derive(Clone, Copy, Debug)]
pub struct Vector<'a, T> { values: &'a [T] }
impl<'a, T> Vector<'a, T> {
    pub fn new(values: &'a [T], max_elements: usize) -> MathResult<Self> {
        if values.len() > max_elements { return Err(MathError::BudgetExceeded); }
        Ok(Self { values })
    }
    pub fn as_slice(&self) -> &'a [T] { self.values }
}

/// Validate shape and allocation budget before accessing a dense buffer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Shape2 { rows: usize, cols: usize, elements: usize }
impl Shape2 {
    pub fn new(rows: usize, cols: usize, max_elements: usize) -> MathResult<Self> {
        let elements = rows.checked_mul(cols).ok_or(MathError::Overflow)?;
        if elements > max_elements { return Err(MathError::BudgetExceeded); }
        Ok(Self { rows, cols, elements })
    }
    pub fn rows(self) -> usize { self.rows }
    pub fn cols(self) -> usize { self.cols }
    pub fn elements(self) -> usize { self.elements }
    pub fn validate_len(self, len: usize) -> MathResult<()> {
        if len != self.elements { return Err(MathError::ShapeMismatch); }
        Ok(())
    }
}

/// Typed operations compose directly; no dynamic registry or serialization.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Capability { pub id: &'static str, pub revision: u32 }

pub trait Operation<I> {
    type Output;
    fn capability(&self) -> Capability;
    fn evaluate(&self, input: I) -> MathResult<Self::Output>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Sum;
impl<'a, T: Scalar> Operation<Vector<'a, T>> for Sum {
    type Output = T;
    fn capability(&self) -> Capability { Capability { id: "math.vector.sum", revision: 1 } }
    fn evaluate(&self, input: Vector<'a, T>) -> MathResult<T> {
        input.values.iter().try_fold(T::zero(), |acc, &x| acc.add(x))
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Dot;
impl<'a, 'b, T: Scalar> Operation<(Vector<'a, T>, Vector<'b, T>)> for Dot {
    type Output = T;
    fn capability(&self) -> Capability { Capability { id: "math.vector.dot", revision: 1 } }
    fn evaluate(&self, (a, b): (Vector<'a, T>, Vector<'b, T>)) -> MathResult<T> {
        if a.values.len() != b.values.len() { return Err(MathError::ShapeMismatch); }
        a.values.iter().zip(b.values).try_fold(T::zero(), |acc, (&x, &y)| acc.add(x.mul(y)?))
    }
}

/// Reference mean, not a numerically stable or certified reduction.
#[derive(Clone, Copy, Debug, Default)]
pub struct Mean;
impl<'a> Operation<Vector<'a, FiniteF64>> for Mean {
    type Output = FiniteF64;
    fn capability(&self) -> Capability { Capability { id: "math.statistics.mean.reference", revision: 1 } }
    fn evaluate(&self, input: Vector<'a, FiniteF64>) -> MathResult<FiniteF64> {
        let len = input.values.len();
        if len == 0 { return Err(MathError::EmptyInput); }
        if len as u128 > (1u128 << 53) { return Err(MathError::BudgetExceeded); }
        Sum.evaluate(input)?.checked_div(FiniteF64::new(len as f64)?)
    }
}

/// Compose any two compatible operations. Graph lowering is intentionally left to
/// the incumbent host; this is a local typed call, not a second graph.
pub fn compose<I, A, B>(first: &A, second: &B, input: I) -> MathResult<B::Output>
where A: Operation<I>, B: Operation<A::Output> {
    second.evaluate(first.evaluate(input)?)
}

pub mod adapter {
    use super::{Capability, MathError, MathResult, Operation};
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Target { Product, Arbor }
    /// Candidate mapping from a caller-owned active host snapshot.
    /// A binding is not a grant, execution receipt or registration.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Binding {
        pub target: Target,
        pub capability: Capability,
        pub host_key: String,
        pub generation: u64,
    }
    /// Product/ARBOR implement this over their existing registry when admitted.
    /// Labs never owns the mapping or persists it. Unknown capabilities stay unbound.
    pub trait Resolver {
        fn resolve(&self, target: Target, capability: Capability) -> MathResult<Binding>;
    }
    /// Re-resolve on each invocation to reject stale or mismatched bindings.
    /// Still a pure local kernel evaluation; no remote call or effect.
    pub fn invoke<I, O: Operation<I>, R: Resolver>(
        resolver: &R, expected: &Binding, operation: &O, input: I,
    ) -> MathResult<O::Output> {
        let capability = operation.capability();
        if expected.capability != capability || expected.host_key.is_empty() || expected.generation == 0 {
            return Err(MathError::BindingMismatch);
        }
        let current = resolver.resolve(expected.target, capability)?;
        if current.generation != expected.generation { return Err(MathError::StaleBinding); }
        if &current != expected { return Err(MathError::BindingMismatch); }
        operation.evaluate(input)
    }
}
