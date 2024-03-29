use approx::{ RelativeEq, AbsDiffEq };
use num_traits::{ Num, Signed, Float, FloatConst, PrimInt, NumCast };


/*
    Trait
        Definitions
*/


/// Implements common behaviours and additional operations for all primitives.
pub trait Scalar: Clone + Copy + Num + Default + PartialOrd + std::fmt::Display + std::fmt::Debug + NumCast {

    /// Returns the minimum value of this value and another.
    /// This is implemented manually to not rely on the Ord trait.
    fn min(self, other: Self) -> Self {
        if self < other {
            self
        } else {
            other
        }
    }

    /// Returns the maximum value of this value and another.
    /// This is implemented manually to not rely on the Ord trait.
    fn max(self, other: Self) -> Self {
        if self > other {
            self
        } else {
            other
        }
    }

    /// Clamps this value between a provided minimum and maximum.
    /// This is implemented manually to not rely on the Ord trait.
    fn clamp(self, min: Self, max: Self) -> Self {
        self.min(max).max(min)
    }

    /// A simple linear interpolation between two values.
    /// Samples at the point `t` between `self` and `other`.
    fn lerp(self, other: Self, t: Self) -> Self {
        self + (other - self) * t
    }
}

/// Implements unique integer operations for all integer primitives.
pub trait IntScalar<T: IntScalar<T>>: Scalar + Ord + PrimInt + IntUnique<T> {}

/// Implements unique operations for all signed primitives.
pub trait SignedScalar: Scalar + Signed {

    /// Calculates the derivative of the Bézier curve set by this scalar and the given control and terminal points
    /// at position `t`.
    fn bezier_derivative(self, control_1: Self, control_2: Self, terminal: Self, t: Self) -> Self {

        // Define some commonly used constants.
        let t_3: Self = Self::from(3).unwrap();
        let t_6: Self = Self::from(6).unwrap();

        // Formula from https://en.wikipedia.org/wiki/Bézier_curve
		let omt:  Self = Self::one() - t;
		let omt2: Self = omt * omt;
		let t2:   Self = t * t;

		(control_1 - self) * t_3 * omt2 + (control_2 - control_1) * t_6 * omt * t + (terminal - control_2) * t_3 * t2
    }

    /// Calculates the point on the Bézier curve set by this scalar and the given control and terminal points
    /// at position `t`.
    fn bezier_sample(self, control_1: Self, control_2: Self, terminal: Self, t: Self) -> Self {

        // Define some commonly used constants.
        let t_3: Self = Self::from(3).unwrap();

        // Formula from https://en.wikipedia.org/wiki/Bézier_curve
        let omt:  Self = Self::one() - t;
		let omt2: Self = omt * omt;
		let omt3: Self = omt2 * omt;
		let t2:   Self = t * t;
		let t3:   Self = t2 * t;

		self * omt3 + control_1 * omt2 * t * t_3 + control_2 * omt * t2 * t_3 + terminal * t3
    }

    /// Calculates and samples the cubic interpolation between this scalar and another
    /// given `pre_start` and `post_terminal` scalars as handles, and a given `t` value.
    fn cubic_interpolate(self, terminal: Self, pre_start: Self, post_terminal: Self, t: Self) -> Self {

        // Define some commonly used constants.
        let t_05: Self = Self::from(0.5).unwrap();
        let t_2:  Self = Self::from(2.0).unwrap();
        let t_3:  Self = Self::from(3.0).unwrap();
        let t_4:  Self = Self::from(4.0).unwrap();
        let t_5:  Self = Self::from(5.0).unwrap();
        
        // Derived from https://github.com/godotengine/godot/blob/1952f64b07b2a0d63d5ba66902fd88190b0dcf08/core/math/math_funcs.h#L275
        t_05 * (
            (self * t_2) +
            (-pre_start + terminal) * t +
            (t_2 * pre_start - t_5 * self + t_4 * terminal - post_terminal) * (t * t) +
            (-pre_start + t_3 * self - t_3 * terminal + post_terminal) * (t * t * t)
        )
    }

    /// Similar to `cubic_interpolate`, but it has additional time parameters `terminal_t`, `pre_start_t`, and `post_terminal_t`.
    /// This can be smoother than `cubic_interpolate` in certain instances.
    fn cubic_interpolate_in_time(self, terminal: Self, pre_start: Self, post_terminal: Self, t0: Self, terminal_t: Self, pre_start_t: Self, post_terminal_t: Self) -> Self {

        // Define some commonly used constants.
        let t_0:  Self = Self::zero();
        let t_05: Self = Self::from(0.5).unwrap();
        let t_1:  Self = Self::one();
        
        // Formula of the Barry-Goldman method.
        let t:  Self = t_0.lerp(terminal_t, t0);
        let a1: Self = pre_start.lerp(self, if pre_start_t == t_0 { t_0 } else { (t - pre_start_t) / -pre_start_t });
        let a2: Self = self.lerp(terminal, if terminal_t == t_0 { t_05 } else { t / terminal_t });
        let a3: Self = terminal.lerp(post_terminal, if post_terminal_t - terminal_t == t_0 { t_1 } else { (t - terminal_t) / (post_terminal_t - terminal_t) });
        let b1: Self = a1.lerp(a2, if terminal_t - pre_start_t == t_0 { t_0 } else { (t - pre_start_t) / (terminal_t - pre_start_t) });
        let b2: Self = a2.lerp(a3, if post_terminal_t == t_0 { t_1 } else { t / post_terminal_t });
        b1.lerp(b2, if terminal_t == t_0 { t_05 } else { t / terminal_t })
    }
}

/// Implements unique operations for all floating point primitives.
pub trait FloatScalar: SignedScalar + Float + FloatConst + RelativeEq + AbsDiffEq<Epsilon = Self> {}

/// Adds some additional operations featured in rust that are not available in the standard PrimInt trait for some odd reason.
pub trait IntUnique<T: IntScalar<T>> {
    fn ilog(self, base: T) -> T;
}


/*
    Trait
        Implementations
*/


impl <T: Clone + Copy + Num + Default + PartialOrd + std::fmt::Display + std::fmt::Debug + NumCast> Scalar for T {}
impl <T: Scalar + Ord + PrimInt + IntUnique<T>> IntScalar<T> for T {}
impl <T: Scalar + Signed> SignedScalar for T {}
impl <T: SignedScalar + Float + FloatConst + RelativeEq + AbsDiffEq<Epsilon = Self>> FloatScalar for T {}

impl IntUnique<u8> for u8 {
    fn ilog(self, base: u8) -> u8 {
        self.ilog(base) as u8
    }
}

impl IntUnique<u16> for u16 {
    fn ilog(self, base: u16) -> u16 {
        self.ilog(base) as u16
    }
}

impl IntUnique<u32> for u32 {
    fn ilog(self, base: u32) -> u32 {
        self.ilog(base) as u32
    }
}

impl IntUnique<u64> for u64 {
    fn ilog(self, base: u64) -> u64 {
        self.ilog(base) as u64
    }
}

impl IntUnique<u128> for u128 {
    fn ilog(self, base: u128) -> u128 {
        self.ilog(base) as u128
    }
}

impl IntUnique<usize> for usize {
    fn ilog(self, base: usize) -> usize {
        self.ilog(base) as usize
    }
}

impl IntUnique<isize> for isize {
    fn ilog(self, base: isize) -> isize {
        self.ilog(base) as isize
    }
}

impl IntUnique<i8> for i8 {
    fn ilog(self, base: i8) -> i8 {
        self.ilog(base) as i8
    }
}

impl IntUnique<i16> for i16 {
    fn ilog(self, base: i16) -> i16 {
        self.ilog(base) as i16
    }
}

impl IntUnique<i32> for i32 {
    fn ilog(self, base: i32) -> i32 {
        self.ilog(base) as i32
    }
}

impl IntUnique<i64> for i64 {
    fn ilog(self, base: i64) -> i64 {
        self.ilog(base) as i64
    }
}

impl IntUnique<i128> for i128 {
    fn ilog(self, base: i128) -> i128 {
        self.ilog(base) as i128
    }
}