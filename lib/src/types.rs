/// A variant can be defined as a structure which conforms to the Variant trait.
/// This trait requires definition of the Board and MoveGen traits.
/// A variant must be supplied with a name (and also protocol/webui stuff)
pub trait Variant: Board + MoveGen {
    const NAME: String;
}

/// A trait for sides of a board
pub trait Side {}

/// A trait that implements a board state
pub trait Board {
    type Move;

    fn make_move(&mut self, mv: Self::Move);

    fn from_fen(string: String) -> Self;
}

pub trait MoveGen {
    type Move;

    fn gen_quiet(&self) -> Vec<Self::Move>;
    fn gen_noisy(&self) -> Vec<Self::Move>;
}
