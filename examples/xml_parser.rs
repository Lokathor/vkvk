#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::single_match)]
#![allow(clippy::match_single_binding)]
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

macro_rules! assert_attrs_comment_only {
  ($attrs:expr) => {
    for TagAttribute { key, value } in TagAttributeIterator::new($attrs) {
      match key {
        "comment" => (),
        _ => panic!("{key:?} = {value:?}"),
      }
    }
  };
}

#[derive(Debug, Clone, Default)]
pub struct Registry {
  pub platforms: Vec<Platform>,
  pub org_tags: Vec<OrgTag>,
  pub types: Vec<TypeEntry>,
  pub enums: Vec<Enumeration>,
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
        StartTag { name: "comment", attrs: "" } => loop {
          if let EndTag { name: "comment" } = iter.next().unwrap() {
            break;
          }
        },
        StartTag { name: "platforms", attrs } => do_platforms(attrs, &mut registry.platforms, iter),
        StartTag { name: "tags", attrs } => do_tags(attrs, &mut registry.org_tags, iter),
        StartTag { name: "types", attrs } => do_types(attrs, &mut registry.types, iter),
        StartTag { name: "enums", attrs } => do_enums(attrs, &mut registry.enums, iter),
        StartTag { name: "commands", attrs } => do_commands(attrs, &mut registry.commands, iter),
        StartTag { name: "feature", attrs } => do_feature(attrs, &mut registry.features, iter),
        StartTag { name: "extensions", attrs } => {
          do_extensions(attrs, &mut registry.extensions, iter)
        }
        StartTag { name: "formats", attrs } => do_formats(attrs, &mut registry.formats, iter),
        StartTag { name: "spirvextensions", attrs } => {
          do_spirv_extensions(attrs, &mut registry.spirv_extensions, iter)
        }
        StartTag { name: "spirvcapabilities", attrs } => {
          do_spirv_capabilities(attrs, &mut registry.spirv_capabilities, iter)
        }
        other => panic!("{other:?}"),
      }
    }
  }
}

fn do_platforms(
  attrs: StaticStr, platforms: &mut Vec<Platform>,
  iter: &mut impl Iterator<Item = XmlElement<'static>>,
) {
  assert_attrs_comment_only!(attrs);
  loop {
    match iter.next().unwrap() {
      EndTag { name: "platforms" } => return,
      EmptyTag { name: "platform", attrs } => {
        platforms.push(Platform::from_attrs(attrs));
      }
      other => panic!("{other:?}"),
    }
  }
}

fn do_tags(
  attrs: StaticStr, org_tags: &mut Vec<OrgTag>,
  iter: &mut impl Iterator<Item = XmlElement<'static>>,
) {
  assert_attrs_comment_only!(attrs);
  loop {
    match iter.next().unwrap() {
      EndTag { name: "tags" } => return,
      EmptyTag { name: "tag", attrs } => {
        org_tags.push(OrgTag::from_attrs(attrs));
      }
      other => panic!("{other:?}"),
    }
  }
}

fn do_types(
  attrs: StaticStr, types: &mut Vec<TypeEntry>,
  iter: &mut impl Iterator<Item = XmlElement<'static>>,
) {
  assert_attrs_comment_only!(attrs);
  loop {
    match iter.next().unwrap() {
      EndTag { name: "types" } => return,
      StartTag { name: "type", attrs } => {
        let mut ty_entry = TypeEntry::from_attrs(attrs);
        'ty: loop {
          match iter.next().unwrap() {
            EndTag { name: "type" } => break 'ty,
            Text(t) => ty_entry.texts.push(t),
            StartTag { name: "comment", attrs: "" } => {
              ty_entry.comment = iter.next().unwrap().unwrap_text();
              assert_eq!(iter.next().unwrap().unwrap_end_tag(), "comment");
            }
            StartTag { name: "name", attrs: "" } => {
              assert!(ty_entry.name.is_empty());
              ty_entry.name = iter.next().unwrap().unwrap_text();
              assert_eq!(iter.next().unwrap().unwrap_end_tag(), "name");
              ty_entry.texts.push(ty_entry.name);
            }
            StartTag { name: "type", attrs: "" } => {
              ty_entry.ty_src = iter.next().unwrap().unwrap_text();
              assert_eq!(iter.next().unwrap().unwrap_end_tag(), "type");
              ty_entry.texts.push(ty_entry.ty_src);
            }
            StartTag { name: "member", attrs } => {
              let mut member = Member::from_attrs(attrs);
              match iter.next().unwrap() {
                // If we see the keyword `struct` as part of the type's
                // prefix we ignore it. That's a C way to declare an
                // opaque type inline with the function signature, which
                // we don't do in Rust anyway.
                Text("const") | Text("const struct") => {
                  member.ty_variant = TypeVariant::ConstPtr;
                  match iter.next().unwrap() {
                    StartTag { name: "type", attrs: "" } => (),
                    other => panic!("{other:?}"),
                  }
                }
                Text("struct") => match iter.next().unwrap() {
                  StartTag { name: "type", attrs: "" } => (),
                  other => panic!("{other:?}"),
                },
                StartTag { name: "type", attrs: "" } => (),
                other => panic!("{other:?}"),
              }
              member.ty = match iter.next().unwrap().unwrap_text() {
                "char" => "u8",
                "uint32_t" => "u32",
                other => other,
              };
              assert_eq!(iter.next().unwrap().unwrap_end_tag(), "type");

              match iter.next().unwrap() {
                Text("*") => {
                  match member.ty_variant {
                    TypeVariant::Normal => member.ty_variant = TypeVariant::MutPtr,
                    TypeVariant::ConstPtr => (/* nothing */),
                    other => panic!("{other:?}"),
                  }
                  match iter.next().unwrap() {
                    StartTag { name: "name", attrs: "" } => (),
                    other => panic!("{other:?}"),
                  }
                }
                Text("**") => {
                  match member.ty_variant {
                    TypeVariant::Normal => member.ty_variant = TypeVariant::MutPtrMutPtr,
                    TypeVariant::ConstPtr => member.ty_variant = TypeVariant::MutPtrConstPtr,
                    other => panic!("{other:?}"),
                  }
                  match iter.next().unwrap() {
                    StartTag { name: "name", attrs: "" } => (),
                    other => panic!("{other:?}"),
                  }
                }
                Text("* const*") | Text("* const *") => {
                  match member.ty_variant {
                    TypeVariant::ConstPtr => member.ty_variant = TypeVariant::ConstPtrConstPtr,
                    other => panic!("{other:?}"),
                  }
                  match iter.next().unwrap() {
                    StartTag { name: "name", attrs: "" } => (),
                    other => panic!("{other:?}"),
                  }
                }
                StartTag { name: "name", attrs: "" } => (),
                other => panic!("{other:?}"),
              };
              member.name = iter.next().unwrap().unwrap_text();
              assert_eq!(iter.next().unwrap().unwrap_end_tag(), "name");
              loop {
                match iter.next().unwrap() {
                  EndTag { name: "member" } => break,
                  Text("[") => {
                    assert_eq!(iter.next().unwrap().unwrap_start_tag(), ("enum", ""));
                    let arr_len = iter.next().unwrap().unwrap_text();
                    match member.ty_variant {
                      TypeVariant::Normal => {
                        member.ty_variant = TypeVariant::ConstArrayPtrNamed(arr_len)
                      }
                      other => panic!("{other:?}"),
                    }
                    assert_eq!(iter.next().unwrap().unwrap_end_tag(), "enum");
                    assert_eq!(iter.next().unwrap().unwrap_text(), "]");
                  }
                  Text("[2]") => match member.ty_variant {
                    TypeVariant::Normal => member.ty_variant = TypeVariant::ArrayLit(2),
                    other => panic!("{other:?}"),
                  },
                  Text("[3]") => match member.ty_variant {
                    TypeVariant::Normal => member.ty_variant = TypeVariant::ArrayLit(3),
                    other => panic!("{other:?}"),
                  },
                  Text("[4]") => match member.ty_variant {
                    TypeVariant::Normal => member.ty_variant = TypeVariant::ArrayLit(4),
                    other => panic!("{other:?}"),
                  },
                  Text("[3][4]") => match member.ty_variant {
                    TypeVariant::Normal => member.ty_variant = TypeVariant::ArrayOfArrayLit(3, 4),
                    other => panic!("{other:?}"),
                  },
                  Text(":8") => match member.ty_variant {
                    TypeVariant::Normal => member.ty_variant = TypeVariant::BitfieldsLit(8),
                    other => panic!("{other:?}"),
                  },
                  Text(":24") => match member.ty_variant {
                    TypeVariant::Normal => member.ty_variant = TypeVariant::BitfieldsLit(24),
                    other => panic!("{other:?}"),
                  },
                  StartTag { name: "comment", attrs: "" } => {
                    assert!(member.comment.is_empty());
                    member.comment = iter.next().unwrap().unwrap_text();
                    assert_eq!(iter.next().unwrap().unwrap_end_tag(), "comment");
                  }
                  other => panic!("{other:?}"),
                }
              }
              ty_entry.members.push(member);
            }
            other => panic!("{other:?}"),
          }
        }
        types.push(ty_entry);
      }
      EmptyTag { name: "type", attrs } => {
        types.push(TypeEntry::from_attrs(attrs));
      }
      StartTag { name: "comment", attrs: "" } => {
        iter.next().unwrap().unwrap_text();
        assert_eq!(iter.next().unwrap().unwrap_end_tag(), "comment");
      }
      other => panic!("{other:?}"),
    }
  }
}

fn do_enums(
  attrs: StaticStr, enums: &mut Vec<Enumeration>,
  iter: &mut impl Iterator<Item = XmlElement<'static>>,
) {
  let mut enumeration = Enumeration::from_attrs(attrs);
  loop {
    match iter.next().unwrap() {
      EndTag { name: "enums" } => {
        enums.push(enumeration);
        return;
      }
      EmptyTag { name: "enum", attrs } => {
        enumeration.entries.push(EnumerationEntry::from_attrs(attrs))
      }
      _ => (),
    }
  }
}

fn do_commands(
  attrs: StaticStr, commands: &mut Vec<Command>,
  iter: &mut impl Iterator<Item = XmlElement<'static>>,
) {
  assert_attrs_comment_only!(attrs);
  loop {
    match iter.next().unwrap() {
      EndTag { name: "commands" } => return,
      StartTag { name: "command", attrs } => {
        let mut command = Command::from_attrs(attrs);
        'command: loop {
          match iter.next().unwrap() {
            EndTag { name: "command" } => {
              break 'command;
            }
            StartTag { name: "proto", attrs: "" } => {
              assert_eq!(iter.next().unwrap().unwrap_start_tag(), ("type", ""));
              command.return_ty = iter.next().unwrap().unwrap_text();
              if command.return_ty == "void" {
                command.return_ty = "()";
              }
              assert_eq!(iter.next().unwrap().unwrap_end_tag(), "type");
              //
              assert_eq!(iter.next().unwrap().unwrap_start_tag(), ("name", ""));
              command.name = iter.next().unwrap().unwrap_text();
              assert_eq!(iter.next().unwrap().unwrap_end_tag(), "name");
              //
              assert_eq!(iter.next().unwrap().unwrap_end_tag(), "proto");
            }
            StartTag { name: "param", attrs } => {
              let mut command_param = CommandParam::from_attrs(attrs);
              match iter.next().unwrap() {
                // If we see the keyword `struct` as part of the type's
                // prefix we ignore it. That's a C way to declare an
                // opaque type inline with the function signature, which
                // we don't do in Rust anyway.
                Text("const") | Text("const struct") => {
                  command_param.ty_variant = TypeVariant::ConstPtr;
                  match iter.next().unwrap() {
                    StartTag { name: "type", attrs: "" } => (),
                    other => panic!("{other:?}"),
                  }
                }
                Text("struct") => match iter.next().unwrap() {
                  StartTag { name: "type", attrs: "" } => (),
                  other => panic!("{other:?}"),
                },
                StartTag { name: "type", attrs: "" } => (),
                other => panic!("{other:?}"),
              }
              command_param.ty = match iter.next().unwrap().unwrap_text() {
                "char" => "u8",
                "uint32_t" => "u32",
                other => other,
              };
              assert_eq!(iter.next().unwrap().unwrap_end_tag(), "type");
              match iter.next().unwrap() {
                Text("*") => {
                  match command_param.ty_variant {
                    TypeVariant::Normal => command_param.ty_variant = TypeVariant::MutPtr,
                    TypeVariant::ConstPtr => (/* nothing */),
                    other => panic!("{other:?}"),
                  }
                  match iter.next().unwrap() {
                    StartTag { name: "name", attrs: "" } => (),
                    other => panic!("{other:?}"),
                  }
                }
                Text("**") => {
                  match command_param.ty_variant {
                    TypeVariant::Normal => command_param.ty_variant = TypeVariant::MutPtrMutPtr,
                    TypeVariant::ConstPtr => command_param.ty_variant = TypeVariant::MutPtrConstPtr,
                    other => panic!("{other:?}"),
                  }
                  match iter.next().unwrap() {
                    StartTag { name: "name", attrs: "" } => (),
                    other => panic!("{other:?}"),
                  }
                }
                Text("* const*") => {
                  match command_param.ty_variant {
                    TypeVariant::ConstPtr => {
                      command_param.ty_variant = TypeVariant::ConstPtrConstPtr
                    }
                    other => panic!("{other:?}"),
                  }
                  match iter.next().unwrap() {
                    StartTag { name: "name", attrs: "" } => (),
                    other => panic!("{other:?}"),
                  }
                }
                StartTag { name: "name", attrs: "" } => (),
                other => panic!("{other:?}"),
              };
              match iter.next().unwrap() {
                Text(param_name) => {
                  command_param.name = param_name;
                  assert_eq!(iter.next().unwrap().unwrap_end_tag(), "name");
                  match iter.next().unwrap() {
                    Text(t) => {
                      match (t, command_param.ty_variant) {
                        ("[2]", TypeVariant::ConstPtr) => {
                          command_param.ty_variant = TypeVariant::ConstArrayPtrLit(2)
                        }
                        ("[4]", TypeVariant::ConstPtr) => {
                          command_param.ty_variant = TypeVariant::ConstArrayPtrLit(4)
                        }
                        other => panic!("{other:?}"),
                      }
                      assert_eq!(iter.next().unwrap().unwrap_end_tag(), "param");
                    }
                    EndTag { name: "param" } => (),
                    other => panic!("{other:?}"),
                  };
                }
                other => panic!("{other:?}"),
              }
              command.params.push(command_param);
            }
            StartTag { name: "implicitexternsyncparams", attrs: "" } => loop {
              match iter.next().unwrap() {
                EndTag { name: "implicitexternsyncparams" } => break,
                StartTag { name: "param", attrs: "" } => {
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
        commands.push(command);
      }
      other => panic!("{other:?}"),
    }
  }
}

fn do_feature(
  attrs: StaticStr, features: &mut Vec<Feature>,
  iter: &mut impl Iterator<Item = XmlElement<'static>>,
) {
  let mut feature = Feature::from_attrs(attrs);
  loop {
    match iter.next().unwrap() {
      EndTag { name: "feature" } => {
        features.push(feature);
        return;
      }
      StartTag { name: "require", attrs } => {
        assert_attrs_comment_only!(attrs);
        'require: loop {
          match iter.next().unwrap() {
            EndTag { name: "require" } => {
              break 'require;
            }
            StartTag { name: "comment", attrs: "" } => loop {
              if let EndTag { name: "comment" } = iter.next().unwrap() {
                break;
              }
            },
            EmptyTag { name: "enum", attrs } => {
              feature.required_enums.push(RequiredEnum::from_attrs(attrs));
            }
            EmptyTag { name: "type", attrs } => {
              for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
                match key {
                  "name" => feature.required_types.push(value),
                  "comment" => (),
                  _ => panic!("{key:?} = {value:?}"),
                }
              }
            }
            EmptyTag { name: "command", attrs } => {
              for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
                match key {
                  "name" => feature.required_commands.push(value),
                  "comment" => (),
                  _ => panic!("{key:?} = {value:?}"),
                }
              }
            }
            other => panic!("{other:?}"),
          }
        }
      }
      StartTag { name: "remove", attrs } => {
        assert_attrs_comment_only!(attrs);
        'require: loop {
          match iter.next().unwrap() {
            EndTag { name: "remove" } => {
              break 'require;
            }
            StartTag { name: "comment", attrs: "" } => loop {
              if let EndTag { name: "comment" } = iter.next().unwrap() {
                break;
              }
            },
            EmptyTag { name: "enum", attrs } => {
              for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
                match key {
                  "name" => feature.removed_enums.push(value),
                  "comment" => (),
                  _ => panic!("{key:?} = {value:?}"),
                }
              }
            }
            EmptyTag { name: "type", attrs } => {
              for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
                match key {
                  "name" => feature.removed_types.push(value),
                  "comment" => (),
                  _ => panic!("{key:?} = {value:?}"),
                }
              }
            }
            EmptyTag { name: "command", attrs } => {
              for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
                match key {
                  "name" => feature.removed_commands.push(value),
                  "comment" => (),
                  _ => panic!("{key:?} = {value:?}"),
                }
              }
            }
            other => panic!("{other:?}"),
          }
        }
      }
      EmptyTag { name: "require", attrs } => {
        assert_attrs_comment_only!(attrs)
      }
      other => panic!("{other:?}"),
    }
  }
}

fn do_extensions(
  attrs: StaticStr, extensions: &mut Vec<Extension>,
  iter: &mut impl Iterator<Item = XmlElement<'static>>,
) {
  assert_attrs_comment_only!(attrs);
  loop {
    match iter.next().unwrap() {
      EndTag { name: "extensions" } => return,
      StartTag { name: "extension", attrs } => {
        let mut extension = Extension::from_attrs(attrs);
        loop {
          match iter.next().unwrap() {
            EndTag { name: "extension" } => {
              extensions.push(extension);
              break;
            }
            StartTag { name: "require", attrs } => {
              let mut require_list = RequireListEntry::from_attrs(attrs);
              'require: loop {
                match iter.next().unwrap() {
                  EndTag { name: "require" } => {
                    extension.require_lists.push(require_list);
                    break 'require;
                  }
                  StartTag { name: "comment", attrs: "" } => loop {
                    if let EndTag { name: "comment" } = iter.next().unwrap() {
                      break;
                    }
                  },
                  EmptyTag { name: "enum", attrs } => {
                    require_list.enums.push(RequiredEnum::from_attrs(attrs));
                  }
                  EmptyTag { name: "type", attrs } => {
                    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
                      match key {
                        "name" => require_list.types.push(value),
                        "comment" => (),
                        _ => panic!("{key:?} = {value:?}"),
                      }
                    }
                  }
                  EmptyTag { name: "command", attrs } => {
                    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
                      match key {
                        "name" => require_list.commands.push(value),
                        "comment" => (),
                        _ => panic!("{key:?} = {value:?}"),
                      }
                    }
                  }
                  other => panic!("{other:?}"),
                }
              }
            }
            other => panic!("{other:?}"),
          }
        }
      }
      other => panic!("{other:?}"),
    }
  }
}

fn do_formats(
  attrs: StaticStr, formats: &mut Vec<Format>, iter: &mut impl Iterator<Item = XmlElement<'static>>,
) {
  assert_attrs_comment_only!(attrs);
  loop {
    match iter.next().unwrap() {
      EndTag { name: "formats" } => return,
      StartTag { name: "format", attrs } => {
        let mut format = Format::from_attrs(attrs);
        loop {
          match iter.next().unwrap() {
            EndTag { name: "format" } => {
              formats.push(format);
              break;
            }
            EmptyTag { name: "component", attrs } => {
              format.components.push(FormatComponent::from_attrs(attrs));
            }
            EmptyTag { name: "plane", attrs } => {
              format.planes.push(FormatPlane::from_attrs(attrs));
            }
            EmptyTag { name: "spirvimageformat", attrs } => {
              for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
                match key {
                  "name" => {
                    assert!(format.spirv_image_format.is_empty());
                    format.spirv_image_format = value;
                  }
                  _ => panic!("{key:?} = {value:?}"),
                }
              }
            }
            other => panic!("{other:?}"),
          }
        }
      }
      other => panic!("{other:?}"),
    }
  }
}

fn do_spirv_extensions(
  attrs: StaticStr, spirv_extensions: &mut Vec<SpirvExtension>,
  iter: &mut impl Iterator<Item = XmlElement<'static>>,
) {
  assert_attrs_comment_only!(attrs);
  loop {
    match iter.next().unwrap() {
      EndTag { name: "spirvextensions" } => return,
      StartTag { name: "spirvextension", attrs } => {
        let mut spirv_extension = SpirvExtension::from_attrs(attrs);
        'spirvextension: loop {
          match iter.next().unwrap() {
            EndTag { name: "spirvextension" } => {
              spirv_extensions.push(spirv_extension);
              break 'spirvextension;
            }
            EmptyTag { name: "enable", attrs } => {
              spirv_extension.enables.push(SpirvExtensionEnable::from_attrs(attrs));
            }
            other => panic!("{other:?}"),
          }
        }
      }
      other => panic!("{other:?}"),
    }
  }
}

fn do_spirv_capabilities(
  attrs: StaticStr, spirv_capabilities: &mut Vec<SpirvCapability>,
  iter: &mut impl Iterator<Item = XmlElement<'static>>,
) {
  assert_attrs_comment_only!(attrs);
  loop {
    match iter.next().unwrap() {
      EndTag { name: "spirvcapabilities" } => return,
      StartTag { name: "spirvcapability", attrs } => {
        let mut spirv_capability = SpirvCapability::from_attrs(attrs);
        'spirvcapability: loop {
          match iter.next().unwrap() {
            EndTag { name: "spirvcapability" } => {
              spirv_capabilities.push(spirv_capability);
              break 'spirvcapability;
            }
            EmptyTag { name: "enable", attrs } => {
              spirv_capability.enables.push(SpirvCapabilityEnable::from_attrs(attrs))
            }
            other => panic!("{other:?}"),
          }
        }
      }
      other => panic!("{other:?}"),
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct Platform {
  pub name: StaticStr,
  pub protect: StaticStr,
  pub comment: StaticStr,
}
impl Platform {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => s.name = value,
        "protect" => s.protect = value,
        "comment" => s.comment = value,
        _ => panic!("{key:?} = {value:?}"),
      }
    }
    s
  }
}

/// Organization Tag
#[derive(Debug, Clone, Default)]
pub struct OrgTag {
  pub name: StaticStr,
  pub author: StaticStr,
  pub contact: StaticStr,
}
impl OrgTag {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => s.name = value,
        "author" => s.author = value,
        "contact" => s.contact = value,
        _ => panic!("{key:?} = {value:?}"),
      }
    }
    s
  }
}

#[derive(Debug, Clone, Default)]
pub struct Member {
  pub name: StaticStr,
  pub ty_variant: TypeVariant,
  pub ty: StaticStr,
  //
  pub comment: StaticStr,
  pub optional: StaticStr,
  pub no_auto_validity: StaticStr,
  pub limit_type: StaticStr,
  pub values: StaticStr,
  pub len: StaticStr,
  pub alt_len: StaticStr,
  pub object_type: StaticStr,
  pub extern_sync: StaticStr,
  pub selection: StaticStr,
  pub selector: StaticStr,
  pub deprecated: StaticStr,
  pub api: StaticStr,
}
impl Member {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "optional" => s.optional = value,
        "noautovalidity" => s.no_auto_validity = value,
        "limittype" => s.limit_type = value,
        "values" => s.values = value,
        "len" => s.len = value,
        "altlen" => s.alt_len = value,
        "objecttype" => s.object_type = value,
        "externsync" => s.extern_sync = value,
        "selection" => s.selection = value,
        "selector" => s.selector = value,
        "deprecated" => s.deprecated = value,
        "api" => s.api = value,
        _ => panic!("{key:?} = {value:?}"),
      }
    }
    s
  }
}

#[derive(Debug, Clone, Default)]
pub struct TypeEntry {
  pub name: StaticStr,
  pub category: StaticStr,
  pub texts: Vec<StaticStr>,
  pub comment: StaticStr,
  pub requires: StaticStr,
  pub ty_src: StaticStr,
  pub alias: StaticStr,
  pub bit_values: StaticStr,
  pub obj_type_enum: StaticStr,
  pub parent: StaticStr,
  pub returned_only: StaticStr,
  pub members: Vec<Member>,
  pub struct_extends: StaticStr,
  pub allow_duplicate: StaticStr,
  pub deprecated: StaticStr,
  pub api: StaticStr,
}
impl TypeEntry {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => s.name = value,
        "category" => s.category = value,
        "requires" => s.requires = value,
        "alias" => s.alias = value,
        "bitvalues" => s.bit_values = value,
        "objtypeenum" => s.obj_type_enum = value,
        "parent" => s.parent = value,
        "returnedonly" => s.returned_only = value,
        "structextends" => s.struct_extends = value,
        "comment" => s.comment = value,
        "allowduplicate" => s.allow_duplicate = value,
        "deprecated" => s.deprecated = value,
        "api" => s.api = value,
        _ => panic!("{key:?} = {value:?}"),
      }
    }
    s
  }
}

#[derive(Debug, Clone, Default)]
pub struct EnumerationEntry {
  pub name: StaticStr,
  pub value: StaticStr,
  pub comment: StaticStr,
  pub ty: StaticStr,
  pub alias: StaticStr,
  pub bitpos: StaticStr,
  pub api: StaticStr,
  pub deprecated: StaticStr,
}
impl EnumerationEntry {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => s.name = value,
        "value" => s.value = value,
        "comment" => s.comment = value,
        "type" => s.ty = value,
        "alias" => s.alias = value,
        "bitpos" => s.bitpos = value,
        "api" => s.api = value,
        "deprecated" => s.deprecated = value,
        _ => panic!("{key:?} = {value:?}"),
      }
    }
    s
  }
}

#[derive(Debug, Clone, Default)]
pub struct Enumeration {
  pub name: StaticStr,
  pub ty: StaticStr,
  pub comment: StaticStr,
  pub bitwidth: StaticStr,
  pub entries: Vec<EnumerationEntry>,
}
impl Enumeration {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => s.name = value,
        "type" => s.ty = value,
        "comment" => s.comment = value,
        "bitwidth" => s.bitwidth = value,
        _ => panic!("{key:?} = {value:?}"),
      }
    }
    s
  }
}

#[derive(Debug, Clone, Default)]
pub enum TypeVariant {
  /// `T`
  #[default]
  Normal,
  /// `*const T`
  ConstPtr,
  /// `*mut T`
  MutPtr,
  /// `*mut *mut T`
  MutPtrMutPtr,
  /// `*const [T; N]`
  ConstArrayPtrLit(usize),
  /// `*const [T; NAMED_CONSTANT]`
  ConstArrayPtrNamed(StaticStr),
  /// `*mut *const T`
  MutPtrConstPtr,
  /// `*const *const T`
  ConstPtrConstPtr,
  /// `[T; N]`
  ArrayLit(usize),
  /// `[[T; A]; B]`
  ArrayOfArrayLit(usize, usize),
  /// `:N`
  BitfieldsLit(usize),
}

#[derive(Clone, Default)]
pub struct CommandParam {
  pub name: StaticStr,
  pub ty_variant: TypeVariant,
  pub ty: StaticStr,
  //
  pub optional: StaticStr,
  pub extern_sync: StaticStr,
  pub len: StaticStr,
  pub alt_len: StaticStr,
  pub no_auto_validity: StaticStr,
  pub stride: StaticStr,
  pub object_type: StaticStr,
  pub valid_structs: StaticStr,
  pub api: StaticStr,
}
impl core::fmt::Debug for CommandParam {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let name = self.name;
    let ty = self.ty;
    match self.ty_variant {
      TypeVariant::Normal => write!(f, "{name}: {ty}"),
      TypeVariant::ConstPtr => write!(f, "{name}: *const {ty}"),
      TypeVariant::MutPtr => write!(f, "{name}: *mut {ty}"),
      TypeVariant::MutPtrMutPtr => write!(f, "{name}: *mut *mut {ty}"),
      TypeVariant::ConstArrayPtrLit(n) => write!(f, "{name}: *const [{ty}; {n}]"),
      TypeVariant::ConstArrayPtrNamed(n) => write!(f, "{name}: *const [{ty}; {n}]"),
      TypeVariant::MutPtrConstPtr => write!(f, "{name}: *mut *const {ty}"),
      TypeVariant::ConstPtrConstPtr => write!(f, "{name}: *const *const {ty}"),
      TypeVariant::ArrayLit(n) => write!(f, "{name}: [{ty}; {n}]"),
      TypeVariant::ArrayOfArrayLit(a, b) => write!(f, "{name}: [[{ty}; {a}]; {b}]"),
      TypeVariant::BitfieldsLit(n) => write!(f, "{name}: {ty}{{:{n}}}"),
    }?;
    match self.optional {
      "" => (),
      "true" => write!(f, " /* optional */")?,
      other => write!(f, " /* optional={other:?} */")?,
    }
    match self.extern_sync {
      "" => (),
      "true" => write!(f, " /* extern_sync */")?,
      other => write!(f, " /* extern_sync={other:?} */")?,
    }
    if !self.len.is_empty() {
      write!(f, " /* len={} */", self.len)?;
    }
    if !self.alt_len.is_empty() {
      write!(f, " /* alt_len={} */", self.alt_len)?;
    }
    match self.no_auto_validity {
      "" => (),
      "true" => write!(f, " /* no_auto_validity */")?,
      other => write!(f, " /* no_auto_validity={other:?} */")?,
    }
    if !self.stride.is_empty() {
      write!(f, " /* stride={} */", self.stride)?;
    }
    if !self.object_type.is_empty() {
      write!(f, " /* object_type={} */", self.object_type)?;
    }
    if !self.valid_structs.is_empty() {
      write!(f, " /* valid_structs={} */", self.valid_structs)?;
    }
    Ok(())
  }
}
impl CommandParam {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "len" => s.len = value,
        "altlen" => s.alt_len = value,
        "stride" => s.stride = value,
        "optional" => s.optional = value,
        "externsync" => s.extern_sync = value,
        "objecttype" => s.object_type = value,
        "noautovalidity" => s.no_auto_validity = value,
        "validstructs" => s.valid_structs = value,
        "api" => s.api = value,
        _ => panic!("{key:?} = {value:?}"),
      }
    }
    s
  }
}

#[derive(Debug, Clone, Default)]
pub struct Command {
  pub name: StaticStr,
  pub params: Vec<CommandParam>,
  pub return_ty: StaticStr,
  //
  pub comment: StaticStr,
  pub success_codes: StaticStr,
  pub error_codes: StaticStr,
  pub queues: StaticStr,
  pub alias: StaticStr,
  pub render_pass: StaticStr,
  pub cmd_buffer_level: StaticStr,
  pub tasks: StaticStr,
  pub video_coding: StaticStr,
  pub implicitexternsyncparams: Vec<StaticStr>,
  pub api: StaticStr,
}
impl Command {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "successcodes" => s.success_codes = value,
        "errorcodes" => s.error_codes = value,
        "queues" => s.queues = value,
        "alias" => s.alias = value,
        "renderpass" => s.render_pass = value,
        "cmdbufferlevel" => s.cmd_buffer_level = value,
        "tasks" => s.tasks = value,
        "comment" => s.comment = value,
        "videocoding" => s.video_coding = value,
        "name" => s.name = value,
        "api" => s.api = value,
        _ => panic!("{key:?} = {value:?}"),
      }
    }
    s
  }
}

#[derive(Debug, Clone, Default)]
pub struct Feature {
  pub name: StaticStr,
  pub api: StaticStr,
  pub number: StaticStr,
  pub comment: StaticStr,
  pub required_commands: Vec<StaticStr>,
  pub required_types: Vec<StaticStr>,
  pub required_enums: Vec<RequiredEnum>,
  pub removed_commands: Vec<StaticStr>,
  pub removed_types: Vec<StaticStr>,
  pub removed_enums: Vec<StaticStr>,
}
impl Feature {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => s.name = value,
        "api" => s.api = value,
        "number" => s.number = value,
        "comment" => s.comment = value,
        _ => panic!("{key:?} = {value:?}"),
      }
    }
    s
  }
}

#[derive(Debug, Clone, Default)]
pub struct RequiredEnum {
  pub name: StaticStr,
  pub value: StaticStr,
  pub offset: StaticStr,
  pub extends: StaticStr,
  pub dir: StaticStr,
  pub extnumber: StaticStr,
  pub bitpos: StaticStr,
  pub comment: StaticStr,
  pub alias: StaticStr,
  pub deprecated: StaticStr,
  pub api: StaticStr,
  pub protect: StaticStr,
}
impl RequiredEnum {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => s.name = value,
        "dir" => s.dir = value,
        "value" => s.value = value,
        "alias" => s.alias = value,
        "bitpos" => s.bitpos = value,
        "offset" => s.offset = value,
        "extends" => s.extends = value,
        "comment" => s.comment = value,
        "extnumber" => s.extnumber = value,
        "deprecated" => s.deprecated = value,
        "api" => s.api = value,
        "protect" => s.protect = value,
        _ => panic!("{key:?} = {value:?}"),
      }
    }
    s
  }
}

#[derive(Debug, Clone, Default)]
pub struct RequireListEntry {
  pub enums: Vec<RequiredEnum>,
  pub types: Vec<StaticStr>,
  pub commands: Vec<StaticStr>,
  pub depends: StaticStr,
  pub api: StaticStr,
  pub comment: StaticStr,
}
impl RequireListEntry {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "depends" => s.depends = value,
        "api" => s.api = value,
        "comment" => s.comment = value,
        _ => panic!("{key:?} = {value:?}"),
      }
    }
    s
  }
}

#[derive(Debug, Clone, Default)]
pub struct Extension {
  pub name: StaticStr,
  pub requires: StaticStr,
  pub comment: StaticStr,
  pub number: StaticStr,
  pub ty: StaticStr,
  pub author: StaticStr,
  pub contact: StaticStr,
  pub supported: StaticStr,
  pub platform: StaticStr,
  pub special_use: StaticStr,
  pub deprecated_by: StaticStr,
  pub promoted_to: StaticStr,
  pub requires_core: StaticStr,
  pub obsoleted_by: StaticStr,
  pub provisional: StaticStr,
  pub sort_order: StaticStr,
  pub depends: StaticStr,
  pub require_lists: Vec<RequireListEntry>,
}
impl Extension {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => s.name = value,
        "type" => s.ty = value,
        "number" => s.number = value,
        "author" => s.author = value,
        "contact" => s.contact = value,
        "comment" => s.comment = value,
        "requires" => s.requires = value,
        "platform" => s.platform = value,
        "sortorder" => s.sort_order = value,
        "supported" => s.supported = value,
        "specialuse" => s.special_use = value,
        "promotedto" => s.promoted_to = value,
        "provisional" => s.provisional = value,
        "obsoletedby" => s.obsoleted_by = value,
        "deprecatedby" => s.deprecated_by = value,
        "requiresCore" => s.requires_core = value,
        "depends" => s.depends = value,
        _ => panic!("{key:?} = {value:?}"),
      }
    }
    s
  }
}

#[derive(Debug, Clone, Default)]
pub struct FormatComponent {
  pub name: StaticStr,
  pub plane_index: StaticStr,
  pub bits: StaticStr,
  pub numeric_format: StaticStr,
}
impl FormatComponent {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => s.name = value,
        "planeIndex" => s.plane_index = value,
        "bits" => s.bits = value,
        "numericFormat" => s.numeric_format = value,
        _ => panic!("{key:?} = {value:?}"),
      }
    }
    s
  }
}

#[derive(Debug, Clone, Default)]
pub struct FormatPlane {
  pub index: StaticStr,
  pub width_divisor: StaticStr,
  pub height_divisor: StaticStr,
  pub compatible: StaticStr,
}
impl FormatPlane {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "index" => s.index = value,
        "widthDivisor" => s.width_divisor = value,
        "heightDivisor" => s.height_divisor = value,
        "compatible" => s.compatible = value,
        _ => panic!("{key:?} = {value:?}"),
      }
    }
    s
  }
}

#[derive(Debug, Clone, Default)]
pub struct Format {
  pub name: StaticStr,
  pub class: StaticStr,
  pub block_size: StaticStr,
  pub texels_per_block: StaticStr,
  pub packed: StaticStr,
  pub block_extent: StaticStr,
  pub compressed: StaticStr,
  pub chroma: StaticStr,
  pub spirv_image_format: StaticStr,
  pub components: Vec<FormatComponent>,
  pub planes: Vec<FormatPlane>,
}
impl Format {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => s.name = value,
        "class" => s.class = value,
        "blockSize" => s.block_size = value,
        "texelsPerBlock" => s.texels_per_block = value,
        "packed" => s.packed = value,
        "blockExtent" => s.block_extent = value,
        "compressed" => s.compressed = value,
        "chroma" => s.chroma = value,
        _ => panic!("{key:?} = {value:?}"),
      }
    }
    s
  }
}

#[derive(Debug, Clone, Default)]
pub struct SpirvExtensionEnable {
  pub version: StaticStr,
  pub extension: StaticStr,
}
impl SpirvExtensionEnable {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "version" => s.version = value,
        "extension" => s.extension = value,
        _ => panic!("{key:?} = {value:?}"),
      }
    }
    s
  }
}

#[derive(Debug, Clone, Default)]
pub struct SpirvExtension {
  pub name: StaticStr,
  pub enables: Vec<SpirvExtensionEnable>,
}
impl SpirvExtension {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => s.name = value,
        _ => panic!("{key:?} = {value:?}"),
      }
    }
    s
  }
}

#[derive(Debug, Clone, Default)]
pub struct SpirvCapabilityEnable {
  pub struct_: StaticStr,
  pub feature: StaticStr,
  pub requires: StaticStr,
  pub version: StaticStr,
  pub extension: StaticStr,
  pub property: StaticStr,
  pub member: StaticStr,
  pub value: StaticStr,
  pub alias: StaticStr,
}
impl SpirvCapabilityEnable {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "struct" => s.struct_ = value,
        "feature" => s.feature = value,
        "requires" => s.requires = value,
        "version" => s.version = value,
        "extension" => s.extension = value,
        "property" => s.property = value,
        "member" => s.member = value,
        "value" => s.value = value,
        "alias" => s.alias = value,
        _ => panic!("{key:?} = {value:?}"),
      }
    }
    s
  }
}

#[derive(Debug, Clone, Default)]
pub struct SpirvCapability {
  pub name: StaticStr,
  pub enables: Vec<SpirvCapabilityEnable>,
}
impl SpirvCapability {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => s.name = value,
        _ => panic!("{key:?} = {value:?}"),
      }
    }
    s
  }
}
