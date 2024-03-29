//===================================================================================================================================================================================//
//
//  /$$$$$$$                        /$$            /$$$$$$  /$$$$$$$ 
// | $$__  $$                      | $$           /$$__  $$| $$__  $$
// | $$  \ $$  /$$$$$$   /$$$$$$$ /$$$$$$        |__/  \ $$| $$  \ $$
// | $$$$$$$/ /$$__  $$ /$$_____/|_  $$_/          /$$$$$$/| $$  | $$
// | $$__  $$| $$$$$$$$| $$        | $$           /$$____/ | $$  | $$
// | $$  \ $$| $$_____/| $$        | $$ /$$      | $$      | $$  | $$
// | $$  | $$|  $$$$$$$|  $$$$$$$  |  $$$$/      | $$$$$$$$| $$$$$$$/
// |__/  |__/ \_______/ \_______/   \___/        |________/|_______/
//
//===================================================================================================================================================================================//

//?
//? Created by LunaticWyrm467 and others.
//? 
//? All code is licensed under the MIT license.
//? Feel free to reproduce, modify, and do whatever.
//?

//!
//! A private submodule for the rect module that contains all of the implementations
//! for any of the non-shared behaviours of the 2D rect.
//!

use super::*;
use crate::vector::{ Vec2, Axis2 };


/*
    2D Rect
        Implementation
*/


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side2 {
    Top,
    Bottom,
    Left,
    Right
}

/// A 2D Rectangle with a position and size.
/// Contains common geometric and bounding box methods.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct Rect2<T: Scalar>(pub Vec2<T>, pub Vec2<T>);

impl <T: Scalar> RectAbstract<T, Vec2<T>, Rect2<T>> for Rect2<T> {}

impl <T: Scalar> Rect<T, Vec2<T>, Rect2<T>, Axis2, Side2> for Rect2<T> {
    fn new(position: Vec2<T>, size: Vec2<T>) -> Rect2<T> {
        Rect2(position, size)
    }

    fn encompass_points(points: &Vec<Vec2<T>>) -> Rect2<T> {
        let top:    T = points.iter().map(|p| p.y()).reduce(T::min).unwrap();
        let bottom: T = points.iter().map(|p| p.y()).reduce(T::max).unwrap();
        let left:   T = points.iter().map(|p| p.x()).reduce(T::min).unwrap();
        let right:  T = points.iter().map(|p| p.x()).reduce(T::max).unwrap();

        Rect2::from_offsets(left, top, right, bottom)
    }

    fn identity(&self) -> &Rect2<T> {
        self
    }

    fn position(&self) -> &Vec2<T> {
        &self.0
    }

    fn position_mut(&mut self) -> &mut Vec2<T> {
        &mut self.0
    }

    fn size(&self) -> &Vec2<T> {
        &self.1
    }

    fn size_mut(&mut self) -> &mut Vec2<T> {
        &mut self.1
    }

    fn set_position(&mut self, position: Vec2<T>) {
        self.0 = position;
    }

    fn set_size(&mut self, size: Vec2<T>) {
        self.1 = size;
    }

    fn vertex(&self, idx: usize) -> Vec2<T> {
        match idx {
            0 => self.position().to_owned(),
            1 => self.position() + &self.size().of_x(),
            2 => self.position() + self.size(),
            3 => self.position() + &self.size().of_y(),
            _ => panic!("Vertex index out of bounds.")
        }
    }

    fn longest_axis(&self) -> Axis2 {
        if self.size().y() > self.size().x() {
            return Axis2::Y;
        }
        Axis2::X
    }

    fn shortest_axis(&self) -> Axis2 {
        if self.size().y() < self.size().x() {
            return Axis2::Y;
        }
        Axis2::X
    }

    fn axis_length(&self, axis: Axis2) -> T {
        match axis {
            Axis2::X    => self.size().x(),
            Axis2::Y    => self.size().y(),
            Axis2::None => panic!("Axis cannot be None.")
        }
    }

    fn expand_to_include(&self, point: Vec2<T>) -> Rect2<T> {
        
        // Break down this rectangle into a simple "origin" and "end" pair of vectors.
        let mut origin: Vec2<T> = self.position().to_owned();
        let mut end:    Vec2<T> = self.end().to_owned();

        // Check each component of the origin and end and compare it to that of the given point.
        // If a component of a vector is out of bounds, then update the respective component of either the origin or end.
        if point.x() < origin.x() {
            origin.set_x(point.x());
        }
        if point.y() < origin.y() {
            origin.set_y(point.y());
        }

        if point.x() > end.x() {
            end.set_x(point.x());
        }
        if point.y() > end.y() {
            end.set_y(point.y());
        }

        Rect2(origin, end)
    }

    fn grow_side(&self, side: Side2, amount: T) -> Rect2<T> {
        match side {
            Side2::Top    => Rect2(self.0.to_owned() - Vec2::on_y(amount), self.1.to_owned() + Vec2::on_y(amount)),
            Side2::Bottom => Rect2(self.0.to_owned(), self.1.to_owned() + Vec2::on_y(amount)),
            Side2::Left   => Rect2(self.0.to_owned() - Vec2::on_x(amount), self.1.to_owned() + Vec2::on_x(amount)),
            Side2::Right  => Rect2(self.0.to_owned(), self.1.to_owned() + Vec2::on_x(amount))
        }
    }

    fn intersects(&self, other: &Rect2<T>, including_borders: bool) -> bool {
        if including_borders {
            if self.position().x() > other.position().x() + other.size().x() {
                return false;
            }
            if self.position().x() + self.size().x() < other.position().x() {
                return false;
            }
            if self.position().y() > other.position().y() + other.size().y() {
                return false;
            }
            if self.position().y() + self.size().y() < other.position().y() {
                return false;
            }
        } else {
            if self.position().x() >= other.position().x() + other.size().x() {
                return false;
            }
            if self.position().x() + self.size().x() <= other.position().x() {
                return false;
            }
            if self.position().y() >= other.position().y() + other.size().y() {
                return false;
            }
            if self.position().y() + self.size().y() <= other.position().y() {
                return false;
            }
        }

        true
    }
}

impl <T: SignedScalar> SignedRect<T, Vec2<T>, Rect2<T>, Axis2, Side2> for Rect2<T> {}

impl <T: FloatScalar> FloatRect<T, Vec2<T>, Rect2<T>, Axis2, Side2> for Rect2<T> {}

impl <T: Scalar> Rect2<T> {

    /// Creates a new Rect2 whose sides are offset from the origin by the given amounts.
    pub fn from_offsets(left: T, top: T, right: T, bottom: T) -> Rect2<T> {
        Rect2(Vec2(left, top), Vec2(right - left, bottom - top))
    }

    /// Creates a new Rect2 from the given origin's x and y components and the given width and height.
    pub fn from_components(x: T, y: T, width: T, height: T) -> Rect2<T> {
        Rect2(Vec2(x, y), Vec2(width, height))
    }

    /// Converts a `Rect2` to a `Rect2` of a different type.
    pub fn cast<U: Scalar>(&self) -> Rect2<U> {
        Rect2(self.0.cast(), self.1.cast())
    }

    /// Returns the x component of the origin of the `Rect2`.
    pub fn x(&self) -> T {
        self.0.x()
    }

    /// Returns the y component of the origin of the `Rect2`.
    pub fn y(&self) -> T {
        self.0.y()
    }

    /// Returns the width of the `Rect2`.
    pub fn width(&self) -> T {
        self.1.x()
    }

    /// Returns the height of the `Rect2`.
    pub fn height(&self) -> T {
        self.1.y()
    }
}


/*
    Global
        Behaviours
*/


impl <T: Scalar> Default for Rect2<T> {
    fn default() -> Self {
        Rect2(Vec2::default(), Vec2::default())
    }
}

impl <T: Scalar> std::fmt::Display for Rect2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rect2({}, {})", self.0, self.1)
    }
}