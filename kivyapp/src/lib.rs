#[macro_use]
extern crate cpython;
use cpython::{Python, PyResult};

use sqlite;

fn get_family_data_from_db(_: Python, _famid: u32) -> PyResult<Vec<(String, String)>>{
    let connection = sqlite::open("sqlite.db").unwrap();
    let mut result = Vec::new();
    let statement = format!(r#"
    select hof.hof,  m.members from (select famid, group_concat(member_name, '{delimiter}') as members
    from members group by famid) as m left join
    (select famid, member_name as hof from members where rltshp = "Self") as hof
    on m.famid = hof.famid;
    "#, delimiter=", ");
    connection.iterate(format!("{}", statement), |row| {
        let family = row.iter().map(
            |x| match x.1 {
                None => "NOt FOUND".to_string(),
                Some(v) => v.to_string()
            }
        ).collect::<Vec<_>>(); // x.0 for selectinn first item in tuple inside list   | ).collect::<Vec<_>>();
        println!("{:?}", family);
        result.push(
            (
            family.get(0).unwrap().to_string(), // hof
            family.get(1).unwrap().to_string(), // members string comma separated
            )
        );
        true
    })
    .unwrap();
    Ok(result)
}

// making above function importable from pytho,n
py_module_initializer!(functions_from_rust_4_kivy, initfunctions_from_rust_4_kivy, PyInit_functions_from_rust_4_kivy, |py, m| {
    m.add(py, "get_family_data_from_db", py_fn!(py, get_family_data_from_db(famid: u32)))
    });

