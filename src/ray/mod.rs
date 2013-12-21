//! Ray casting utilities.

// types an traits
pub use ray::ray::{Ray, RayCast, RayCastWithTransform};

// functions
pub use ray::ray_plane::plane_toi_with_ray;
pub use ray::ray_implicit::gjk_toi_and_normal_with_ray;
pub use ray::ray_subsimplex_mesh::triangle_toi_and_normal_with_ray;
pub use ray::ray_subsimplex_mesh::segment_toi_and_normal_with_ray;

// modules
pub mod ray;
#[doc(hidden)]
pub mod ray_plane;
#[doc(hidden)]
pub mod ray_ball;
#[doc(hidden)]
pub mod ray_box;
#[doc(hidden)]
pub mod ray_aabb;
#[doc(hidden)]
pub mod ray_compound;
#[doc(hidden)]
pub mod ray_implicit;
#[doc(hidden)]
pub mod ray_subsimplex_mesh;
