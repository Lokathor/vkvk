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

// TODO: all of the `attrs: _` must be removed.

#[derive(Debug, Clone, Default)]
pub struct Registry {
  // TODO: platforms?
  // TODO: tags
  // TODO: types
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
        StartTag { name: "enums", attrs } => {
          let mut enumeration = Enumeration::from_attrs(attrs);
          'enums: loop {
            match iter.next().unwrap() {
              EndTag { name: "enums" } => {
                break 'enums;
              }
              EmptyTag { name: "enum", attrs } => {
                enumeration.entries.push(EnumerationEntry::from_attrs(attrs))
              }
              _ => (),
            }
          }
          registry.enums.push(enumeration);
        }
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
                                command_param.ty_variant = TypeVariant::ConstArrayPtr(2)
                              }
                              ("[4]", TypeVariant::ConstPtr) => {
                                command_param.ty_variant = TypeVariant::ConstArrayPtr(4)
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
pub struct EnumerationEntry {
  pub name: StaticStr,
  pub value: StaticStr,
  pub comment: StaticStr,
  pub ty: StaticStr,
  pub alias: StaticStr,
  pub bitpos: StaticStr,
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
  ConstArrayPtr(usize),
  /// `*mut *const T`
  MutPtrConstPtr,
  /// `*const *const T`
  ConstPtrConstPtr,
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
      TypeVariant::ConstArrayPtr(n) => write!(f, "{name}: *const [{ty}; {n}]"),
      TypeVariant::MutPtrConstPtr => write!(f, "{name}: *mut *const {ty}"),
      TypeVariant::ConstPtrConstPtr => write!(f, "{name}: *const *const {ty}"),
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
  pub successcodes: StaticStr,
  pub errorcodes: StaticStr,
  pub queues: StaticStr,
  pub alias: StaticStr,
  pub renderpass: StaticStr,
  pub cmdbufferlevel: StaticStr,
  pub tasks: StaticStr,
  pub videocoding: StaticStr,
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
  pub api: StaticStr,
  pub number: StaticStr,
  pub comment: StaticStr,
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
pub struct SpirvExtension {
  pub name: StaticStr,
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
pub struct SpirvCapability {
  pub name: StaticStr,
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
