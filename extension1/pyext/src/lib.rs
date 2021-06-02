
#[macro_use]
extern crate cpython;
use cpython::{Python, PyResult, PyDict};

// fn count_string_chars(_py: Python, word: &str) -> PyResult<u64> {
//     let mut total = 0u64;

//     for _ in word.chars() {
//         total += 1;
//     }
//     Ok(total)
// }
// // making above function importable from python
// py_module_initializer!(rust_pyext, initrust_pyext, PyInit_rust_pyext, |py, m| {
//     m.add(py, "count_string_chars", py_fn!(py, count_string_chars(word: &str)))
// });



fn count_string_chars(python: Python, word: &str) -> PyResult<PyDict> {
    let mut total = 0u64;

    for _ in word.chars() {
        total += 1;
    }
    let result = PyDict::new(python);
    result.set_item(python, "os", python.import("os")?)?;
    Ok(result)
}

// making above function importable from python
py_module_initializer!(rust_pyext, initrust_pyext, PyInit_rust_pyext, |py, m| {
    m.add(py, "count_string_chars", py_fn!(py, count_string_chars(word: &str)))
});