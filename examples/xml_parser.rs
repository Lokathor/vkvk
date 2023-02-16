#![allow(unused_mut)]
#![allow(clippy::single_match)]
#![allow(clippy::field_reassign_with_default)]

use std::collections::HashMap;

use magnesium::{XmlElement::*, *};

const XML: &str = include_str!("../vk.xml");

type StaticStr = &'static str;

fn main() {
  let mut iter = ElementIterator::new(XML)
    .filter_map(skip_comments)
    .map(trim_text)
    .filter_map(skip_empty_text_elements);
  assert_eq!(iter.next().unwrap().unwrap_start_tag().0, "registry");
  let registry = Registry::from_iter(&mut iter);
  println!("{registry:#?}");
}

fn attr_hashmap(attrs: StaticStr) -> HashMap<StaticStr, StaticStr> {
  let mut hashmap = HashMap::new();
  for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
    hashmap.insert(key, value);
  }
  hashmap
}

#[derive(Debug, Clone, Default)]
pub struct Registry {
  pub features: Vec<Feature>,
  pub extensions: Vec<Extension>,
  pub formats: Vec<Format>,
  pub spirv_extensions: Vec<SpirvExtension>,
  pub spirv_capabilities: Vec<SpirvCapability>,
}

impl Registry {
  fn from_iter(iter: &mut impl Iterator<Item = XmlElement<'static>>) -> Self {
    let mut registry = Self::default();
    loop {
      match iter.next().unwrap() {
        EndTag { name: "registry" } => return registry,
        StartTag { name: "comment", attrs: _ } => loop {
          if let EndTag { name: "comment" } = iter.next().unwrap() {
            break;
          }
        },
        StartTag { name: "platforms", attrs: _ } => loop {
          if let EndTag { name: "platforms" } = iter.next().unwrap() {
            break;
          }
        },
        StartTag { name: "tags", attrs: _ } => loop {
          if let EndTag { name: "tags" } = iter.next().unwrap() {
            break;
          }
        },
        StartTag { name: "types", attrs: _ } => loop {
          if let EndTag { name: "types" } = iter.next().unwrap() {
            break;
          }
        },
        StartTag { name: "enums", attrs: _ } => loop {
          if let EndTag { name: "enums" } = iter.next().unwrap() {
            break;
          }
        },
        StartTag { name: "commands", attrs: _ } => loop {
          if let EndTag { name: "commands" } = iter.next().unwrap() {
            break;
          }
        },
        StartTag { name: "feature", attrs } => {
          let attr_map = attr_hashmap(attrs);
          let mut feature = Feature { name: attr_map["name"] };
          'feature: loop {
            match iter.next().unwrap() {
              EndTag { name: "feature" } => {
                registry.features.push(feature);
                break 'feature;
              }
              StartTag { name: "require", attrs: _ } => 'require: loop {
                match iter.next().unwrap() {
                  EndTag { name: "require" } => {
                    break 'require;
                  }
                  StartTag { name: "comment", attrs: _ } => loop {
                    if let EndTag { name: "comment" } = iter.next().unwrap() {
                      break;
                    }
                  },
                  EmptyTag { name: "enum", attrs: _ } => (/* TODO */),
                  EmptyTag { name: "type", attrs: _ } => (/* TODO */),
                  EmptyTag { name: "command", attrs: _ } => (/* TODO */),
                  other => panic!("{other:?}"),
                }
              },
              EmptyTag { name: "require", attrs: _ } => (/* TODO */),
              other => panic!("{other:?}"),
            }
          }
        }
        StartTag { name: "extensions", attrs: _ } => loop {
          match iter.next().unwrap() {
            EndTag { name: "extensions" } => break,
            StartTag { name: "extension", attrs } => {
              let attr_map = attr_hashmap(attrs);
              let mut extension = Extension { name: attr_map["name"] };
              loop {
                match iter.next().unwrap() {
                  EndTag { name: "extension" } => {
                    registry.extensions.push(extension);
                    break;
                  }
                  StartTag { name: "require", attrs: _ } => loop {
                    match iter.next().unwrap() {
                      EndTag { name: "require" } => {
                        break;
                      }
                      StartTag { name: "comment", attrs: _ } => loop {
                        if let EndTag { name: "comment" } = iter.next().unwrap() {
                          break;
                        }
                      },
                      EmptyTag { name: "enum", attrs: _ } => (/* TODO */),
                      EmptyTag { name: "type", attrs: _ } => (/* TODO */),
                      EmptyTag { name: "command", attrs: _ } => (/* TODO */),
                      other => panic!("{other:?}"),
                    }
                  },
                  other => panic!("{other:?}"),
                }
              }
            }
            other => panic!("{other:?}"),
          }
        },
        StartTag { name: "formats", attrs: _ } => loop {
          match iter.next().unwrap() {
            EndTag { name: "formats" } => break,
            StartTag { name: "format", attrs } => {
              let attr_map = attr_hashmap(attrs);
              let mut format = Format { name: attr_map["name"] };
              loop {
                match iter.next().unwrap() {
                  EndTag { name: "format" } => {
                    registry.formats.push(format);
                    break;
                  }
                  EmptyTag { name: "component", attrs: _ } => (/*TODO*/),
                  EmptyTag { name: "plane", attrs: _ } => (/*TODO*/),
                  EmptyTag { name: "spirvimageformat", attrs: _ } => (/*TODO*/),
                  other => panic!("{other:?}"),
                }
              }
            }
            other => panic!("{other:?}"),
          }
        },
        StartTag { name: "spirvextensions", attrs: _ } => loop {
          match iter.next().unwrap() {
            EndTag { name: "spirvextensions" } => break,
            StartTag { name: "spirvextension", attrs } => {
              let attr_map = attr_hashmap(attrs);
              let mut spirv_extension = SpirvExtension { name: attr_map["name"] };
              loop {
                match iter.next().unwrap() {
                  EndTag { name: "spirvextension" } => {
                    registry.spirv_extensions.push(spirv_extension);
                    break;
                  }
                  EmptyTag { name: "enable", attrs: _ } => (/*TODO*/),
                  other => panic!("{other:?}"),
                }
              }
            }
            other => panic!("{other:?}"),
          }
        },
        StartTag { name: "spirvcapabilities", attrs: _ } => loop {
          match iter.next().unwrap() {
            EndTag { name: "spirvcapabilities" } => break,
            StartTag { name: "spirvcapability", attrs } => {
              let attr_map = attr_hashmap(attrs);
              let mut spirv_capability = SpirvCapability { name: attr_map["name"] };
              loop {
                match iter.next().unwrap() {
                  EndTag { name: "spirvcapability" } => {
                    registry.spirv_capabilities.push(spirv_capability);
                    break;
                  }
                  EmptyTag { name: "enable", attrs: _ } => (/*TODO*/),
                  other => panic!("{other:?}"),
                }
              }
            }
            other => panic!("{other:?}"),
          }
        },
        other => panic!("{other:?}"),
      }
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct Feature {
  pub name: StaticStr,
}

#[derive(Debug, Clone, Default)]
pub struct Extension {
  pub name: StaticStr,
}

#[derive(Debug, Clone, Default)]
pub struct Format {
  pub name: StaticStr,
}

#[derive(Debug, Clone, Default)]
pub struct SpirvExtension {
  pub name: StaticStr,
}

#[derive(Debug, Clone, Default)]
pub struct SpirvCapability {
  pub name: StaticStr,
}
