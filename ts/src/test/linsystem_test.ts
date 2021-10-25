/**
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2021.10.25
 *
 * Copyright (c) 2021 Luca Carlon. All rights reserved.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

import { linsolve } from "../core/linsystem"
import { backwardSub, forwardSub, luDecomp } from "../core/lu"
import { ColVector, Matrix2 } from "../core/matrix"
// @ts-expect-error
var assert = require("assert")

// Test 1
{
    let A = new Matrix2([
        [1, 1],
        [-3, 1]
    ])
    let b = new ColVector([6, 2])
    let x = linsolve(A, b)
    assert(x.equals(new ColVector([1, 5])))
}
