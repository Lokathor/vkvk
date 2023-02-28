#![allow(clippy::should_implement_trait)]

use magnesium::{XmlElement::*, *};

pub type StaticStr = &'static str;

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

fn fix_ty(ty: StaticStr) -> StaticStr {
  match ty {
    "char" => "u8",
    "uint8_t" => "u8",
    "int32_t" => "i32",
    "uint32_t" => "u32",
    "uint64_t" => "u64",
    "size_t" => "usize",
    "float" => "c_float",
    "void" => "c_void", // command return void is fixed to () elsewhere.
    other => other,
  }
}

/// Registry of all useful data out of a `vk.xml` file.
///
/// ```rust,no_run
/// use magnesium::*;
/// use vkvk_generator::vk_dot_xml_parser::*;
/// let XML: &'static str = unimplemented!();
/// let mut iter = ElementIterator::new(XML)
///   .filter_map(skip_comments)
///   .map(trim_text)
///   .filter_map(skip_empty_text_elements);
/// assert_eq!(iter.next().unwrap().unwrap_start_tag().0, "registry");
/// let registry = Registry::from_iter(&mut iter);
/// ```
#[derive(Debug, Clone, Default)]
pub struct Registry {
  pub platforms: Vec<Platform>,
  pub vendor_tags: Vec<VendorTag>,
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
  #[allow(clippy::field_reassign_with_default)]
  pub fn from_iter(iter: &mut impl Iterator<Item = XmlElement<'static>>) -> Self {
    let mut registry = Self::default();
    loop {
      match iter.next().unwrap() {
        EndTag { name: "registry" } => {
          // POST PROCESSING
          let mut extra_types = Vec::new();
          // If we see FooFlags or FooFlagBits without the matching other side of the
          // pairing then we add a dummy entry for the other side so that registry lookup
          // (ideally) won't ever fail.
          for ty in registry.types.iter() {
            if let Some(n) = ty.name.strip_suffix("Flags") {
              let s = format!("{n}FlagBits");
              if !registry.types.iter().any(|ty| ty.name == s) {
                let mut te = TypeEntry::default();
                te.category = Some("bitmask");
                te.name = Box::leak(s.into_boxed_str());
                extra_types.push(te);
              }
            } else if let Some(n) = ty.name.strip_suffix("FlagBits") {
              let s = format!("{n}Flags");
              if !registry.types.iter().any(|ty| ty.name == s) {
                let mut te = TypeEntry::default();
                te.category = Some("bitmask");
                te.name = Box::leak(s.into_boxed_str());
                extra_types.push(te);
              }
            }
          }
          registry.types.extend(extra_types.into_iter());
          // We're going to change sone specific fields into our custom newtype.
          for (s, f) in [
            ("VkPhysicalDeviceProperties", "apiVersion"),
            ("VkLayerProperties", "specVersion"),
            ("VkApplicationInfo", "apiVersion"),
          ] {
            if let Some(te) = registry.types.iter_mut().find(|t| t.name == s) {
              if let Some(m) = te.members.iter_mut().find(|m| m.name == f) {
                m.ty = "VkVersion";
              }
            }
          }
          // add VkVersion as a fake base type
          let mut te = TypeEntry::default();
          te.name = "VkVersion";
          te.category = Some("basetype");
          registry.types.push(te);
          return registry;
        }
        StartTag { name: "comment", attrs: "" } => loop {
          if let EndTag { name: "comment" } = iter.next().unwrap() {
            break;
          }
        },
        StartTag { name: "platforms", attrs } => {
          do_platforms(attrs, &mut registry.platforms, iter)
        }
        StartTag { name: "tags", attrs } => {
          do_tags(attrs, &mut registry.vendor_tags, iter)
        }
        StartTag { name: "types", attrs } => do_types(attrs, &mut registry.types, iter),
        StartTag { name: "enums", attrs } => do_enums(attrs, &mut registry.enums, iter),
        StartTag { name: "commands", attrs } => {
          do_commands(attrs, &mut registry.commands, iter)
        }
        StartTag { name: "feature", attrs } => {
          do_feature(attrs, &mut registry.features, iter)
        }
        StartTag { name: "extensions", attrs } => {
          do_extensions(attrs, &mut registry.extensions, iter)
        }
        StartTag { name: "formats", attrs } => {
          do_formats(attrs, &mut registry.formats, iter)
        }
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
  attrs: StaticStr, org_tags: &mut Vec<VendorTag>,
  iter: &mut impl Iterator<Item = XmlElement<'static>>,
) {
  assert_attrs_comment_only!(attrs);
  loop {
    match iter.next().unwrap() {
      EndTag { name: "tags" } => return,
      EmptyTag { name: "tag", attrs } => {
        org_tags.push(VendorTag::from_attrs(attrs));
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
              ty_entry.comment = Some(iter.next().unwrap().unwrap_text());
              assert_eq!(iter.next().unwrap().unwrap_end_tag(), "comment");
            }
            StartTag { name: "name", attrs: "" } => {
              assert!(ty_entry.name.is_empty());
              ty_entry.name = iter.next().unwrap().unwrap_text();
              assert_eq!(iter.next().unwrap().unwrap_end_tag(), "name");
              ty_entry.texts.push(ty_entry.name);
            }
            StartTag { name: "type", attrs: "" } => {
              let src = iter.next().unwrap().unwrap_text();
              ty_entry.ty_src = Some(src);
              assert_eq!(iter.next().unwrap().unwrap_end_tag(), "type");
              ty_entry.texts.push(src);
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
              member.ty = fix_ty(iter.next().unwrap().unwrap_text());
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
                    TypeVariant::ConstPtr => {
                      member.ty_variant = TypeVariant::MutPtrConstPtr
                    }
                    other => panic!("{other:?}"),
                  }
                  match iter.next().unwrap() {
                    StartTag { name: "name", attrs: "" } => (),
                    other => panic!("{other:?}"),
                  }
                }
                Text("* const*") | Text("* const *") => {
                  match member.ty_variant {
                    TypeVariant::ConstPtr => {
                      member.ty_variant = TypeVariant::ConstPtrConstPtr
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
                        member.ty_variant = TypeVariant::ArrayNamed(arr_len);
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
                    TypeVariant::Normal => {
                      member.ty_variant = TypeVariant::ArrayOfArrayLit(3, 4)
                    }
                    other => panic!("{other:?}"),
                  },
                  Text(":8") => match member.ty_variant {
                    TypeVariant::Normal => {
                      member.ty_variant = TypeVariant::BitfieldsLit(8)
                    }
                    other => panic!("{other:?}"),
                  },
                  Text(":24") => match member.ty_variant {
                    TypeVariant::Normal => {
                      member.ty_variant = TypeVariant::BitfieldsLit(24)
                    }
                    other => panic!("{other:?}"),
                  },
                  StartTag { name: "comment", attrs: "" } => {
                    assert!(member.comment.is_none());
                    member.comment = Some(iter.next().unwrap().unwrap_text());
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
        let mut te = TypeEntry::from_attrs(attrs);
        if te.name.ends_with("FlagBits") {
          te.category = Some("bitmask");
        }
        types.push(te);
      }
      StartTag { name: "comment", attrs: "" } => {
        let _ = iter.next().unwrap().unwrap_text();
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
              commands.push(command);
              break 'command;
            }
            StartTag { name: "proto", attrs: "" } => {
              assert_eq!(iter.next().unwrap().unwrap_start_tag(), ("type", ""));
              command.return_ty = match iter.next().unwrap().unwrap_text() {
                "void" => "()",
                other => fix_ty(other),
              };
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
              command_param.ty = fix_ty(iter.next().unwrap().unwrap_text());
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
                    TypeVariant::Normal => {
                      command_param.ty_variant = TypeVariant::MutPtrMutPtr
                    }
                    TypeVariant::ConstPtr => {
                      command_param.ty_variant = TypeVariant::MutPtrConstPtr
                    }
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
  attrs: StaticStr, formats: &mut Vec<Format>,
  iter: &mut impl Iterator<Item = XmlElement<'static>>,
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
pub struct VendorTag {
  pub name: StaticStr,
  pub author: StaticStr,
  pub contact: StaticStr,
}
impl VendorTag {
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
  pub comment: Option<StaticStr>,
  pub optional: Option<StaticStr>,
  pub no_auto_validity: Option<StaticStr>,
  pub limit_type: Option<StaticStr>,
  pub values: Option<StaticStr>,
  pub len: Option<StaticStr>,
  pub alt_len: Option<StaticStr>,
  pub object_type: Option<StaticStr>,
  pub extern_sync: Option<StaticStr>,
  pub selection: Option<StaticStr>,
  pub selector: Option<StaticStr>,
  pub deprecated: Option<StaticStr>,
  pub api: Option<StaticStr>,
}
impl Member {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "optional" => s.optional = Some(value),
        "noautovalidity" => s.no_auto_validity = Some(value),
        "limittype" => s.limit_type = Some(value),
        "values" => s.values = Some(value),
        "len" => s.len = Some(value),
        "altlen" => s.alt_len = Some(value),
        "objecttype" => s.object_type = Some(value),
        "externsync" => s.extern_sync = Some(value),
        "selection" => s.selection = Some(value),
        "selector" => s.selector = Some(value),
        "deprecated" => s.deprecated = Some(value),
        "api" => s.api = Some(value),
        _ => panic!("{key:?} = {value:?}"),
      }
    }
    s
  }
}

#[derive(Debug, Clone, Default)]
pub struct TypeEntry {
  pub name: StaticStr,
  pub api: Option<StaticStr>,
  pub category: Option<StaticStr>,
  pub texts: Vec<StaticStr>,
  pub comment: Option<StaticStr>,
  pub requires: Option<StaticStr>,
  pub ty_src: Option<StaticStr>,
  pub is_alias_for: Option<StaticStr>,
  pub bit_values: Option<StaticStr>,
  pub obj_type_enum: Option<StaticStr>,
  pub parent: Option<StaticStr>,
  pub returned_only: Option<StaticStr>,
  pub members: Vec<Member>,
  pub struct_extends: Option<StaticStr>,
  pub allow_duplicate: Option<StaticStr>,
  pub deprecated: Option<StaticStr>,
}
impl TypeEntry {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => s.name = value,
        "category" => s.category = Some(value),
        "requires" => s.requires = Some(value),
        "alias" => s.is_alias_for = Some(value),
        "bitvalues" => s.bit_values = Some(value),
        "objtypeenum" => s.obj_type_enum = Some(value),
        "parent" => s.parent = Some(value),
        "returnedonly" => s.returned_only = Some(value),
        "structextends" => s.struct_extends = Some(value),
        "comment" => s.comment = Some(value),
        "allowduplicate" => s.allow_duplicate = Some(value),
        "deprecated" => s.deprecated = Some(value),
        "api" => s.api = Some(value),
        _ => panic!("{key:?} = {value:?}"),
      }
    }
    s
  }
}

#[derive(Debug, Clone, Default)]
pub struct EnumerationEntry {
  pub name: StaticStr,
  pub value: Option<StaticStr>,
  pub comment: Option<StaticStr>,
  pub ty: Option<StaticStr>,
  pub alias: Option<StaticStr>,
  pub bitpos: Option<StaticStr>,
  pub api: Option<StaticStr>,
  pub deprecated: Option<StaticStr>,
}
impl EnumerationEntry {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => s.name = value,
        "value" => s.value = Some(value),
        "comment" => s.comment = Some(value),
        "type" => s.ty = Some(value),
        "alias" => s.alias = Some(value),
        "bitpos" => s.bitpos = Some(value),
        "api" => s.api = Some(value),
        "deprecated" => s.deprecated = Some(value),
        _ => panic!("{key:?} = {value:?}"),
      }
    }
    s
  }
}

#[derive(Debug, Clone, Default)]
pub struct Enumeration {
  pub name: StaticStr,
  pub ty: Option<StaticStr>,
  pub comment: Option<StaticStr>,
  pub bitwidth: Option<StaticStr>,
  pub entries: Vec<EnumerationEntry>,
}
impl Enumeration {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => s.name = value,
        "type" => s.ty = Some(value),
        "comment" => s.comment = Some(value),
        "bitwidth" => s.bitwidth = Some(value),
        _ => panic!("{key:?} = {value:?}"),
      }
    }
    s
  }
}

#[derive(Debug, Clone, Copy, Default)]
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
  /// `*mut *const T`
  MutPtrConstPtr,
  /// `*const *const T`
  ConstPtrConstPtr,
  /// `[T; N]`
  ArrayLit(usize),
  /// `[T; NAMED_CONSTANT]`
  ArrayNamed(StaticStr),
  /// `[[T; A]; B]`
  ArrayOfArrayLit(usize, usize),
  /// `:N`
  BitfieldsLit(usize),
}
impl TypeVariant {
  pub fn is_ptr(&self) -> bool {
    matches!(
      *self,
      TypeVariant::ConstPtr
        | TypeVariant::MutPtr
        | TypeVariant::MutPtrMutPtr
        | TypeVariant::ConstArrayPtrLit(_)
        | TypeVariant::MutPtrConstPtr
        | TypeVariant::ConstPtrConstPtr
    )
  }
}

#[derive(Clone, Default)]
pub struct CommandParam {
  pub name: StaticStr,
  pub ty_variant: TypeVariant,
  pub ty: StaticStr,
  //
  pub optional: Option<StaticStr>,
  pub extern_sync: Option<StaticStr>,
  pub len: Option<StaticStr>,
  pub alt_len: Option<StaticStr>,
  pub no_auto_validity: Option<StaticStr>,
  pub stride: Option<StaticStr>,
  pub object_type: Option<StaticStr>,
  pub valid_structs: Option<StaticStr>,
  pub api: Option<StaticStr>,
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
      TypeVariant::MutPtrConstPtr => write!(f, "{name}: *mut *const {ty}"),
      TypeVariant::ConstPtrConstPtr => write!(f, "{name}: *const *const {ty}"),
      TypeVariant::ArrayLit(n) => write!(f, "{name}: [{ty}; {n}]"),
      TypeVariant::ArrayNamed(n) => write!(f, "{name}: [{ty}; {n}]"),
      TypeVariant::ArrayOfArrayLit(a, b) => write!(f, "{name}: [[{ty}; {a}]; {b}]"),
      TypeVariant::BitfieldsLit(n) => write!(f, "{name}: {ty}{{:{n}}}"),
    }?;
    match self.optional {
      Some("true") => write!(f, " /* optional */")?,
      Some(other) => write!(f, " /* optional={other:?} */")?,
      _ => (),
    }
    match self.extern_sync {
      Some("true") => write!(f, " /* extern_sync */")?,
      Some(other) => write!(f, " /* extern_sync={other:?} */")?,
      _ => (),
    }
    if let Some(len) = self.len {
      write!(f, " /* len={len} */")?;
    }
    if let Some(alt_len) = self.alt_len {
      write!(f, " /* alt_len={alt_len} */")?;
    }
    match self.no_auto_validity {
      Some("true") => write!(f, " /* no_auto_validity */")?,
      Some(other) => write!(f, " /* no_auto_validity={other:?} */")?,
      _ => (),
    }
    if let Some(stride) = self.stride {
      write!(f, " /* stride={stride} */")?;
    }
    if let Some(object_type) = self.object_type {
      write!(f, " /* object_type={object_type} */")?;
    }
    if let Some(valid_structs) = self.valid_structs {
      write!(f, " /* valid_structs={valid_structs} */")?;
    }
    Ok(())
  }
}
impl CommandParam {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "len" => s.len = Some(value),
        "altlen" => s.alt_len = Some(value),
        "stride" => s.stride = Some(value),
        "optional" => s.optional = Some(value),
        "externsync" => s.extern_sync = Some(value),
        "objecttype" => s.object_type = Some(value),
        "noautovalidity" => s.no_auto_validity = Some(value),
        "validstructs" => s.valid_structs = Some(value),
        "api" => s.api = Some(value),
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
  pub comment: Option<StaticStr>,
  pub success_codes: Option<StaticStr>,
  pub error_codes: Option<StaticStr>,
  pub queues: Option<StaticStr>,
  pub alias: Option<StaticStr>,
  pub render_pass: Option<StaticStr>,
  pub cmd_buffer_level: Option<StaticStr>,
  pub tasks: Option<StaticStr>,
  pub video_coding: Option<StaticStr>,
  pub implicitexternsyncparams: Vec<StaticStr>,
  pub api: Option<StaticStr>,
}
impl Command {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "successcodes" => s.success_codes = Some(value),
        "errorcodes" => s.error_codes = Some(value),
        "queues" => s.queues = Some(value),
        "alias" => s.alias = Some(value),
        "renderpass" => s.render_pass = Some(value),
        "cmdbufferlevel" => s.cmd_buffer_level = Some(value),
        "tasks" => s.tasks = Some(value),
        "comment" => s.comment = Some(value),
        "videocoding" => s.video_coding = Some(value),
        "name" => s.name = value,
        "api" => s.api = Some(value),
        _ => panic!("{key:?} = {value:?} ({attrs})"),
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
  pub value: Option<StaticStr>,
  pub offset: Option<StaticStr>,
  pub extends: Option<StaticStr>,
  pub dir: Option<StaticStr>,
  pub extnumber: Option<StaticStr>,
  pub bitpos: Option<StaticStr>,
  pub comment: Option<StaticStr>,
  pub alias: Option<StaticStr>,
  pub deprecated: Option<StaticStr>,
  pub api: Option<StaticStr>,
  pub protect: Option<StaticStr>,
}
impl RequiredEnum {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => s.name = value,
        "dir" => s.dir = Some(value),
        "value" => s.value = Some(value),
        "alias" => s.alias = Some(value),
        "bitpos" => s.bitpos = Some(value),
        "offset" => s.offset = Some(value),
        "extends" => s.extends = Some(value),
        "comment" => s.comment = Some(value),
        "extnumber" => s.extnumber = Some(value),
        "deprecated" => s.deprecated = Some(value),
        "api" => s.api = Some(value),
        "protect" => s.protect = Some(value),
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
  pub depends: Option<StaticStr>,
  pub api: Option<StaticStr>,
  pub comment: Option<StaticStr>,
}
impl RequireListEntry {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "api" => s.api = Some(value),
        "depends" => s.depends = Some(value),
        "comment" => s.comment = Some(value),
        _ => panic!("{key:?} = {value:?}"),
      }
    }
    s
  }
}

#[derive(Debug, Clone, Default)]
pub struct Extension {
  pub name: StaticStr,
  pub requires: Option<StaticStr>,
  pub comment: Option<StaticStr>,
  pub number: Option<StaticStr>,
  pub ty: Option<StaticStr>,
  pub author: Option<StaticStr>,
  pub contact: Option<StaticStr>,
  pub supported: Option<StaticStr>,
  pub platform: Option<StaticStr>,
  pub special_use: Option<StaticStr>,
  pub deprecated_by: Option<StaticStr>,
  pub promoted_to: Option<StaticStr>,
  pub obsoleted_by: Option<StaticStr>,
  pub provisional: Option<StaticStr>,
  pub sort_order: Option<StaticStr>,
  pub depends: Option<StaticStr>,
  pub require_lists: Vec<RequireListEntry>,
}
impl Extension {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => s.name = value,
        "type" => s.ty = Some(value),
        "number" => s.number = Some(value),
        "author" => s.author = Some(value),
        "contact" => s.contact = Some(value),
        "comment" => s.comment = Some(value),
        "requires" => s.requires = Some(value),
        "platform" => s.platform = Some(value),
        "sortorder" => s.sort_order = Some(value),
        "supported" => s.supported = Some(value),
        "specialuse" => s.special_use = Some(value),
        "promotedto" => s.promoted_to = Some(value),
        "provisional" => s.provisional = Some(value),
        "obsoletedby" => s.obsoleted_by = Some(value),
        "deprecatedby" => s.deprecated_by = Some(value),
        "depends" => s.depends = Some(value),
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
