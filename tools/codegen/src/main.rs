use anyhow::Result;
use ethabi::{Constructor, Param, ParamType};
use heck::ToSnakeCase;
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use std::env;
use std::ffi::OsString;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use substreams_ethereum::Abigen;
use syn::{File as SynFile, Ident, Index, Item, ItemMod};
use walkdir::WalkDir;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    // If specific ABI paths are provided, only process those
    // Otherwise, process all ABIs
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let abi_base = manifest_dir.join("../../abi").canonicalize()?;
    let out_base = manifest_dir.join("../../src").canonicalize()?;

    let entries: Vec<PathBuf> = if args.len() > 1 {
        // Process only specified files/dirs
        args[1..]
            .iter()
            .flat_map(|arg| {
                let path = PathBuf::from(arg);
                if path.is_dir() {
                    WalkDir::new(&path)
                        .into_iter()
                        .filter_map(|e| e.ok())
                        .filter(|e| {
                            e.path().is_file()
                                && e.path().extension().and_then(|ext| ext.to_str()) == Some("json")
                        })
                        .filter_map(|e| e.path().canonicalize().ok())
                        .collect::<Vec<_>>()
                } else if path.is_file() {
                    match path.canonicalize() {
                        Ok(path) => vec![path],
                        Err(err) => {
                            eprintln!("Warning: failed to resolve {}: {}, skipping", arg, err);
                            vec![]
                        }
                    }
                } else {
                    eprintln!("Warning: {} not found, skipping", arg);
                    vec![]
                }
            })
            .collect()
    } else {
        // Process all ABIs
        WalkDir::new(&abi_base)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path().is_file()
                    && e.path().extension().and_then(|ext| ext.to_str()) == Some("json")
            })
            .filter_map(|e| e.path().canonicalize().ok())
            .collect()
    };

    println!("Processing {} ABI files...", entries.len());

    for json_path in &entries {
        let contract_name = json_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown_contract")
            .to_lowercase();

        // Strip the abi base prefix to get relative path
        let relative_path = json_path
            .strip_prefix(&abi_base)
            .map(PathBuf::from)
            .unwrap_or_else(|_| json_path.clone());

        // Convert hyphens to underscores in directory components
        let sanitized: PathBuf = relative_path
            .iter()
            .map(|c| OsString::from(c.to_str().unwrap_or("").replace('-', "_")))
            .collect();
        let mut output_path = out_base.join(sanitized);
        output_path.set_file_name(format!("{}.rs", contract_name));

        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }

        println!("  {} -> {:?}", contract_name, output_path);
        Abigen::new(&contract_name, &json_path.to_string_lossy().to_string())?
            .generate()?
            .write_to_file(&output_path)?;
        append_constructor_module(json_path, &output_path)?;
    }

    println!("Done!");
    Ok(())
}

fn append_constructor_module(json_path: &PathBuf, output_path: &PathBuf) -> Result<()> {
    let contract = ethabi::Contract::load(File::open(json_path)?)?;
    let Some(constructor) = contract.constructor.as_ref() else {
        return Ok(());
    };

    let mut contents = fs::read_to_string(output_path)?;
    if !contents.ends_with('\n') {
        contents.push('\n');
    }
    contents.push('\n');
    contents.push_str(&render_constructor_module(constructor)?);
    fs::write(output_path, contents)?;

    Ok(())
}

fn render_constructor_module(constructor: &Constructor) -> Result<String> {
    let module = generate_constructor_module(constructor);
    let parsed: ItemMod = syn::parse2(module)?;
    let file = SynFile {
        shebang: None,
        attrs: vec![],
        items: vec![Item::Mod(parsed)],
    };

    Ok(prettyplease::unparse(&file))
}

fn generate_constructor_module(constructor: &Constructor) -> TokenStream {
    let input_names = param_names(&constructor.inputs);
    let input_kinds: Vec<_> = constructor
        .inputs
        .iter()
        .map(|param| rust_type(&param.kind))
        .collect();
    let input_struct_fields: Vec<_> = input_names
        .iter()
        .zip(input_kinds.iter())
        .map(|(param_name, kind)| quote! { pub #param_name: #kind })
        .collect();

    let input_ethabi_param_types: Vec<_> = constructor
        .inputs
        .iter()
        .map(|input| to_syntax_string(&input.kind))
        .collect();

    let input_struct_decoded_fields: Vec<_> = constructor
        .inputs
        .iter()
        .zip(input_names.iter())
        .map(|(param, name)| {
            let data_access = quote! { values.pop().expect(INTERNAL_ERR) };
            let decode_input = from_token(&param.kind, &data_access);
            quote! {
                #name: #decode_input
            }
        })
        .collect();

    let tokenize: Vec<_> = input_names
        .iter()
        .zip(constructor.inputs.iter())
        .map(|(param_name, param)| to_token(&quote! { self.#param_name }, &param.kind))
        .collect();

    let decode_impl = if constructor.inputs.is_empty() {
        quote! {}
    } else {
        quote! {
            let mut values = ethabi::decode(&[#(#input_ethabi_param_types),*], data.as_ref())
                .map_err(|e| format!("unable to decode constructor input: {:?}", e))?;
            values.reverse();
        }
    };

    let struct_header = if constructor.inputs.iter().any(|param| is_long_tuple(&param.kind)) {
        quote! {
            #[derive(Clone)]
        }
    } else {
        quote! {
            #[derive(Debug, Clone, PartialEq)]
        }
    };

    quote! {
        /// Contract's constructor arguments.
        #[allow(dead_code, unused_imports, unused_variables)]
        pub mod constructor {
            use super::INTERNAL_ERR;

            #struct_header
            pub struct Constructor {
                #(#input_struct_fields),*
            }

            impl Constructor {
                pub fn decode(data: &[u8]) -> Result<Self, String> {
                    #decode_impl

                    Ok(Self {
                        #(#input_struct_decoded_fields),*
                    })
                }

                pub fn encode(&self) -> Vec<u8> {
                    ethabi::encode(&[#(#tokenize),*])
                }
            }
        }
    }
}

fn to_syntax_string(param_type: &ParamType) -> TokenStream {
    match *param_type {
        ParamType::Address => quote! { ethabi::ParamType::Address },
        ParamType::Bytes => quote! { ethabi::ParamType::Bytes },
        ParamType::Int(size) => quote! { ethabi::ParamType::Int(#size) },
        ParamType::Uint(size) => quote! { ethabi::ParamType::Uint(#size) },
        ParamType::Bool => quote! { ethabi::ParamType::Bool },
        ParamType::String => quote! { ethabi::ParamType::String },
        ParamType::Array(ref kind) => {
            let inner = to_syntax_string(kind);
            quote! { ethabi::ParamType::Array(Box::new(#inner)) }
        }
        ParamType::FixedBytes(size) => quote! { ethabi::ParamType::FixedBytes(#size) },
        ParamType::FixedArray(ref kind, size) => {
            let inner = to_syntax_string(kind);
            quote! { ethabi::ParamType::FixedArray(Box::new(#inner), #size) }
        }
        ParamType::Tuple(ref types) => {
            let param_types = types.iter().map(to_syntax_string);
            quote! { ethabi::ParamType::Tuple(vec![#(#param_types),*]) }
        }
    }
}

fn rust_type(input: &ParamType) -> TokenStream {
    match *input {
        ParamType::Address => quote! { Vec<u8> },
        ParamType::Bytes => quote! { Vec<u8> },
        ParamType::FixedBytes(size) => quote! { [u8; #size] },
        ParamType::Int(_) => quote! { substreams::scalar::BigInt },
        ParamType::Uint(_) => quote! { substreams::scalar::BigInt },
        ParamType::Bool => quote! { bool },
        ParamType::String => quote! { String },
        ParamType::Array(ref kind) => {
            let inner = rust_type(kind);
            quote! { Vec<#inner> }
        }
        ParamType::FixedArray(ref kind, size) => {
            let inner = rust_type(kind);
            quote! { [#inner; #size] }
        }
        ParamType::Tuple(ref types) => {
            let tuple_elements = types.iter().map(rust_type);
            quote! { (#(#tuple_elements,)*) }
        }
    }
}

fn is_long_tuple(input: &ParamType) -> bool {
    match input {
        ParamType::Address
        | ParamType::Int(_)
        | ParamType::Uint(_)
        | ParamType::Bool
        | ParamType::FixedBytes(_)
        | ParamType::Bytes
        | ParamType::String => false,
        ParamType::Array(sub_type) => is_long_tuple(sub_type),
        ParamType::FixedArray(sub_type, _) => is_long_tuple(sub_type),
        ParamType::Tuple(types) => types.len() > 12 || types.iter().any(is_long_tuple),
    }
}

fn to_token(name: &TokenStream, kind: &ParamType) -> TokenStream {
    match *kind {
        ParamType::Address => {
            quote! { ethabi::Token::Address(ethabi::Address::from_slice(&#name)) }
        }
        ParamType::Bytes => quote! { ethabi::Token::Bytes(#name.clone()) },
        ParamType::FixedBytes(_) => quote! { ethabi::Token::FixedBytes(#name.as_ref().to_vec()) },
        ParamType::Int(_) => {
            quote! {
                {
                    let non_full_signed_bytes = #name.to_signed_bytes_be();
                    let full_signed_bytes_init =
                        if non_full_signed_bytes[0] & 0x80 == 0x80 { 0xff } else { 0x00 };
                    let mut full_signed_bytes = [full_signed_bytes_init as u8; 32];
                    non_full_signed_bytes
                        .into_iter()
                        .rev()
                        .enumerate()
                        .for_each(|(i, byte)| full_signed_bytes[31 - i] = byte);

                    ethabi::Token::Int(ethabi::Int::from_big_endian(full_signed_bytes.as_ref()))
                }
            }
        }
        ParamType::Uint(_) => {
            quote! {
                ethabi::Token::Uint(
                    ethabi::Uint::from_big_endian(
                        match #name.clone().to_bytes_be() {
                            (num_bigint::Sign::Plus, bytes) => bytes,
                            (num_bigint::Sign::NoSign, bytes) => bytes,
                            (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported")
                            }
                        }
                        .as_slice(),
                    ),
                )
            }
        }
        ParamType::Bool => quote! { ethabi::Token::Bool(#name.clone()) },
        ParamType::String => quote! { ethabi::Token::String(#name.clone()) },
        ParamType::Array(ref kind) => {
            let inner_name = quote! { inner };
            let inner_token = to_token(&inner_name, kind);
            quote! {
                {
                    let v = #name.iter().map(|#inner_name| #inner_token).collect();
                    ethabi::Token::Array(v)
                }
            }
        }
        ParamType::FixedArray(ref kind, _) => {
            let inner_name = quote! { inner };
            let inner_token = to_token(&inner_name, kind);
            quote! {
                {
                    let v = #name.iter().map(|#inner_name| #inner_token).collect();
                    ethabi::Token::FixedArray(v)
                }
            }
        }
        ParamType::Tuple(ref types) => {
            let inner_names = (0..types.len())
                .map(|index| {
                    let index = Index::from(index);
                    quote! { #name.#index }
                })
                .collect::<Vec<_>>();
            let inner_tokens = types
                .iter()
                .zip(inner_names.iter())
                .map(|(kind, inner_name)| to_token(&inner_name.to_token_stream(), kind))
                .collect::<Vec<_>>();

            quote! {
                ethabi::Token::Tuple(vec![#(#inner_tokens),*])
            }
        }
    }
}

fn from_token(kind: &ParamType, token: &TokenStream) -> TokenStream {
    match *kind {
        ParamType::Address => {
            quote! { #token.into_address().expect(INTERNAL_ERR).as_bytes().to_vec() }
        }
        ParamType::Bytes => quote! { #token.into_bytes().expect(INTERNAL_ERR) },
        ParamType::FixedBytes(size) => {
            let size: Index = size.into();
            quote! {
                {
                    let mut result = [0u8; #size];
                    let v = #token.into_fixed_bytes().expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                }
            }
        }
        ParamType::Int(_) => quote! {
            {
                let mut v = [0u8; 32];
                #token.into_int().expect(INTERNAL_ERR).to_big_endian(v.as_mut_slice());
                substreams::scalar::BigInt::from_signed_bytes_be(&v)
            }
        },
        ParamType::Uint(_) => quote! {
            {
                let mut v = [0u8; 32];
                #token.into_uint().expect(INTERNAL_ERR).to_big_endian(v.as_mut_slice());
                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
            }
        },
        ParamType::Bool => quote! { #token.into_bool().expect(INTERNAL_ERR) },
        ParamType::String => quote! { #token.into_string().expect(INTERNAL_ERR) },
        ParamType::Array(ref kind) => {
            let inner = quote! { inner };
            let inner_conversion = from_token(kind, &inner);
            quote! {
                #token
                    .into_array()
                    .expect(INTERNAL_ERR)
                    .into_iter()
                    .map(|#inner| #inner_conversion)
                    .collect()
            }
        }
        ParamType::FixedArray(ref kind, size) => {
            let inner = quote! { inner };
            let inner_conversion = from_token(kind, &inner);
            let to_array = vec![quote! { iter.next().expect(INTERNAL_ERR) }; size];
            quote! {
                {
                    let mut iter = #token
                        .into_fixed_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|#inner| #inner_conversion);
                    [#(#to_array),*]
                }
            }
        }
        ParamType::Tuple(ref types) => {
            let conversions = types.iter().enumerate().map(|(index, kind)| {
                let inner = quote! { tuple_elements[#index].clone() };
                from_token(kind, &inner)
            });
            quote! {
                {
                    let tuple_elements = #token.into_tuple().expect(INTERNAL_ERR);
                    (#(#conversions,)*)
                }
            }
        }
    }
}

fn param_names(inputs: &[Param]) -> Vec<Ident> {
    inputs
        .iter()
        .enumerate()
        .map(|(index, param)| {
            if param.name.is_empty() {
                Ident::new(&format!("param{}", index), Span::call_site())
            } else {
                Ident::new(&rust_variable(&param.name), Span::call_site())
            }
        })
        .collect()
}

fn rust_variable(name: &str) -> String {
    match name {
        "self" => "_self".to_string(),
        other => other.to_snake_case(),
    }
}
