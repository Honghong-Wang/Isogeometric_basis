/*
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2022.02.01
 *
 * Copyright (c) 2022 Luca Carlon. All rights reserved.
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

use crate::bezier::BezierSurf;
use crate::bezier::BezierFactory;

pub const TEASPOON_PACTHES: [[usize; 16]; 16] = [
    [   1,   2,   3,   4,   5,   6,   7,   8,   9,  10,  11,  12,  13,  14,  15,  16 ],
    [  17,  18,  19,  20,  21,  22,  23,  24,  25,  26,  27,  28,  29,  30,  31,  32 ],
    [  33,  34,  35,  36,  37,  38,  39,  40,  41,  42,  43,  44,  45,  46,  47,  48 ],
    [  49,  50,  51,  52,  53,  54,  55,  56,  57,  58,  59,  60,  61,  62,  63,  64 ],
    [  65,  66,  67,  68,  69,  70,  71,  72,  73,  74,  75,  76,  77,  78,  79,  80 ],
    [  81,  82,  83,  84,  85,  86,  87,  88,  89,  90,  91,  92,  93,  94,  95,  96 ],
    [  97,  98,  99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112 ],
    [ 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128 ],
    [ 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144 ],
    [ 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160 ],
    [ 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176 ],
    [ 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192 ],
    [ 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208 ],
    [ 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224 ],
    [ 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240 ],
    [ 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255, 256 ]
];

pub const TEASPOON_VERTICES: [[f64; 3]; 256] = [
    [ -0.000107143,  0.205357000,  0.000000000 ],
    [  0.000000000,  0.196429000, -0.017857100 ],
    [  0.000000000,  0.196429000, -0.017857100 ],
    [  0.000107143,  0.205357000,  0.000000000 ],
    [ -0.053571400,  0.205357000,  0.000000000 ],
    [ -0.022271400,  0.178571000, -0.053428600 ],
    [  0.022271400,  0.178571000, -0.053428600 ],
    [  0.053571400,  0.205357000,  0.000000000 ],
    [ -0.107143000,  0.095242900, -0.017857100 ],
    [ -0.044642900,  0.095242900, -0.089285700 ],
    [  0.044642900,  0.095242900, -0.089285700 ],
    [  0.107143000,  0.095242900, -0.017857100 ],
    [ -0.107143000,  0.000000000, -0.017857100 ],
    [ -0.044642900,  0.000000000, -0.089285700 ],
    [  0.044642900,  0.000000000, -0.089285700 ],
    [  0.107143000,  0.000000000, -0.017857100 ],
    [  0.000107143,  0.205357000,  0.000000000 ],
    [  0.000135714,  0.207589000,  0.004464290 ],
    [  0.000157143,  0.216518000,  0.004464290 ],
    [  0.000125000,  0.214286000,  0.000000000 ],
    [  0.053571400,  0.205357000,  0.000000000 ],
    [  0.061396400,  0.212054000,  0.013357100 ],
    [  0.071428600,  0.220982000,  0.015625000 ],
    [  0.062500000,  0.214286000,  0.000000000 ],
    [  0.107143000,  0.095242900, -0.017857100 ],
    [  0.122768000,  0.095242900,  0.000000000 ],
    [  0.142857000,  0.095242900,  0.004464290 ],
    [  0.125000000,  0.095242900, -0.017857100 ],
    [  0.107143000,  0.000000000, -0.017857100 ],
    [  0.122768000,  0.000000000,  0.000000000 ],
    [  0.142857000,  0.000000000,  0.004464290 ],
    [  0.125000000,  0.000000000, -0.017857100 ],
    [  0.000125000,  0.214286000,  0.000000000 ],
    [  0.000000000,  0.205357000, -0.017857100 ],
    [  0.000000000,  0.205357000, -0.017857100 ],
    [ -0.000125000,  0.214286000,  0.000000000 ],
    [  0.062500000,  0.214286000,  0.000000000 ],
    [  0.026785700,  0.187500000, -0.062500000 ],
    [ -0.026785700,  0.187500000, -0.062500000 ],
    [ -0.062500000,  0.214286000,  0.000000000 ],
    [  0.125000000,  0.095242900, -0.017857100 ],
    [  0.053571400,  0.095242900, -0.107143000 ],
    [ -0.053571400,  0.095242900, -0.107143000 ],
    [ -0.125000000,  0.095242900, -0.017857100 ],
    [  0.125000000,  0.000000000, -0.017857100 ],
    [  0.053571400,  0.000000000, -0.107143000 ],
    [ -0.053571400,  0.000000000, -0.107143000 ],
    [ -0.125000000,  0.000000000, -0.017857100 ],
    [ -0.000125000,  0.214286000,  0.000000000 ],
    [ -0.000157143,  0.216518000,  0.004464290 ],
    [ -0.000135714,  0.207589000,  0.004464290 ],
    [ -0.000107143,  0.205357000,  0.000000000 ],
    [ -0.062500000,  0.214286000,  0.000000000 ],
    [ -0.071428600,  0.220982000,  0.015625000 ],
    [ -0.061396400,  0.212054000,  0.013357100 ],
    [ -0.053571400,  0.205357000,  0.000000000 ],
    [ -0.125000000,  0.095242900, -0.017857100 ],
    [ -0.142857000,  0.095242900,  0.004464290 ],
    [ -0.122768000,  0.095242900,  0.000000000 ],
    [ -0.107143000,  0.095242900, -0.017857100 ],
    [ -0.125000000,  0.000000000, -0.017857100 ],
    [ -0.142857000,  0.000000000,  0.004464290 ],
    [ -0.122768000,  0.000000000,  0.000000000 ],
    [ -0.107143000,  0.000000000, -0.017857100 ],
    [ -0.107143000,  0.000000000, -0.017857100 ],
    [ -0.044642900,  0.000000000, -0.089285700 ],
    [  0.044642900,  0.000000000, -0.089285700 ],
    [  0.107143000,  0.000000000, -0.017857100 ],
    [ -0.107143000, -0.142857000, -0.017857100 ],
    [ -0.044642900, -0.142857000, -0.089285700 ],
    [  0.044642900, -0.142857000, -0.089285700 ],
    [  0.107143000, -0.142857000, -0.017857100 ],
    [ -0.013392900, -0.160714000,  0.038689300 ],
    [ -0.005578570, -0.160714000,  0.038689300 ],
    [  0.005578570, -0.160714000,  0.038689300 ],
    [  0.013392900, -0.160714000,  0.038689300 ],
    [ -0.013392900, -0.250000000,  0.053571400 ],
    [ -0.005578570, -0.250000000,  0.053571400 ],
    [  0.005578570, -0.250000000,  0.053571400 ],
    [  0.013392900, -0.250000000,  0.053571400 ],
    [  0.107143000,  0.000000000, -0.017857100 ],
    [  0.122768000,  0.000000000,  0.000000000 ],
    [  0.142857000,  0.000000000,  0.004464290 ],
    [  0.125000000,  0.000000000, -0.017857100 ],
    [  0.107143000, -0.142857000, -0.017857100 ],
    [  0.122768000, -0.142857000,  0.000000000 ],
    [  0.142857000, -0.142857000,  0.004464290 ],
    [  0.125000000, -0.142857000, -0.017857100 ],
    [  0.013392900, -0.160714000,  0.038689300 ],
    [  0.015346400, -0.160714000,  0.038689300 ],
    [  0.017857100, -0.160714000,  0.031435700 ],
    [  0.015625000, -0.160714000,  0.029760700 ],
    [  0.013392900, -0.250000000,  0.053571400 ],
    [  0.015346400, -0.250000000,  0.053571400 ],
    [  0.017857100, -0.250000000,  0.046317900 ],
    [  0.015625000, -0.250000000,  0.044642900 ],
    [  0.125000000,  0.000000000, -0.017857100 ],
    [  0.053571400,  0.000000000, -0.107143000 ],
    [ -0.053571400,  0.000000000, -0.107143000 ],
    [ -0.125000000,  0.000000000, -0.017857100 ],
    [  0.125000000, -0.142857000, -0.017857100 ],
    [  0.053571400, -0.142857000, -0.107143000 ],
    [ -0.053571400, -0.142857000, -0.107143000 ],
    [ -0.125000000, -0.142857000, -0.017857100 ],
    [  0.015625000, -0.160714000,  0.029760700 ],
    [  0.006696430, -0.160714000,  0.023064300 ],
    [ -0.007810710, -0.160714000,  0.020832100 ],
    [ -0.015625000, -0.160714000,  0.029760700 ],
    [  0.015625000, -0.250000000,  0.044642900 ],
    [  0.006696430, -0.250000000,  0.037946400 ],
    [ -0.007810710, -0.250000000,  0.035714300 ],
    [ -0.015625000, -0.250000000,  0.044642900 ],
    [ -0.125000000,  0.000000000, -0.017857100 ],
    [ -0.142857000,  0.000000000,  0.004464290 ],
    [ -0.122768000,  0.000000000,  0.000000000 ],
    [ -0.107143000,  0.000000000, -0.017857100 ],
    [ -0.125000000, -0.142857000, -0.017857100 ],
    [ -0.142857000, -0.142857000,  0.004464290 ],
    [ -0.122768000, -0.142857000,  0.000000000 ],
    [ -0.107143000, -0.142857000, -0.017857100 ],
    [ -0.015625000, -0.160714000,  0.029760700 ],
    [ -0.017578600, -0.160714000,  0.031992900 ],
    [ -0.015346400, -0.160714000,  0.038689300 ],
    [ -0.013392900, -0.160714000,  0.038689300 ],
    [ -0.015625000, -0.250000000,  0.044642900 ],
    [ -0.017578600, -0.250000000,  0.046875000 ],
    [ -0.015346400, -0.250000000,  0.053571400 ],
    [ -0.013392900, -0.250000000,  0.053571400 ],
    [ -0.013392900, -0.250000000,  0.053571400 ],
    [ -0.005578570, -0.250000000,  0.053571400 ],
    [  0.005578570, -0.250000000,  0.053571400 ],
    [  0.013392900, -0.250000000,  0.053571400 ],
    [ -0.013392900, -0.464250000,  0.089285700 ],
    [ -0.005578570, -0.464250000,  0.089285700 ],
    [  0.005578570, -0.464250000,  0.089285700 ],
    [  0.013392900, -0.464250000,  0.089285700 ],
    [ -0.044642900, -0.678571000,  0.053571400 ],
    [ -0.008928570, -0.678571000,  0.062500000 ],
    [  0.008928570, -0.678571000,  0.062500000 ],
    [  0.044642900, -0.678571000,  0.053571400 ],
    [ -0.044642900, -0.857143000,  0.035714300 ],
    [ -0.008928570, -0.857143000,  0.044642900 ],
    [  0.008928570, -0.857143000,  0.044642900 ],
    [  0.044642900, -0.857143000,  0.035714300 ],
    [  0.013392900, -0.250000000,  0.053571400 ],
    [  0.015346400, -0.250000000,  0.053571400 ],
    [  0.017857100, -0.250000000,  0.046317900 ],
    [  0.015625000, -0.250000000,  0.044642900 ],
    [  0.013392900, -0.464250000,  0.089285700 ],
    [  0.015346400, -0.464286000,  0.089285700 ],
    [  0.017857100, -0.464250000,  0.082032100 ],
    [  0.015625000, -0.464250000,  0.080357100 ],
    [  0.044642900, -0.678571000,  0.053571400 ],
    [  0.053571400, -0.678571000,  0.051339300 ],
    [  0.053571400, -0.678571000,  0.033482100 ],
    [  0.044642900, -0.678571000,  0.035714300 ],
    [  0.044642900, -0.857143000,  0.035714300 ],
    [  0.053571400, -0.857143000,  0.033482100 ],
    [  0.053571400, -0.857143000,  0.015625000 ],
    [  0.044642900, -0.857143000,  0.017857100 ],
    [  0.015625000, -0.250000000,  0.044642900 ],
    [  0.006696430, -0.250000000,  0.037946400 ],
    [ -0.007810710, -0.250000000,  0.035714300 ],
    [ -0.015625000, -0.250000000,  0.044642900 ],
    [  0.015625000, -0.464250000,  0.080357100 ],
    [  0.006696430, -0.464286000,  0.073660700 ],
    [ -0.007810710, -0.464250000,  0.071428600 ],
    [ -0.015625000, -0.464250000,  0.080357100 ],
    [  0.044642900, -0.678571000,  0.035714300 ],
    [  0.008928570, -0.678571000,  0.044642900 ],
    [ -0.008928570, -0.678571000,  0.044642900 ],
    [ -0.044642900, -0.678571000,  0.035714300 ],
    [  0.044642900, -0.857143000,  0.017857100 ],
    [  0.008928570, -0.857143000,  0.026785700 ],
    [ -0.008928570, -0.857143000,  0.026785700 ],
    [ -0.044642900, -0.857143000,  0.017857100 ],
    [ -0.015625000, -0.250000000,  0.044642900 ],
    [ -0.017578600, -0.250000000,  0.046875000 ],
    [ -0.015346400, -0.250000000,  0.053571400 ],
    [ -0.013392900, -0.250000000,  0.053571400 ],
    [ -0.015625000, -0.464250000,  0.080357100 ],
    [ -0.017578600, -0.464286000,  0.082589300 ],
    [ -0.015346400, -0.464286000,  0.089285700 ],
    [ -0.013392900, -0.464250000,  0.089285700 ],
    [ -0.044642900, -0.678571000,  0.035714300 ],
    [ -0.053571400, -0.678571000,  0.033482100 ],
    [ -0.053571400, -0.678571000,  0.051339300 ],
    [ -0.044642900, -0.678571000,  0.053571400 ],
    [ -0.044642900, -0.857143000,  0.017857100 ],
    [ -0.053571400, -0.857143000,  0.015625000 ],
    [ -0.053571400, -0.857143000,  0.033482100 ],
    [ -0.044642900, -0.857143000,  0.035714300 ],
    [ -0.044642900, -0.857143000,  0.035714300 ],
    [ -0.008928570, -0.857143000,  0.044642900 ],
    [  0.008928570, -0.857143000,  0.044642900 ],
    [  0.044642900, -0.857143000,  0.035714300 ],
    [ -0.044642900, -0.928571000,  0.028571400 ],
    [ -0.008928570, -0.928571000,  0.037500000 ],
    [  0.008928570, -0.928571000,  0.037500000 ],
    [  0.044642900, -0.928571000,  0.028571400 ],
    [ -0.053928600, -0.999643000,  0.017857100 ],
    [  0.000357143, -0.999643000,  0.017857100 ],
    [  0.000000000, -0.999643000,  0.017857100 ],
    [  0.053571400, -0.999643000,  0.017857100 ],
    [ -0.000357143, -1.000000000,  0.017857100 ],
    [  0.000357143, -1.000000000,  0.017857100 ],
    [  0.000000000, -1.000000000,  0.017857100 ],
    [  0.000000000, -1.000000000,  0.017857100 ],
    [  0.044642900, -0.857143000,  0.035714300 ],
    [  0.053571400, -0.857143000,  0.033482100 ],
    [  0.053571400, -0.857143000,  0.015625000 ],
    [  0.044642900, -0.857143000,  0.017857100 ],
    [  0.044642900, -0.928571000,  0.028571400 ],
    [  0.053571400, -0.928571000,  0.026339300 ],
    [  0.053571400, -0.928571000,  0.008482140 ],
    [  0.044642900, -0.928571000,  0.010714300 ],
    [  0.053571400, -0.999643000,  0.017857100 ],
    [  0.066964300, -0.999643000,  0.017857100 ],
    [  0.067321400, -0.999643000,  0.000000000 ],
    [  0.053928600, -0.999643000,  0.000000000 ],
    [  0.000000000, -1.000000000,  0.017857100 ],
    [  0.000000000, -1.000000000,  0.017857100 ],
    [  0.000357143, -1.000000000,  0.000000000 ],
    [  0.000357143, -1.000000000,  0.000000000 ],
    [  0.044642900, -0.857143000,  0.017857100 ],
    [  0.008928570, -0.857143000,  0.026785700 ],
    [ -0.008928570, -0.857143000,  0.026785700 ],
    [ -0.044642900, -0.857143000,  0.017857100 ],
    [  0.044642900, -0.928571000,  0.010714300 ],
    [  0.008928570, -0.928571000,  0.019642900 ],
    [ -0.008928570, -0.928571000,  0.019642900 ],
    [ -0.044642900, -0.928571000,  0.010714300 ],
    [  0.053928600, -0.999643000,  0.000000000 ],
    [  0.000357143, -0.999643000,  0.000000000 ],
    [ -0.000357143, -0.999643000,  0.000000000 ],
    [ -0.053928600, -0.999643000,  0.000000000 ],
    [  0.000357143, -1.000000000,  0.000000000 ],
    [  0.000357143, -1.000000000,  0.000000000 ],
    [ -0.000357143, -1.000000000,  0.000000000 ],
    [ -0.000357143, -1.000000000,  0.000000000 ],
    [ -0.044642900, -0.857143000,  0.017857100 ],
    [ -0.053571400, -0.857143000,  0.015625000 ],
    [ -0.053571400, -0.857143000,  0.033482100 ],
    [ -0.044642900, -0.857143000,  0.035714300 ],
    [ -0.044642900, -0.928571000,  0.010714300 ],
    [ -0.053571400, -0.928571000,  0.008482140 ],
    [ -0.053571400, -0.928571000,  0.026339300 ],
    [ -0.044642900, -0.928571000,  0.028571400 ],
    [ -0.053928600, -0.999643000,  0.000000000 ],
    [ -0.067321400, -0.999643000,  0.000000000 ],
    [ -0.067500000, -0.999643000,  0.017857100 ],
    [ -0.053928600, -0.999643000,  0.017857100 ],
    [ -0.000357143, -1.000000000,  0.000000000 ],
    [ -0.000357143, -1.000000000,  0.000000000 ],
    [ -0.000535714, -1.000000000,  0.017857100 ],
    [ -0.000357143, -1.000000000,  0.017857100 ]
];

///
/// Builds the pacthes needed to draw the teaspoon.
/// 
/// ```rust
/// use isogeometric_analysis::bezier::BezierTeaspoon;
/// use isogeometric_analysis::core::RealRange;
/// use isogeometric_analysis::core::Evaluator;
/// let patches = BezierTeaspoon::build_patches();
/// for patch in patches {
///     let r = RealRange { a: 0f64, b: 1f64 };
///     let (_xpoints, ypoints) = Evaluator::<2, 3, 100>::evaluate_parametric_range2d(&patch, &r, &r);
///     let (xvalues, yvalues, zvalues) = Evaluator::<2, 3, 0>::split_coords(0, &ypoints, 1, &ypoints, 2, &ypoints);
/// }
/// ```
/// 
pub struct BezierTeaspoon {}

impl BezierTeaspoon {
	///
	/// Build the patches from raw data.
	/// 
	pub fn build_patches() -> Vec<BezierSurf<3>> {
		BezierFactory::from_indexed_vertices(TEASPOON_PACTHES.to_vec(), TEASPOON_VERTICES.to_vec())
	}
}
