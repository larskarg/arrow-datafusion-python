// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use pyo3::{basic::CompareOp, prelude::*};
use std::convert::{From, Into};

use datafusion::arrow::datatypes::DataType;
use datafusion::arrow::pyarrow::PyArrowType;
use datafusion_expr::{col, lit, Expr};

use datafusion::scalar::ScalarValue;

/// An PyExpr that can be used on a DataFrame
#[pyclass(name = "Expression", module = "datafusion", subclass)]
#[derive(Debug, Clone)]
pub(crate) struct PyExpr {
    pub(crate) expr: Expr,
}

impl From<PyExpr> for Expr {
    fn from(expr: PyExpr) -> Expr {
        expr.expr
    }
}

impl From<Expr> for PyExpr {
    fn from(expr: Expr) -> PyExpr {
        PyExpr { expr }
    }
}

#[pymethods]
impl PyExpr {
    fn __richcmp__(&self, other: PyExpr, op: CompareOp) -> PyExpr {
        let expr = match op {
            CompareOp::Lt => self.expr.clone().lt(other.expr),
            CompareOp::Le => self.expr.clone().lt_eq(other.expr),
            CompareOp::Eq => self.expr.clone().eq(other.expr),
            CompareOp::Ne => self.expr.clone().not_eq(other.expr),
            CompareOp::Gt => self.expr.clone().gt(other.expr),
            CompareOp::Ge => self.expr.clone().gt_eq(other.expr),
        };
        expr.into()
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{}", self.expr))
    }

    fn __add__(&self, rhs: PyExpr) -> PyResult<PyExpr> {
        Ok((self.expr.clone() + rhs.expr).into())
    }

    fn __sub__(&self, rhs: PyExpr) -> PyResult<PyExpr> {
        Ok((self.expr.clone() - rhs.expr).into())
    }

    fn __truediv__(&self, rhs: PyExpr) -> PyResult<PyExpr> {
        Ok((self.expr.clone() / rhs.expr).into())
    }

    fn __mul__(&self, rhs: PyExpr) -> PyResult<PyExpr> {
        Ok((self.expr.clone() * rhs.expr).into())
    }

    fn __mod__(&self, rhs: PyExpr) -> PyResult<PyExpr> {
        Ok(self.expr.clone().modulus(rhs.expr).into())
    }

    fn __and__(&self, rhs: PyExpr) -> PyResult<PyExpr> {
        Ok(self.expr.clone().and(rhs.expr).into())
    }

    fn __or__(&self, rhs: PyExpr) -> PyResult<PyExpr> {
        Ok(self.expr.clone().or(rhs.expr).into())
    }

    fn __invert__(&self) -> PyResult<PyExpr> {
        Ok(self.expr.clone().not().into())
    }

    fn __getitem__(&self, key: &str) -> PyResult<PyExpr> {
        Ok(Expr::GetIndexedField {
            expr: Box::new(self.expr.clone()),
            key: ScalarValue::Utf8(Some(key.to_string())),
        }
        .into())
    }

    #[staticmethod]
    pub fn literal(value: ScalarValue) -> PyExpr {
        lit(value).into()
    }

    #[staticmethod]
    pub fn column(value: &str) -> PyExpr {
        col(value).into()
    }

    /// assign a name to the PyExpr
    pub fn alias(&self, name: &str) -> PyExpr {
        self.expr.clone().alias(name).into()
    }

    /// Create a sort PyExpr from an existing PyExpr.
    #[args(ascending = true, nulls_first = true)]
    pub fn sort(&self, ascending: bool, nulls_first: bool) -> PyExpr {
        self.expr.clone().sort(ascending, nulls_first).into()
    }

    pub fn is_null(&self) -> PyExpr {
        self.expr.clone().is_null().into()
    }

    pub fn cast(&self, to: PyArrowType<DataType>) -> PyExpr {
        // self.expr.cast_to() requires DFSchema to validate that the cast
        // is supported, omit that for now
        let expr = Expr::Cast {
            expr: Box::new(self.expr.clone()),
            data_type: to.0,
        };
        expr.into()
    }
}
