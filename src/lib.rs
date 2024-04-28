use pyo3::basic::CompareOp;
use pyo3::exceptions::{PyRuntimeError, PyTypeError};
use pyo3::prelude::*;
use pyo3::types::PyString;

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

    #[allow(non_snake_case)]
    #[getter]
    fn get__factory__(&self) -> PyResult<PyObject> {
        Ok(self.__factory__.clone())
    }

    #[allow(non_snake_case)]
    #[setter]
    fn set__factory__(&mut self, value: PyObject) -> PyResult<()> {
        self.__factory__ = value;
        Ok(())
    }

    #[allow(non_snake_case)]
    #[getter]
    fn get__target__(&mut self) -> PyResult<PyObject> {
        self.target()
    }

    #[allow(non_snake_case)]
    #[setter]
    fn set__target__(&mut self, value: PyObject) -> PyResult<()> {
        let _ = self.__target__.insert(value);
        Ok(())
    }

    fn __getattr__<'py>(&'py mut self, name: &Bound<'py, PyString>) -> PyResult<Bound<'py, PyAny>> {
        self.target()?.bind(name.py()).getattr(name)
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
        self.target()?.bind(other.py()).div(other)
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

    fn target(&mut self) -> PyResult<PyObject> {
        if self.__target__.is_none() {
            let result =
                Python::with_gil(|py| -> PyResult<PyObject> { self.__factory__.call0(py) })?;
            let _ = self.__target__.insert(result);
        }

        if let Some(target) = self.__target__.clone() {
            Ok(target)
        } else {
            Err(PyRuntimeError::new_err("Failed to resolve proxy."))
        }
    }
}

#[pymodule]
fn hobachi(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Proxy>()?;
    Ok(())
}
