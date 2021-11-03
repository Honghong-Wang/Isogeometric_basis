/**
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2021.11.02
 *
 * Copyright (c) 2021 Luca Carlon. All rights reserved.
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 3 of the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 * 
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
 */

use array2d::Array2D;
use super::size::Size;

#[derive(Debug)]
pub struct Matrix2 {
    data: Array2D<f64>
}

impl PartialEq for Matrix2 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl Matrix2 {
    ///
    /// Constructs a matrix from data. Ownership is transferred.
    /// 
    pub fn from_array(data: Array2D<f64>) -> Matrix2 {
        Matrix2 { data: data }
    }

    ///
    /// Constructs a matrix from data. Ownership is not transferred and
    /// the array is copied.
    /// 
    pub fn from_array_ref(data: &Array2D<f64>) -> Matrix2 {
        Matrix2 { data: data.clone() }
    }

    ///
    /// Builds a matrix from an array of vectors.
    /// 
    pub fn from_vec(data: &[Vec<f64>]) -> Matrix2 {
        Matrix2 { data: Array2D::from_rows(data) }
    }

    ///
    /// Returns the value.
    /// 
    pub fn value(&self, row: usize, col: usize) -> f64 {
        self.data[(row, col)]
    }

    ///
    /// Sets a value.
    /// 
    pub fn set_value(&mut self, row: usize, col: usize, value: f64) {
        self.data[(row, col)] = value
    }

    ///
    /// Returns the number of rows.
    /// 
    pub fn rows(&self) -> usize {
        self.data.num_rows()
    }

    ///
    /// Returns the number of columns.
    /// 
    pub fn cols(&self) -> usize {
        self.data.num_columns()
    }

    ///
    /// Returns the size of the matrix.
    /// 
    pub fn size(&self) -> Size {
        Size {
            width: self.cols(),
            height: self.rows()
        }
    }

    ///
    /// Adds two matrices.
    ///
    pub fn add(&mut self, other: &Matrix2) -> &Matrix2 {
        if self.size() != other.size() {
            panic!()
        }
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                self.data[(i, j)] += other.value(i, j)
            }
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::core::Matrix2;

    #[test]
    fn test_get() {
        let m = Matrix2::from_vec(&vec![
            vec![1f64, 2f64],
            vec![3f64, 4f64]
        ]);
        assert_eq!(m.value(0, 0), 1f64);
        assert_eq!(m.value(0, 1), 2f64);
        assert_eq!(m.value(1, 0), 3f64);
        assert_eq!(m.value(1, 1), 4f64);
    }

    #[test]
    fn test_set() {
        let mut m = Matrix2::from_vec(&vec![
            vec![1f64, 2f64],
            vec![3f64, 4f64]
        ]);

        m.set_value(0, 0, 15f64);

        assert_eq!(m.value(0, 0), 15f64);
        assert_eq!(m.value(0, 1), 2f64);
        assert_eq!(m.value(1, 0), 3f64);
        assert_eq!(m.value(1, 1), 4f64);
    }

    #[test]
    fn test_size() {
        let m = Matrix2::from_vec(&vec![
            vec![1f64, 2f64],
            vec![3f64, 4f64],
            vec![5f64, 6f64]
        ]);
        assert_eq!(m.rows(), 3);
        assert_eq!(m.cols(), 2);
    }

    #[test]
    fn test_add() {
        let m1 = Matrix2::from_vec(&vec![
            vec![1f64, 2f64, 3f64]
        ]);
        let mut m2 = Matrix2::from_vec(&vec![
            vec![1f64, 1f64, 1f64]
        ]);
        m2.add(&m1);
        assert_ne!(m2, m1);
        assert_eq!(m2, Matrix2::from_vec(&vec![
            vec![2f64, 3f64, 4f64]
        ]));
    }
}
