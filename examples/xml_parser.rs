#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::single_match)]
#![allow(clippy::field_reassign_with_default)]

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

#[derive(Debug, Clone, Default)]
pub struct Registry {
  pub commands: Vec<Command>,
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
          match iter.next().unwrap() {
            EndTag { name: "commands" } => break,
            StartTag { name: "command", attrs } => {
              let mut command = Command::from_attrs(attrs);
              loop {
                match iter.next().unwrap() {
                  EndTag { name: "command" } => {
                    registry.commands.push(command);
                    break;
                  }
                  StartTag { name: "proto", attrs: _ } => {
                    match iter.next().unwrap() {
                      StartTag { name: "type", attrs: "" } => (),
                      other => panic!("{other:?}"),
                    }
                    match iter.next().unwrap() {
                      Text(return_ty) => command.return_ty = return_ty,
                      other => panic!("{other:?}"),
                    }
                    match iter.next().unwrap() {
                      EndTag { name: "type" } => (),
                      other => panic!("{other:?}"),
                    }
                    match iter.next().unwrap() {
                      StartTag { name: "name", attrs: "" } => (),
                      other => panic!("{other:?}"),
                    }
                    match iter.next().unwrap() {
                      Text(name) => command.name = name,
                      other => panic!("{other:?}"),
                    }
                    match iter.next().unwrap() {
                      EndTag { name: "name" } => (),
                      other => panic!("{other:?}"),
                    }
                    match iter.next().unwrap() {
                      EndTag { name: "proto" } => (),
                      other => panic!("{other:?}"),
                    }
                  }
                  StartTag { name: "param", attrs: _ } => {
                    let ty = {
                      match iter.next().unwrap() {
                        Text("const") => match iter.next().unwrap() {
                          StartTag { name: "type", attrs: "" } => (/* TODO */),
                          other => panic!("{other:?}"),
                        },
                        Text("struct") => match iter.next().unwrap() {
                          StartTag { name: "type", attrs: "" } => (/* TODO */),
                          other => panic!("{other:?}"),
                        },
                        Text("const struct") => match iter.next().unwrap() {
                          StartTag { name: "type", attrs: "" } => (/* TODO */),
                          other => panic!("{other:?}"),
                        },
                        StartTag { name: "type", attrs: "" } => (/* TODO */),
                        other => panic!("{other:?}"),
                      }
                      match iter.next().unwrap() {
                        Text(param_ty) => {
                          match iter.next().unwrap() {
                            EndTag { name: "type" } => (),
                            other => panic!("{other:?}"),
                          };
                          param_ty
                        }
                        other => panic!("{other:?}"),
                      }
                    };
                    let name = {
                      match iter.next().unwrap() {
                        Text("*") | Text("**") | Text("* const*") => match iter.next().unwrap() {
                          StartTag { name: "name", attrs: "" } => (/* TODO */),
                          other => panic!("{other:?}"),
                        },
                        StartTag { name: "name", attrs: "" } => (/* TODO */),
                        other => panic!("{other:?}"),
                      };
                      match iter.next().unwrap() {
                        Text(param_name) => {
                          match iter.next().unwrap() {
                            EndTag { name: "name" } => (),
                            other => panic!("{other:?}"),
                          };
                          match iter.next().unwrap() {
                            Text("[2]") | Text("[4]") => {
                              match iter.next().unwrap() {
                                EndTag { name: "param" } => (),
                                other => panic!("{other:?}"),
                              };
                              /* TODO */
                            }
                            EndTag { name: "param" } => (),
                            other => panic!("{other:?}"),
                          };
                          param_name
                        }
                        other => panic!("{other:?}"),
                      }
                    };
                    command.params.push(CommandParam { name, ty });
                  }
                  StartTag { name: "implicitexternsyncparams", attrs: _ } => loop {
                    match iter.next().unwrap() {
                      EndTag { name: "implicitexternsyncparams" } => break,
                      StartTag { name: "param", attrs: _ } => {
                        match iter.next().unwrap() {
                          Text(t) => command.implicitexternsyncparams.push(t),
                          other => panic!("{other:?}"),
                        }
                        match iter.next().unwrap() {
                          EndTag { name: "param" } => (),
                          other => panic!("{other:?}"),
                        }
                      }
                      other => panic!("{other:?}"),
                    }
                  },
                  other => panic!("{other:?}"),
                }
              }
            }
            EmptyTag { name: "command", attrs } => {
              let command = Command::from_attrs(attrs);
              registry.commands.push(command);
            }
            other => panic!("{other:?}"),
          }
        },
        StartTag { name: "feature", attrs } => {
          let mut feature = Feature::from_attrs(attrs);
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
              let mut extension = Extension::from_attrs(attrs);
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
              let mut format = Format::from_attrs(attrs);
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
              let mut spirv_extension = SpirvExtension::from_attrs(attrs);
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
              let mut spirv_capability = SpirvCapability::from_attrs(attrs);
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
pub struct CommandParam {
  pub name: StaticStr,
  pub ty: StaticStr,
}

#[derive(Debug, Clone, Default)]
pub struct Command {
  pub successcodes: StaticStr,
  pub errorcodes: StaticStr,
  pub queues: StaticStr,
  pub alias: StaticStr,
  pub renderpass: StaticStr,
  pub cmdbufferlevel: StaticStr,
  pub tasks: StaticStr,
  pub comment: StaticStr,
  pub videocoding: StaticStr,
  //
  pub return_ty: StaticStr,
  pub name: StaticStr,
  pub params: Vec<CommandParam>,
  pub implicitexternsyncparams: Vec<StaticStr>,
}
impl Command {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "successcodes" => s.successcodes = value,
        "errorcodes" => s.errorcodes = value,
        "queues" => s.queues = value,
        "alias" => s.alias = value,
        "renderpass" => s.renderpass = value,
        "cmdbufferlevel" => s.cmdbufferlevel = value,
        "tasks" => s.tasks = value,
        "comment" => s.comment = value,
        "videocoding" => s.videocoding = value,
        "name" => s.name = value,
        _ => panic!("{key:?} = {value:?}"),
      }
    }
    s
  }
}

#[derive(Debug, Clone, Default)]
pub struct Feature {
  pub name: StaticStr,
}
impl Feature {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => s.name = value,
        _ => (/* TODO */),
      }
    }
    s
  }
}

#[derive(Debug, Clone, Default)]
pub struct Extension {
  pub name: StaticStr,
}
impl Extension {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => s.name = value,
        _ => (/* TODO */),
      }
    }
    s
  }
}

#[derive(Debug, Clone, Default)]
pub struct Format {
  pub name: StaticStr,
}
impl Format {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => s.name = value,
        _ => (/* TODO */),
      }
    }
    s
  }
}

#[derive(Debug, Clone, Default)]
pub struct SpirvExtension {
  pub name: StaticStr,
}
impl SpirvExtension {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => s.name = value,
        _ => (/* TODO */),
      }
    }
    s
  }
}

#[derive(Debug, Clone, Default)]
pub struct SpirvCapability {
  pub name: StaticStr,
}
impl SpirvCapability {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => s.name = value,
        _ => (/* TODO */),
      }
    }
    s
  }
}
