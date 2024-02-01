#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Cell {
    #[default]
    Closed,
    Yes,
    No,
}

impl From<bool> for Cell {
    fn from(value: bool) -> Self {
        if value {
            Cell::Yes
        } else {
            Cell::No
        }
    }
}

impl From<Cell> for bool {
    fn from(value: Cell) -> Self {
        value == Cell::Yes
    }
}

/*******************************************************************************
 * Tests
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cells_should_be_closed_by_default() {
        assert_eq!(Cell::default(), Cell::Closed);
    }

    #[test]
    fn closed_cells_should_be_false() {
        let cell: bool = Cell::Closed.into();
        assert!(!cell);
    }

    #[test]
    fn yes_cells_should_be_true() {
        let cell: bool = Cell::Yes.into();
        assert!(cell);
    }

    #[test]
    fn no_cells_should_be_false() {
        let cell: bool = Cell::No.into();
        assert!(!cell);
    }

    #[test]
    fn true_should_be_translated_to_yes() {
        let cell: Cell = true.into();
        assert_eq!(cell, Cell::Yes);
    }

    #[test]
    fn false_should_be_translated_to_no() {
        let cell: Cell = false.into();
        assert_eq!(cell, Cell::No);
    }

    #[test]
    fn false_should_not_be_translated_to_closed() {
        let cell: Cell = false.into();
        assert_ne!(cell, Cell::Closed);
    }
}
