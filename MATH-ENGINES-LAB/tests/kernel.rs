use vivim_math_engines_lab::*;
use vivim_math_engines_lab::adapter::*;

#[test]
fn integer_operations_are_checked() {
    assert_eq!(i128::MAX.add(1), Err(MathError::Overflow));
    assert_eq!(i128::MIN.mul(-1), Err(MathError::Overflow));
    assert_eq!(2i128.mul(3), Ok(6));
}

#[test]
fn rational_normalization_and_boundaries() {
    let r = Rational::new(2,-4).unwrap();
    assert_eq!((r.numerator(),r.denominator()),(-1,2));
    assert_eq!(Rational::new(0, i128::MIN), Rational::new(0,1));
    assert_eq!(Rational::new(i128::MIN, i128::MIN), Rational::new(1,1));
    assert_eq!(Rational::new(1, i128::MIN), Err(MathError::Overflow));
    assert_eq!(Rational::new(i128::MIN, -1), Err(MathError::Overflow));
    assert_eq!(Rational::new(1,0), Err(MathError::DivisionByZero));
}

#[test]
fn rational_composition_is_exact() {
    let a = Rational::new(1,3).unwrap();
    let b = Rational::new(1,6).unwrap();
    assert_eq!(a.add(b), Rational::new(1,2));
    assert_eq!(a.mul(b), Rational::new(1,18));
    assert_eq!(a.checked_div(b), Rational::new(2,1));
    assert_eq!(a.checked_div(Rational::zero()), Err(MathError::DivisionByZero));
    let min = Rational::new(i128::MIN,1).unwrap();
    assert_eq!(Rational::zero().checked_div(min), Ok(Rational::zero()));
    assert_eq!(min.checked_div(min), Rational::new(1,1));
    assert_eq!(Rational::new(i128::MAX,2).unwrap().mul(Rational::new(2,i128::MAX).unwrap()), Rational::new(1,1));
}

#[test]
fn exact_rational_small_domain_crosscheck() {
    for n in -8i128..=8 { for d in 1i128..=8 {
        for m in -8i128..=8 { for e in 1i128..=8 {
            let a = Rational::new(n,d).unwrap();
            let b = Rational::new(m,e).unwrap();
            assert_eq!(a.add(b), Rational::new(n*e+m*d,d*e));
            assert_eq!(a.mul(b), Rational::new(n*m,d*e));
            if m != 0 { assert_eq!(a.checked_div(b), Rational::new(n*e,d*m)); }
        }}
    }}
}

#[test]
fn floating_domain_rejects_nonfinite_and_zero_division() {
    assert_eq!(FiniteF64::new(f64::NAN), Err(MathError::NonFinite));
    assert_eq!(FiniteF64::new(f64::INFINITY), Err(MathError::NonFinite));
    assert_eq!(FiniteF64::new(-0.0).unwrap().get().to_bits(), 0);
    let a = FiniteF64::new(f64::MAX).unwrap();
    assert_eq!(a.add(a), Err(MathError::NonFinite));
    assert_eq!(a.checked_div(FiniteF64::zero()), Err(MathError::DivisionByZero));
}

#[test]
fn vectors_and_shapes_validate_before_computation() {
    assert_eq!(Shape2::new(usize::MAX,2,usize::MAX), Err(MathError::Overflow));
    assert_eq!(Shape2::new(2,3,5), Err(MathError::BudgetExceeded));
    assert_eq!(Shape2::new(2,3,6).unwrap().validate_len(5), Err(MathError::ShapeMismatch));
    let a = Vector::new(&[1i128,2,3],3).unwrap();
    let b = Vector::new(&[4i128,5,6],3).unwrap();
    assert_eq!(Sum.evaluate(a), Ok(6));
    assert_eq!(Dot.evaluate((a,b)), Ok(32));
    assert_eq!(Dot.evaluate((a,Vector::new(&[1i128],1).unwrap())), Err(MathError::ShapeMismatch));
    assert!(matches!(Vector::new(&[1,2],1), Err(MathError::BudgetExceeded)));
    assert_eq!(Sum.evaluate(Vector::new(&[] as &[i128],0).unwrap()), Ok(0));
}

#[test]
fn statistics_reuses_scalar_and_sum() {
    let data = [FiniteF64::new(2.0).unwrap(),FiniteF64::new(4.0).unwrap()];
    assert_eq!(Mean.evaluate(Vector::new(&data,2).unwrap()), FiniteF64::new(3.0));
    assert_eq!(Mean.evaluate(Vector::new(&[],0).unwrap()), Err(MathError::EmptyInput));
}

struct Double;
impl Operation<i128> for Double {
    type Output = i128;
    fn capability(&self) -> Capability { Capability { id: "lab.test.double", revision: 1 } }
    fn evaluate(&self, input: i128) -> MathResult<i128> { input.mul(2) }
}
#[test]
fn new_operation_composes_without_kernel_changes() {
    assert_eq!(compose(&Sum,&Double,Vector::new(&[1i128,2,3],3).unwrap()),Ok(12));
}

struct Snapshot(Binding);
impl Resolver for Snapshot {
    fn resolve(&self, target: Target, capability: Capability) -> MathResult<Binding> {
        if self.0.target != target || self.0.capability != capability { return Err(MathError::Unbound); }
        Ok(self.0.clone())
    }
}
#[test]
fn both_targets_use_identical_math_without_shared_registry() {
    for target in [Target::Product,Target::Arbor] {
        let b = Binding { target, capability: Double.capability(), host_key: "candidate-key".into(), generation: 1 };
        assert_eq!(invoke(&Snapshot(b.clone()),&b,&Double,5),Ok(10));
        let mut next=b.clone(); next.generation=2;
        assert_eq!(invoke(&Snapshot(next),&b,&Double,5),Err(MathError::StaleBinding));
        let mut wrong=b.clone(); wrong.host_key="different-key".into();
        assert_eq!(invoke(&Snapshot(wrong),&b,&Double,5),Err(MathError::BindingMismatch));
    }
}
