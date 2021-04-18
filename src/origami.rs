use rabbit_ear::Vector;
use rabbit_ear::Line;

pub mod gridvec;
// pub mod quadtree;
pub mod linecontainer;
pub mod make;
pub mod draw;

// tuple pairs for point and line with a second parameter
// which keeps track of how many repeated occurences there are
pub type CountPoint = (Vector, u64);
pub type CountLine = (Line, u64);

pub use self::make::make_intersections;
pub use self::make::make_axiom1;
pub use self::make::make_axiom2;
pub use self::make::make_axiom3;
pub use self::make::make_axiom4;
pub use self::make::make_axiom5;
pub use self::make::make_axiom6;
pub use self::make::make_axiom7;

// pub use self::quadtree::QuadTree;
// pub use self::quadtree::make_tree;
pub use self::gridvec::GridVec;
pub use self::gridvec::make_grid;
pub use self::linecontainer::LineContainer;
