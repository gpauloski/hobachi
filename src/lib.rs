use pyo3::exceptions::{PyTypeError, PyRuntimeError};
use pyo3::prelude::*;


#[pyclass]
struct Proxy {
    factory: Py<PyAny>,
    target: Option<Py<PyAny>>,
}

#[pymethods]
impl Proxy {
    #[new]
    fn new(factory: Py<PyAny>) -> PyResult<Self> {
        Python::with_gil(|py| {
            if !factory.bind(py).is_callable() {
                Err(PyTypeError::new_err("The factory is not callable."))
            } else {
                Ok(Self { factory, target: None })
            }
        })
    }

    fn __bool__(slf: &Bound<'_, Self>) -> PyResult<bool> {
        Python::with_gil(|py| {
            slf.borrow_mut().get_target()?.bind(py).is_truthy()
        })
    }
}

impl Proxy {
    fn resolve(&mut self) -> PyResult<()> {
        if self.target.is_none() {
            let target = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                self.factory.call0(py)
            })?;
            let _ = self.target.insert(target);
        }
        Ok(())
    }

    fn get_target(&mut self) -> PyResult<&Py<PyAny>> {
        self.resolve()?;
        if let Some(target) = &self.target {
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
