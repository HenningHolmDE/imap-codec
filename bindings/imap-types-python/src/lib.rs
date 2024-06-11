use imap_codec::{decode::Decoder, encode::Encoder, GreetingCodec};
use imap_types::{
    core::Text,
    response::{Code, Greeting, GreetingKind},
};
use pyo3::{exceptions::PyRuntimeError, prelude::*};
use serde::{de::value::StrDeserializer, Deserialize};

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
#[pyclass(name = "Code")]
struct PyCode(Code<'static>);

#[pymethods]
impl PyCode {
    #[new]
    pub fn new(py: Python, code: PyObject) -> PyResult<Self> {
        Ok(Self(serde_pyobject::from_pyobject(code.into_bound(py))?))
    }

    fn __repr__(&self, py: Python) -> String {
        let obj = serde_pyobject::to_pyobject(py, &self.0).unwrap();
        format!("Code({:?})", obj)
    }

    fn __str__(&self) -> String {
        format!("{:?}", self.0)
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
#[pyclass(name = "Greeting")]
struct PyGreeting(Greeting<'static>);

#[pymethods]
impl PyGreeting {
    #[new]
    pub fn new(kind: GreetingKind, text: &str, code: Option<PyCode>) -> PyResult<Self> {
        Ok(Self(Greeting {
            kind,
            code: code.map(|c| c.0),
            text: Text::deserialize(StrDeserializer::<serde_pyobject::Error>::new(text))?,
        }))
    }

    fn __repr__(&self, py: Python) -> String {
        let obj = serde_pyobject::to_pyobject(py, &self.0).unwrap();
        format!("Greeting({:?})", obj)
    }

    fn __str__(&self) -> String {
        let codec = GreetingCodec::new();

        // TODO: * Clarify if we want to use `imap-codec` already
        //       * Resolve `unwrap()`
        format!(
            "{}",
            String::from_utf8(codec.encode(&self.0).dump()).unwrap()
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(name = "GreetingCodec")]
struct PyGreetingCodec(GreetingCodec);

#[pymethods]
impl PyGreetingCodec {
    #[new]
    pub fn new() -> PyResult<Self> {
        Ok(Self(GreetingCodec::default()))
    }

    fn decode<'a>(&self, bytes: &[u8]) -> PyResult<(&[u8], PyGreeting)> {
        // TODO: `PyGreeting`` requires `Greeting<'static>`, can we achieve this differently?
        let bytes = Vec::from(bytes).leak();
        let codec = self.0.clone();
        let (remaining, greeting) = codec
            .decode(bytes)
            .map_err(|e| PyRuntimeError::new_err(format!("GreetingDecodeError::{e:?}")))?;
        Ok((remaining, PyGreeting(greeting)))
    }

    fn __repr__(&self) -> String {
        format!("GreetingCodec({:?})", self.0)
    }

    fn __str__(&self) -> String {
        format!("GreetingCodec({:?})", self.0)
    }
}

#[pymodule]
#[pyo3(name = "imap_types")]
fn imap_types_python(m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<GreetingKind>()?;
    m.add_class::<PyCode>()?;
    m.add_class::<PyGreeting>()?;
    m.add_class::<PyGreetingCodec>()?;

    Ok(())
}
