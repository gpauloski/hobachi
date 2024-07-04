use pyo3::basic::CompareOp;
use pyo3::exceptions::{PyAttributeError, PyRuntimeError, PyTypeError};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::types::PyString;
use pyo3::types::PyTuple;

#[pyclass(module = "hobachi")]
struct Proxy {
    __factory__: PyObject,
    __target__: Option<PyObject>,
}

#[pymethods]
impl Proxy {
    #[new]
    fn new(factory: PyObject) -> PyResult<Self> {
        Python::with_gil(|py| {
            if !factory.bind(py).is_callable() {
                Err(PyTypeError::new_err("The factory is not callable."))
            } else {
                Ok(Self {
                    __factory__: factory,
                    __target__: None,
                })
            }
        })
    }

    fn __getattr__<'py>(&'py mut self, name: &Bound<'py, PyString>) -> PyResult<Bound<'py, PyAny>> {
        let name_str = name.to_str()?;
        if name_str == "__factory__" || name_str == "__target__" {
            Err(PyAttributeError::new_err("Internal error."))
        } else {
            self.target()?.bind(name.py()).getattr(name)
        }
    }

    fn __setattr__(
        &mut self,
        name: &Bound<'_, PyString>,
        value: &Bound<'_, PyAny>,
    ) -> PyResult<()> {
        self.target()?.bind(name.py()).setattr(name, value)
    }

    fn __delattr__(&mut self, name: &Bound<'_, PyString>) -> PyResult<()> {
        self.target()?
            .bind(name.py())
            .call_method1("__delattr__", (name,))?;
        Ok(())
    }

    // pyo3 supported protocols: https://pyo3.rs/v0.22.0/class/protocols

    fn __repr__(&self) -> String {
        if let Some(target) = &self.__target__ {
            format!(
                "<Proxy wrapping {} with factory {}>",
                target, self.__factory__
            )
        } else {
            format!("<Proxy with factory {}>", self.__factory__)
        }
    }

    fn __str__(&mut self) -> PyResult<String> {
        Ok(self.target()?.to_string())
    }

    fn __hash__(&mut self) -> PyResult<isize> {
        Python::with_gil(|py| self.target()?.bind(py).hash())
    }

    #[pyo3(signature = (*args, **kwargs))]
    fn __call__(
        &mut self,
        args: Bound<'_, PyTuple>,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            let target = self.target()?;
            target.call_bound(py, args, kwargs)
        })
    }

    fn __bool__(&mut self) -> PyResult<bool> {
        Python::with_gil(|py| self.target()?.is_truthy(py))
    }

    fn __richcmp__(&mut self, other: &Bound<'_, PyAny>, op: CompareOp) -> PyResult<bool> {
        let binding = self.target()?;
        let left = binding.bind(other.py());
        match op {
            CompareOp::Lt => left.lt(other),
            CompareOp::Le => left.le(other),
            CompareOp::Eq => left.eq(other),
            CompareOp::Ne => left.ne(other),
            CompareOp::Gt => left.gt(other),
            CompareOp::Ge => left.ge(other),
        }
    }

    fn __add__<'py>(&'py mut self, other: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        self.target()?.bind(other.py()).add(other)
    }

    fn __sub__<'py>(&'py mut self, other: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        self.target()?.bind(other.py()).sub(other)
    }

    fn __mul__<'py>(&'py mut self, other: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        self.target()?.bind(other.py()).mul(other)
    }

    fn __truediv__<'py>(&'py mut self, other: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        self.target()?.bind(other.py()).div(other)
    }

    fn __floordiv__<'py>(&'py mut self, other: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        self.target()?.bind(other.py()).floor_div(other)
    }

    fn __lshift__<'py>(&'py mut self, other: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        self.target()?.bind(other.py()).lshift(other)
    }

    fn __rshift__<'py>(&'py mut self, other: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        self.target()?.bind(other.py()).rshift(other)
    }
}

impl Proxy {
    pub fn resolved(&self) -> bool {
        self.__target__.is_some()
    }

    pub fn target(&mut self) -> PyResult<PyObject> {
        if self.__target__.is_none() {
            let result =
                Python::with_gil(|py| -> PyResult<PyObject> { self.__factory__.call0(py) })?;
            let _ = self.__target__.insert(result);
        }

        if let Some(target) = &self.__target__ {
            Python::with_gil(|py| Ok(target.clone_ref(py)))
        } else {
            Err(PyRuntimeError::new_err("Failed to resolve proxy."))
        }
    }
}

#[pyfunction]
fn extract(p: &Bound<'_, Proxy>) -> PyResult<PyObject> {
    p.borrow_mut().target()
}

#[pyfunction]
fn is_resolved(p: &Bound<'_, Proxy>) -> bool {
    p.borrow().resolved()
}

#[pyfunction]
fn resolve(p: &Bound<'_, Proxy>) -> PyResult<()> {
    p.borrow_mut().target().map(|_| ())
}

#[pymodule]
fn hobachi(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Proxy>()?;
    m.add_function(wrap_pyfunction!(extract, m)?).unwrap();
    m.add_function(wrap_pyfunction!(is_resolved, m)?).unwrap();
    m.add_function(wrap_pyfunction!(resolve, m)?).unwrap();
    Ok(())
}
