use rscenes::prelude::*;
use Direction::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    Forward,
    Backward,
    Left,
    Right,
}

impl Direction {
    pub fn mate(&self, value: u8) -> bool {
        let v: u8 = (*self).into();
        (v & value) > 0
    }

    pub fn compl(&self) -> Self {
        match self {
            Forward => Backward,
            Backward => Forward,
            Left => Right,
            Right => Left,
        }
    }
}

impl From<Direction> for u8 {
    fn from(value: Direction) -> Self {
        match value {
            Forward => 0b0001,
            Backward => 0b0010,
            Left => 0b0100,
            Right => 0b1000,
        }
    }
}

impl From<Direction> for Vector3 {
    fn from(value: Direction) -> Self {
        match value {
            Forward => Vector3::FORTH,
            Backward => Vector3::BACK,
            Left => Vector3::LEFT,
            Right => Vector3::RIGHT,
        }
    }
}

pub fn direction_from_to(x0: i32, y0: i32, xf: i32, yf: i32) -> Option<Direction> {
    if x0 == xf {
        if yf == y0 + 1 {
            return Some(Forward);
        }
        if yf == y0 - 1 {
            return Some(Backward);
        }
    }
    if y0 == yf {
        if xf == x0 + 1 {
            return Some(Right);
        }
        if xf == x0 - 1 {
            return Some(Left);
        }
    }
    None
}

#[macro_export]
macro_rules! direction {
    (from($x0:expr, $y0:expr) to($xf:expr, $yf:expr)) => {
        $crate::maze::direction_from_to($x0, $y0, $xf, $yf)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_head_forth() {
        let dir: Vector3 = Forward.into();
        assert!(dir.eq(Vector3::FORTH));
    }

    #[test]
    fn it_should_head_back() {
        let dir: Vector3 = Backward.into();
        assert!(dir.eq(Vector3::BACK));
    }

    #[test]
    fn it_should_head_left() {
        let dir: Vector3 = Left.into();
        assert!(dir.eq(Vector3::LEFT));
    }

    #[test]
    fn it_should_head_right() {
        let dir: Vector3 = Right.into();
        assert!(dir.eq(Vector3::RIGHT));
    }

    #[test]
    fn it_should_get_u8_from_forward() {
        let dir: u8 = Forward.into();
        assert_eq!(0b0001, dir);
    }

    #[test]
    fn it_should_get_u8_from_backward() {
        let dir: u8 = Backward.into();
        assert_eq!(0b0010, dir);
    }

    #[test]
    fn it_should_get_u8_from_left() {
        let dir: u8 = Left.into();
        assert_eq!(0b0100, dir);
    }

    #[test]
    fn it_should_get_u8_from_right() {
        let dir: u8 = Right.into();
        assert_eq!(0b1000, dir);
    }

    #[test]
    fn compl_of_forward_should_be_backward() {
        assert_eq!(Backward, Forward.compl());
    }

    #[test]
    fn compl_of_backward_should_be_forward() {
        assert_eq!(Forward, Backward.compl());
    }

    #[test]
    fn compl_of_left_should_be_right() {
        assert_eq!(Right, Left.compl());
    }

    #[test]
    fn compl_of_right_should_be_left() {
        assert_eq!(Left, Right.compl());
    }

    #[test]
    fn test_forward_mates() {
        assert!(!Forward.mate(0b0000));
        assert!(Forward.mate(0b0001));
        assert!(!Forward.mate(0b0010));
        assert!(Forward.mate(0b0011));
        assert!(!Forward.mate(0b0100));
        assert!(Forward.mate(0b0101));
        assert!(!Forward.mate(0b0110));
        assert!(Forward.mate(0b0111));
        assert!(!Forward.mate(0b1000));
        assert!(Forward.mate(0b1001));
        assert!(!Forward.mate(0b1010));
        assert!(Forward.mate(0b1011));
        assert!(!Forward.mate(0b1100));
        assert!(Forward.mate(0b1101));
        assert!(!Forward.mate(0b1110));
        assert!(Forward.mate(0b1111));
    }

    #[test]
    fn test_backward_mates() {
        assert!(!Backward.mate(0b0000));
        assert!(!Backward.mate(0b0001));
        assert!(Backward.mate(0b0010));
        assert!(Backward.mate(0b0011));
        assert!(!Backward.mate(0b0100));
        assert!(!Backward.mate(0b0101));
        assert!(Backward.mate(0b0110));
        assert!(Backward.mate(0b0111));
        assert!(!Backward.mate(0b1000));
        assert!(!Backward.mate(0b1001));
        assert!(Backward.mate(0b1010));
        assert!(Backward.mate(0b1011));
        assert!(!Backward.mate(0b1100));
        assert!(!Backward.mate(0b1101));
        assert!(Backward.mate(0b1110));
        assert!(Backward.mate(0b1111));
    }

    #[test]
    fn test_left_mates() {
        assert!(!Left.mate(0b0000));
        assert!(!Left.mate(0b0001));
        assert!(!Left.mate(0b0010));
        assert!(!Left.mate(0b0011));
        assert!(Left.mate(0b0100));
        assert!(Left.mate(0b0101));
        assert!(Left.mate(0b0110));
        assert!(Left.mate(0b0111));
        assert!(!Left.mate(0b1000));
        assert!(!Left.mate(0b1001));
        assert!(!Left.mate(0b1010));
        assert!(!Left.mate(0b1011));
        assert!(Left.mate(0b1100));
        assert!(Left.mate(0b1101));
        assert!(Left.mate(0b1110));
        assert!(Left.mate(0b1111));
    }

    #[test]
    fn test_right_mates() {
        assert!(!Right.mate(0b0000));
        assert!(!Right.mate(0b0001));
        assert!(!Right.mate(0b0010));
        assert!(!Right.mate(0b0011));
        assert!(!Right.mate(0b0100));
        assert!(!Right.mate(0b0101));
        assert!(!Right.mate(0b0110));
        assert!(!Right.mate(0b0111));
        assert!(Right.mate(0b1000));
        assert!(Right.mate(0b1001));
        assert!(Right.mate(0b1010));
        assert!(Right.mate(0b1011));
        assert!(Right.mate(0b1100));
        assert!(Right.mate(0b1101));
        assert!(Right.mate(0b1110));
        assert!(Right.mate(0b1111));
    }
}
