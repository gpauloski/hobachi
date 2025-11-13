use pyo3::basic::CompareOp;
use pyo3::exceptions::{PyAttributeError, PyRuntimeError, PyTypeError};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyIterator, PyString, PyTuple};

#[pyclass(module = "hobachi")]
struct Proxy {
    __factory__: Py<PyAny>,
    __target__: Option<Py<PyAny>>,
}

#[pymethods]
impl Proxy {
    #[new]
    fn new(factory: Py<PyAny>) -> PyResult<Self> {
        Python::attach(|py| {
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

    // Basic object customization
    // https://pyo3.rs/v0.22.0/class/protocols#basic-object-customization

    fn __str__(&mut self) -> PyResult<String> {
        Ok(self.target()?.to_string())
    }

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

    fn __hash__(&mut self) -> PyResult<isize> {
        Python::attach(|py| self.target()?.bind(py).hash())
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

    fn __bool__(&mut self) -> PyResult<bool> {
        Python::attach(|py| self.target()?.is_truthy(py))
    }

    #[pyo3(signature = (*args, **kwargs))]
    fn __call__(
        &mut self,
        args: Bound<'_, PyTuple>,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Py<PyAny>> {
        Python::attach(|py| {
            let target = self.target()?;
            target.call(py, args, kwargs)
        })
    }

    // Iterable objects
    // https://pyo3.rs/v0.22.0/class/protocols#numeric-types

    fn __iter__(&mut self) -> PyResult<Py<PyIterator>> {
        Python::attach(|py| Ok(self.target()?.bind(py).try_iter()?.unbind()))
    }

    fn __next__(&mut self) -> PyResult<Py<PyAny>> {
        self.call_method0("__next__")
    }

    // Awaitable objects (not implemented!)
    // https://pyo3.rs/v0.22.0/class/protocols#awaitable-objects

    // Mapping & Sequence types
    // https://pyo3.rs/v0.22.0/class/protocols#mapping--sequence-types
    //
    // Note that __concat__, __repeat__, __inplace_concat__, and __inplace_repeat__
    // are not implemented because __add__, __mul__, __iadd__, and __imul__ are
    // already.

    fn __len__(&mut self) -> PyResult<usize> {
        Python::attach(|py| {
            self.target()?
                .bind(py)
                .getattr("__len__")?
                .unbind()
                .call0(py)?
                .extract::<usize>(py)
        })
    }

    fn __contains__<'py>(&'py mut self, value: &Bound<'py, PyAny>) -> PyResult<bool> {
        self.target()?.bind(value.py()).contains(value)
    }

    fn __getitem__(&mut self, key: Py<PyAny>) -> PyResult<Py<PyAny>> {
        self.call_method1("__getitem__", vec![key])
    }

    fn __setitem__(&mut self, key: Py<PyAny>, value: Py<PyAny>) -> PyResult<()> {
        self.call_method1("__setitem__", vec![key, value])?;
        Ok(())
    }

    fn __delitem__(&mut self, key: Py<PyAny>) -> PyResult<()> {
        self.call_method1("__delitem__", vec![key])?;
        Ok(())
    }

    // Numeric types
    // https://pyo3.rs/v0.22.0/class/protocols#numeric-types

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
    fn resolved(&self) -> bool {
        self.__target__.is_some()
    }

    fn target(&mut self) -> PyResult<Py<PyAny>> {
        if self.__target__.is_none() {
            let result =
                Python::attach(|py| -> PyResult<Py<PyAny>> { self.__factory__.call0(py) })?;
            let _ = self.__target__.insert(result);
        }

        if let Some(target) = &self.__target__ {
            Ok(Python::attach(|py| target.clone_ref(py)))
        } else {
            Err(PyRuntimeError::new_err("Failed to resolve proxy."))
        }
    }

    fn call_method0(&mut self, method: &str) -> PyResult<Py<PyAny>> {
        Python::attach(|py| self.target()?.bind(py).getattr(method)?.unbind().call0(py))
    }

    fn call_method1(&mut self, method: &str, args: Vec<Py<PyAny>>) -> PyResult<Py<PyAny>> {
        Python::attach(|py| {
            self.target()?
                .bind(py)
                .getattr(method)?
                .unbind()
                .call1(py, PyTuple::new(py, args)?)
        })
    }
}

#[pyfunction]
fn extract(p: &Bound<'_, Proxy>) -> PyResult<Py<PyAny>> {
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
