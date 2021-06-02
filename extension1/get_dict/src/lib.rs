
#[macro_use]
extern crate cpython;
use cpython::{Python, PyResult};

fn model_function(_: Python, idx: u32) -> PyResult<(bool, i32)> {  // PyResult<Vec<u32>> return python list
    let real_idx:u32 = 22;
    let v = if idx == real_idx {(true, 22)} else {(false, 23)};
    Ok(v)
}

// making above function importable from pytho,n
py_module_initializer!(rust_pyext, initrust_pyext, PyInit_rust_pyext, |py, m| {
    m.add(py, "model_function", py_fn!(py, model_function(idx: u32)))
    });






// #[macro_use]
// extern crate cpython;
// use cpython::{Python, PyResult, PyDict};

// fn count_string_chars(python: Python, word: &str) -> PyResult<PyDict> {
//     let mut total = 0u64;

//     for _ in word.chars() {
//         total += 1;
//     }
//     let result = PyDict::new(python);
//     result.set_item(python, "output", total)?;
//     Ok(result)
// }

// // making above function importable from python
// py_module_initializer!(rust_pyext, initrust_pyext, PyInit_rust_pyext, |py, m| {
//     m.add(py, "count_string_chars", py_fn!(py, count_string_chars(word: &str)))
//     });




// #[macro_use]
// extern crate cpython;
// use cpython::{Python, PyResult, PyDict};

// fn count_string_chars(python: Python, word: &str) -> PyResult<PyDict> {
//     let mut total = 0u64;

//     for _ in word.chars() {
//         total += 1;
//     }
//     let result = PyDict::new(python);
//     result.set_item(python, "os", python.import("os")?)?;
//     Ok(result)
// }

// // making above function importable from python
// py_module_initializer!(rust_pyext, initrust_pyext, PyInit_rust_pyext, |py, m| {
//     m.add(py, "count_string_chars", py_fn!(py, count_string_chars(word: &str)))
//     });
