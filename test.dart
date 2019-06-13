import 'dart-ext:dart_example';

Null hello() native 'hello';

main(List<String> args) {
  var result = hello();
  print(result);
}
