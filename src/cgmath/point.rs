// Copyright 2013 The CGMath Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Points are fixed positions in affine space with no length or direction. This
//! disinguishes them from vectors, which have a length and direction, but do
//! not have a fixed position.

use std::fmt;
use std::num::{one, zero};

use array::*;
use vector::*;
use partial_ord::PartOrdPrim;

/// A point in 2-dimensional space.
#[deriving(Eq, Clone, Hash)]
pub struct Point2<S> { pub x: S, pub y: S }

/// A point in 3-dimensional space.
#[deriving(Eq, Clone, Hash)]
pub struct Point3<S> { pub x: S, pub y: S, pub z: S }


impl<S: Num> Point2<S> {
    #[inline]
    pub fn new(x: S, y: S) -> Point2<S> {
        Point2 { x: x, y: y }
    }
}

impl<S: Num> Point3<S> {
    #[inline]
    pub fn new(x: S, y: S, z: S) -> Point3<S> {
        Point3 { x: x, y: y, z: z }
    }
}

impl<S: PartOrdPrim> Point3<S> {
    #[inline]
    pub fn from_homogeneous(v: &Vector4<S>) -> Point3<S> {
        let e = v.truncate().mul_s(one::<S>() / v.w);
        Point3::new(e.x.clone(), e.y.clone(), e.z.clone())  //FIXME
    }

    #[inline]
    pub fn to_homogeneous(&self) -> Vector4<S> {
        Vector4::new(self.x.clone(), self.y.clone(), self.z.clone(), one())
    }
}

/// Specifies the numeric operations for point types.
pub trait Point
<
    S: PartOrdPrim,
    V: Vector<S, Slice>,
    Slice
>
:   Array<S, Slice>
{
    /// Create a point at the origin.
    #[inline] fn origin() -> Self{ build(|_i| zero::<S>()) }

    /// Create a point from a vector.
    #[inline] fn from_vec(v: &V) -> Self { build(|i| v.i(i).clone()) }
    /// Convert a point to a vector.
    #[inline] fn to_vec(&self) -> V { build(|i| self.i(i).clone()) }

    /// Multiply each component by a scalar, returning the new point.
    #[inline] fn mul_s(&self, s: S) -> Self { build(|i| self.i(i).mul(&s)) }
    /// Divide each component by a scalar, returning the new point.
    #[inline] fn div_s(&self, s: S) -> Self { build(|i| self.i(i).div(&s)) }
    /// Subtract a scalar from each component, returning the new point.
    #[inline] fn rem_s(&self, s: S) -> Self { build(|i| self.i(i).rem(&s)) }

    /// Add a vector to this point, returning the new point.
    #[inline] fn add_v(&self, other: &V) -> Self { build(|i| self.i(i).add(other.i(i))) }
    /// Subtract another point from this one, returning a new vector.
    #[inline] fn sub_p(&self, other: &Self) -> V { build(|i| self.i(i).sub(other.i(i))) }

    /// Multiply each component by a scalar, in-place.
    #[inline] fn mul_self_s(&mut self, s: S) { self.each_mut(|_, x| *x = x.mul(&s)) }
    /// Divide each component by a scalar, in-place.
    #[inline] fn div_self_s(&mut self, s: S) { self.each_mut(|_, x| *x = x.div(&s)) }
    /// Take the remainder of each component by a scalar, in-place.
    #[inline] fn rem_self_s(&mut self, s: S) { self.each_mut(|_, x| *x = x.rem(&s)) }

    /// Add a vector to this point, in-place.
    #[inline] fn add_self_v(&mut self, other: &V) { self.each_mut(|i, x| *x = x.add(other.i(i))) }

    /// This is a weird one, but its useful for plane calculations.
    #[inline]
    fn dot(&self, v: &V) -> S {
        build::<S, Slice, V>(|i| self.i(i).mul(v.i(i))).comp_add()
    }
}

array!(impl<S> Point2<S> -> [S, ..2] _2)
array!(impl<S> Point3<S> -> [S, ..3] _3)

impl<S: PartOrdPrim> Point<S, Vector2<S>, [S, ..2]> for Point2<S> {}
impl<S: PartOrdPrim> Point<S, Vector3<S>, [S, ..3]> for Point3<S> {}

impl<S: fmt::Show> fmt::Show for Point2<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl<S: fmt::Show> fmt::Show for Point3<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}
