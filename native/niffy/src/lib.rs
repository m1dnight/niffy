use rustler::{nif, NifStruct};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, NifStruct)]
#[module = "MyOtherStruct"]
struct MyOtherStruct {
    value: Vec<u8>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, NifStruct)]
#[module = "MyStruct"]
pub struct MyStruct {
    value: Vec<u8>,
    other_value: MyOtherStruct,
}

#[nif]
fn test() -> MyStruct {
    MyStruct {
        value: vec![1, 2, 3, 4, 5, 6],
        other_value: MyOtherStruct {value: vec![]}
    }
}
#[nif]
fn test_back(my_struct: MyStruct) -> MyStruct {
    my_struct
}

rustler::init!("Elixir.Niffy.NIF");
