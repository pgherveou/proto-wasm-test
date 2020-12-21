use protofish::Context;
use std::io::Read;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[wasm_bindgen]
pub fn test_proto() -> String {
    let context = Context::parse(&[r#"
      syntax = "proto3";
      package Proto;
    
      message Request { string kind = 1; }
      message Response { int32 distance = 1; }
      service Fish {
        rpc Swim( Request ) returns ( Response );
      }
    "#])
    .unwrap();

    let service = context.get_service("Proto.Fish").unwrap();
    let rpc = service.rpc_by_name("Swim").unwrap();

    let input = rpc.input.message.decode(b"\x0a\x05Perch", &context);

    format!("{:?} {:?}", input, rpc)
}

pub fn test_proto_load_all() {
    let mut content = Vec::new();
    let g = glob::glob("~/idl/**/*.proto");

    for path in g.unwrap().filter_map(Result::ok) {
        let mut proto_file = String::new();
        std::fs::File::open(&path)
            .and_then(|mut file| file.read_to_string(&mut proto_file))
            .unwrap();

        content.push(proto_file);
    }

    println!("files! . {}", content.len());
    let content_ref: Vec<_> = content.iter().map(|s| s.as_str()).collect();

    let context = Context::parse(&content_ref).unwrap();
    let hello_name = &String::from("pb.lyft.hello.SayHelloRequest");
    let msg = context.get_message(hello_name).unwrap();
    println!("DONE! {}", msg.full_name)
}
