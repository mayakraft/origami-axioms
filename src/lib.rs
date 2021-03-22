pub mod math;
pub mod axioms;
pub mod fold;
pub mod draw;

pub use math::Vector;
pub use math::Line;
pub use math::Segment;
pub use math::Rect;

pub use axioms::axiom1;
pub use axioms::axiom2;
pub use axioms::axiom3;
pub use axioms::axiom4;
pub use axioms::axiom5;
pub use axioms::axiom6;
pub use axioms::axiom7;

pub use fold::quadtree::QuadTree;
pub use fold::quadtree::make_tree;
pub use fold::linecontainer::LineContainer;
pub use fold::linecontainer::make_line_container;

