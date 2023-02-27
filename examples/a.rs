use vkvk::Entry;

fn main() {
  let entry = Entry::LINKED;
  let _instance_layer_properties = entry.enumerate_instance_layer_properties().unwrap();
}
