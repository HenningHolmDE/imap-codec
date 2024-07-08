use imap_codec::{
    decode::{self, Decoder},
    encode::{Encoded, Encoder},
    CommandCodec, GreetingCodec, ResponseCodec,
};
use pyo3::{create_exception, exceptions::PyException, prelude::*, types::PyBytes};

#[derive(Debug, Clone, PartialEq)]
#[pyclass(name = "CommandCodec")]
struct PyCommandCodec(CommandCodec);

#[derive(Debug, Clone, PartialEq)]
#[pyclass(name = "GreetingCodec")]
struct PyGreetingCodec(GreetingCodec);

#[derive(Debug, Clone, PartialEq)]
#[pyclass(name = "ResponseCodec")]
struct PyResponseCodec(ResponseCodec);

// Create exceptions for command decode errors
create_exception!(imap_codec, CommandDecodeError, PyException);
create_exception!(imap_codec, CommandDecodeFailed, CommandDecodeError);
create_exception!(imap_codec, CommandDecodeIncomplete, CommandDecodeError);
create_exception!(imap_codec, CommandDecodeLiteralFound, CommandDecodeError);
// Create exceptions for greeting decode errors
create_exception!(imap_codec, GreetingDecodeError, PyException);
create_exception!(imap_codec, GreetingDecodeIncomplete, GreetingDecodeError);
create_exception!(imap_codec, GreetingDecodeFailed, GreetingDecodeError);
// Create exceptions for response decode errors
create_exception!(imap_codec, ResponseDecodeError, PyException);
create_exception!(imap_codec, ResponseDecodeFailed, ResponseDecodeError);
create_exception!(imap_codec, ResponseDecodeIncomplete, ResponseDecodeError);
create_exception!(imap_codec, ResponseDecodeLiteralFound, ResponseDecodeError);

#[derive(Debug, Clone)]
#[pyclass(name = "Encoded")]
struct PyEncoded(Option<Encoded>);

#[pymethods]
impl PyEncoded {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__<'py>(&mut self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyAny>>> {
        let Some(encoded) = &mut self.0 else {
            return Ok(None);
        };
        Ok(encoded
            .next()
            .map(|value| serde_pyobject::to_pyobject(py, &value))
            .transpose()?)
    }

    fn dump<'py>(&mut self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        // TODO: Could we change `Encoded::dump(self)` to `Encoded::dump(&mut self)`?
        let encoded = std::mem::take(&mut self.0);
        let dump = match encoded {
            Some(encoded) => encoded.dump(),
            None => Vec::new(),
        };
        Ok(PyBytes::new_bound(py, &dump))
    }
}

#[pymethods]
impl PyCommandCodec {
    #[staticmethod]
    fn encode<'a>(py: Python, message: PyObject) -> PyResult<PyEncoded> {
        let message = serde_pyobject::from_pyobject(message.into_bound(py))?;
        let encoded = CommandCodec::default().encode(&message);
        Ok(PyEncoded(Some(encoded)))
    }

    #[staticmethod]
    fn decode<'a>(py: Python, bytes: &'a [u8]) -> PyResult<(&'a [u8], PyObject)> {
        match CommandCodec::default().decode(bytes) {
            Ok((remaining, command)) => {
                Ok((remaining, serde_pyobject::to_pyobject(py, &command)?.into()))
            }
            Err(err) => Err(match err {
                decode::CommandDecodeError::Incomplete => CommandDecodeIncomplete::new_err(()),
                decode::CommandDecodeError::LiteralFound { tag, length, mode } => {
                    let dict = pyo3::types::PyDict::new_bound(py);
                    dict.set_item("tag", serde_pyobject::to_pyobject(py, &tag)?)?;
                    dict.set_item("length", length)?;
                    dict.set_item("mode", serde_pyobject::to_pyobject(py, &mode)?)?;
                    CommandDecodeLiteralFound::new_err(dict.unbind())
                }
                decode::CommandDecodeError::Failed => CommandDecodeFailed::new_err(()),
            }),
        }
    }
}

#[pymethods]
impl PyGreetingCodec {
    #[staticmethod]
    fn decode<'a>(py: Python, bytes: &'a [u8]) -> PyResult<(&'a [u8], PyObject)> {
        let (remaining, greeting) =
            GreetingCodec::default()
                .decode(bytes)
                .map_err(|e| match e {
                    decode::GreetingDecodeError::Incomplete => {
                        GreetingDecodeIncomplete::new_err(())
                    }
                    decode::GreetingDecodeError::Failed => GreetingDecodeFailed::new_err(()),
                })?;
        Ok((
            remaining,
            serde_pyobject::to_pyobject(py, &greeting)?.into(),
        ))
    }
}

#[pymethods]
impl PyResponseCodec {
    #[staticmethod]
    fn decode<'a>(py: Python, bytes: &'a [u8]) -> PyResult<(&'a [u8], PyObject)> {
        match ResponseCodec::default().decode(bytes) {
            Ok((remaining, response)) => Ok((
                remaining,
                serde_pyobject::to_pyobject(py, &response)?.into(),
            )),
            Err(err) => Err(match err {
                decode::ResponseDecodeError::Incomplete => ResponseDecodeIncomplete::new_err(()),
                decode::ResponseDecodeError::LiteralFound { length } => {
                    let dict = pyo3::types::PyDict::new_bound(py);
                    dict.set_item("length", length)?;
                    ResponseDecodeLiteralFound::new_err(dict.unbind())
                }
                decode::ResponseDecodeError::Failed => ResponseDecodeFailed::new_err(()),
            }),
        }
    }
}

#[pymodule]
#[pyo3(name = "imap_codec")]
fn imap_codec_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyCommandCodec>()?;
    m.add_class::<PyGreetingCodec>()?;
    m.add_class::<PyResponseCodec>()?;
    m.add(
        "CommandDecodeError",
        m.py().get_type_bound::<CommandDecodeError>(),
    )?;
    m.add(
        "CommandDecodeFailed",
        m.py().get_type_bound::<CommandDecodeFailed>(),
    )?;
    m.add(
        "CommandDecodeIncomplete",
        m.py().get_type_bound::<CommandDecodeIncomplete>(),
    )?;
    m.add(
        "CommandDecodeLiteralFound",
        m.py().get_type_bound::<CommandDecodeLiteralFound>(),
    )?;
    m.add(
        "GreetingDecodeError",
        m.py().get_type_bound::<GreetingDecodeError>(),
    )?;
    m.add(
        "GreetingDecodeFailed",
        m.py().get_type_bound::<GreetingDecodeFailed>(),
    )?;
    m.add(
        "GreetingDecodeIncomplete",
        m.py().get_type_bound::<GreetingDecodeIncomplete>(),
    )?;
    m.add(
        "ResponseDecodeError",
        m.py().get_type_bound::<ResponseDecodeError>(),
    )?;
    m.add(
        "ResponseDecodeFailed",
        m.py().get_type_bound::<ResponseDecodeFailed>(),
    )?;
    m.add(
        "ResponseDecodeIncomplete",
        m.py().get_type_bound::<ResponseDecodeIncomplete>(),
    )?;
    m.add(
        "ResponseDecodeLiteralFound",
        m.py().get_type_bound::<ResponseDecodeLiteralFound>(),
    )?;

    m.add_class::<PyEncoded>()?;
    Ok(())
}
