use imap_codec::{encode::Encoder, GreetingCodec};
use imap_types::{
    core::Text,
    response::{Code, Greeting, GreetingKind},
};
use pyo3::prelude::*;
use serde::{de::value::StrDeserializer, Deserialize};

#[pyclass(name = "Greeting")]
struct PyGreeting(Greeting<'static>);

#[pymethods]
impl PyGreeting {
    #[new]
    pub fn new(kind: GreetingKind, text: &str, code: Option<&str>) -> PyResult<Self> {
        Ok(Self(Greeting {
            kind,
            code: code
                .map(|c| Code::deserialize(StrDeserializer::<serde_pyobject::Error>::new(c)))
                .transpose()?,
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

#[pymodule]
#[pyo3(name = "imap_types")]
fn imap_types_python(m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<GreetingKind>()?;
    m.add_class::<PyGreeting>()?;

    Ok(())
}
