use ecma335::pe::ManagedPeFile32;
use std::fs;

#[test]
fn from_pe() {
  let pe = fs::read("data/empty.dll").unwrap();
  let pe = ManagedPeFile32::from_data(&pe).unwrap();
  let md = pe.metadata().unwrap();

  let strings = md
    .streams()
    .filter_map(Result::ok)
    .filter_map(|s| s.as_strings())
    .next()
    .unwrap();

  let tables = md
    .streams()
    .filter_map(Result::ok)
    .filter_map(|s| s.as_tables())
    .next()
    .unwrap();

  for td in tables.type_defs() {
    println!("{:?}", strings.get(td.name()));
  }
}
