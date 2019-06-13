use dart_vm::*;

register_module!(dart_example_Init, hello);

fn hello(args: Args) -> Value<DartNull> {
  println!("from native");
  Value::create_null()
}
