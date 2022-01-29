use std::f32::consts::{FRAC_1_SQRT_2, FRAC_PI_4};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    East,
    Northeast,
    North,
    Northwest,
    West,
    Southwest,
    South,
    Southeast,
}

impl Direction {
    pub const fn angle_to(&self, other: Self) -> Angle {
        Angle::between(*self, other)
    }

    pub const fn rotated_from(start: Self, angle: Angle) -> Self {
        Self::rotated_from_east(Angle::from_octants(
            Angle::between(start, Self::East).octants() + angle.octants(),
        ))
    }

    pub const fn rotated_from_east(angle: Angle) -> Self {
        match angle.octants() {
            0 => Direction::East,
            1 => Direction::Northeast,
            2 => Direction::North,
            3 => Direction::Northwest,
            4 => Direction::West,
            5 => Direction::Southwest,
            6 => Direction::South,
            7 => Direction::Southeast,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Angle {
    // angle in the range [0, 7)
    octants: u8,
}

impl Angle {
    pub const ZERO: Self = Self { octants: 0 };
    pub const FORTY_FIVE: Self = Self { octants: 1 };
    pub const RIGHT: Self = Self { octants: 2 };
    pub const STRAIGHT: Self = Self { octants: 4 };

    pub const fn from_octants(octants: u8) -> Self {
        Self {
            octants: octants % 8,
        }
    }

    pub const fn from_horizontal(direction: Direction) -> Self {
        Self {
            octants: match direction {
                Direction::East => 0,
                Direction::Northeast => 1,
                Direction::North => 2,
                Direction::Northwest => 3,
                Direction::West => 4,
                Direction::Southwest => 5,
                Direction::South => 6,
                Direction::Southeast => 7,
            },
        }
    }

    /// Angle in the clockwise direction between two directions.
    pub const fn between(a: Direction, b: Direction) -> Self {
        Self::from_octants(Self::from_horizontal(b).octants() - Self::from_horizontal(a).octants())
    }

    pub const fn octants(&self) -> u8 {
        self.octants
    }

    pub const fn quadrants(&self) -> u8 {
        self.octants / 2
    }

    pub const fn quadrant(&self) -> u8 {
        self.quadrants() + 1
    }

    pub const fn binary_degrees(&self) -> u8 {
        self.octants * 0x40
    }

    pub const fn degrees(&self) -> u16 {
        self.octants as u16 * 45
    }

    pub const fn radians(&self) -> f32 {
        self.octants as f32 * FRAC_PI_4
    }

    pub const fn sin(&self) -> f32 {
        match self.octants {
            0 => 0.0,
            1 => FRAC_1_SQRT_2,
            2 => 1.0,
            3 => FRAC_1_SQRT_2,
            4 => 0.0,
            5 => -FRAC_1_SQRT_2,
            6 => -1.0,
            7 => -FRAC_1_SQRT_2,
            _ => unreachable!(),
        }
    }

    pub const fn cos(&self) -> f32 {
        match self.octants {
            0 => 1.0,
            1 => FRAC_1_SQRT_2,
            2 => 0.0,
            3 => -FRAC_1_SQRT_2,
            4 => -1.0,
            5 => -FRAC_1_SQRT_2,
            6 => 0.0,
            7 => FRAC_1_SQRT_2,
            _ => unreachable!(),
        }
    }
}

macro_rules! impl_angle_op {
    ($op:ident :: $method:ident) => {
        impl std::ops::$op for Angle {
            type Output = Self;

            fn $method(self, rhs: Self) -> Self {
                Self::from_octants(std::ops::$op::$method(self.octants, rhs.octants))
            }
        }
    };
}

impl_angle_op!(Add::add);
impl_angle_op!(Sub::sub);
impl_angle_op!(Mul::mul);
impl_angle_op!(Div::div);
impl_angle_op!(Rem::rem);

#[test]
pub fn trig_fns() {
    for octant in 0..8 {
        let angle = Angle::from_octants(octant);
        println!("testing angle {:#?}", angle);
        assert!((angle.sin() - angle.radians().sin()).abs() < 0.000001);
        assert!((angle.cos() - angle.radians().cos()).abs() < 0.000001);
    }
}

#[test]
pub fn angle() {}
