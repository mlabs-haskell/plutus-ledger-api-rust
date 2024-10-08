use std::str::FromStr;

use quote::ToTokens;
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote,
    punctuated::Punctuated,
    spanned::Spanned,
    token::Impl,
    Attribute, Block, Data, DataEnum, DeriveInput, Error, Expr, Fields, FieldsNamed, FieldsUnnamed,
    Ident, ImplGenerics, LitStr, Meta, Result, Stmt, TypeGenerics, WhereClause,
};

#[proc_macro_derive(IsPlutusData, attributes(plutus_data_derive_strategy))]
pub fn derive_is_plutus_data(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    get_is_plutus_data_instance(input)
        .unwrap()
        .into_token_stream()
        .into()
}

fn get_is_plutus_data_instance(input: DeriveInput) -> Result<Impl> {
    let type_name = &input.ident;

    let strategy = get_derive_strategy(&input)?;

    let (encoder, decoder) = match strategy {
        DeriveStrategy::Newtype => get_newtype_encoder_decoder(&input),
        DeriveStrategy::List => get_list_encoder_decoder(&input),
        DeriveStrategy::Constr => get_constr_encoder_decoder(&input),
    }?;

    let mut generics = input.generics;

    // TODO(chfanghr): Do we care about type role?
    generics.type_params_mut().for_each(|param| {
        param
            .bounds
            .push(parse_quote!(plutus_ledger_api::plutus_Data::IsPlutusData));
    });

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    Ok(parse_quote!(
        impl #impl_generics plutus_ledger_api::plutus_data::IsPlutusData for #type_name #type_generics #where_clause {
            fn to_plutus_data(&self) -> plutus_ledger_api::plutus_data::PlutusData {
                #encoder
            }

            fn from_plutus_data(plutus_data: &plutus_ledger_api::plutus_data::PlutusData) -> Result<Self, plutus_ledger_api::plutus_data::PlutusDataError>
                where Self: Sized; {
                #decoder
            }
        }
    ))
}

#[derive(Debug)]
enum DeriveStrategy {
    Newtype,
    List,
    Constr,
}

#[derive(Debug, thiserror::Error)]
enum DeriveStrategyError {
    #[error("Unknown strategy {0}. Should be one of Newtype, List and Constr.")]
    UnknownStrategy(String),
    #[error("Unable to parse strategy. Should be an Ident.")]
    UnexpectedToken,
    #[error("More than one strategies specified.")]
    MoreThanOneSpecified,
}

impl Default for DeriveStrategy {
    fn default() -> Self {
        Self::Constr
    }
}

impl FromStr for DeriveStrategy {
    type Err = DeriveStrategyError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Newtype" => Ok(Self::Newtype),
            "List" => Ok(Self::List),
            "Constr" => Ok(Self::Constr),
            _ => Err(DeriveStrategyError::UnknownStrategy(s.into())),
        }
    }
}
impl Parse for DeriveStrategy {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = input.call(Ident::parse)?;
        Self::from_str(&ident.to_string()).map_err(|unknown_strategy| {
            Error::new(
                ident.span(),
                format!("unknown strategy: {}", unknown_strategy),
            )
        })
    }
}

fn try_parse_derive_strategy(attr: &Attribute) -> Option<Result<DeriveStrategy>> {
    let value = match &attr.meta {
        Meta::NameValue(name_value) => name_value
            .path
            .is_ident("plutus_data_derive_strategy")
            .then_some(&name_value.value),
        _ => None,
    }?;

    Some(match &value {
        Expr::Path(path) => (|| -> Result<DeriveStrategy> {
            let ident = path.path.require_ident()?;
            DeriveStrategy::from_str(&ident.to_string())
                .map_err(|err| Error::new(ident.span(), err))
        })(),
        _ => Err(Error::new(
            value.span(),
            DeriveStrategyError::UnexpectedToken,
        )),
    })
}

fn get_derive_strategy(input: &DeriveInput) -> Result<DeriveStrategy> {
    let mut derive_strategy_results: Vec<_> = input
        .attrs
        .iter()
        .map(try_parse_derive_strategy)
        .flatten()
        .collect();

    match derive_strategy_results.len() {
        0 => Ok(DeriveStrategy::default()),
        1 => derive_strategy_results.remove(0),
        _ => Err(Error::new(
            input.span(),
            DeriveStrategyError::MoreThanOneSpecified,
        )),
    }
}

#[derive(Debug, thiserror::Error)]
enum NewtypeStrategyError {
    #[error("Only struct types are supported by newtype strategy")]
    UnexpectedDataVariant,
    #[error("Newtype derivation expects exactly one filed")]
    NotSingleField,
}

fn get_newtype_encoder_decoder(input: &DeriveInput) -> Result<(Block, Block)> {
    let s = match &input.data {
        Data::Struct(s) => Ok(s),
        _ => Err(Error::new(
            input.span(),
            NewtypeStrategyError::UnexpectedDataVariant,
        )),
    }?;

    if s.fields.len() != 1 {
        Err(Error::new(
            input.span(),
            NewtypeStrategyError::NotSingleField,
        ))?
    }

    let field = s.fields.iter().next().unwrap();

    let encoder = parse_quote!({
        self.#field.to_plutus_data()
    });

    let decoder = match &field.ident {
        Some(field_name) => {
            parse_quote!({
                Ok(Self {
                    #field_name: plutus_ledger_api::plutus_data::IsPlutusData::from_plutus_data(plutus_data)?
                })
            })
        }
        None => {
            parse_quote!({
                Ok(Self(
                    plutus_ledger_api::plutus_data::IsPlutusData::from_plutus_data(plutus_data)?,
                ))
            })
        }
    };

    Ok((encoder, decoder))
}

#[derive(Debug, thiserror::Error)]
enum ListStrategyError {
    #[error("Only struct types are supported by list strategy")]
    UnexpectedDataVariant,
}

fn get_list_encoder_decoder(input: &DeriveInput) -> Result<(Block, Block)> {
    match &input.data {
        Data::Struct(s) => {
            let fields = &s.fields;

            let encode_field_stmts = fields
                .iter()
                .map(|f| -> Stmt { parse_quote!(self.#f.to_plutus_data()) });

            let encoder: Block = parse_quote!({
                plutus_ledger_api::plutus_data::PlutusData::List(vec![ #(#encode_field_stmts),*])
            });

            let decoder: Block = match &fields {
                Fields::Named(named_fields) => get_list_decoder_for_named_struct(named_fields),
                Fields::Unnamed(unnamed_fields) => {
                    get_list_decoder_for_unnamed_struct(unnamed_fields)
                }
                Fields::Unit => parse_quote!({ Self }),
            };

            Ok((encoder, decoder))
        }
        _ => Err(Error::new(
            input.span(),
            ListStrategyError::UnexpectedDataVariant,
        )),
    }
}

fn get_list_decoder_for_named_struct(fields: &FieldsNamed) -> Block {
    let len = fields.named.len();

    let decode_field_stmts = fields.named.iter().enumerate().map(|(i, f)| -> Stmt {
        parse_quote!(
            #f: plutus_ledger_api::plutus_data::IsPlutusData::from_plutus_data(fields[#i])?
        )
    });

    parse_quote!({
        let fields = plutus_ledger_api::plutus_data::parse_list(plutus_data)?; // TODO(chfanghr): implement parse_list
        let fields = plutus_ledger_api::plutus_data::parse_constr_with_tag::parse_fixed_len_plutus_data_list::<#len>(fields.as_slice())?;

        Ok(Self {
            #(#decode_field_stmts),*
        })
    })
}

fn get_list_decoder_for_unnamed_struct(fields: &FieldsUnnamed) -> Block {
    let len = fields.unnamed.len();
    let indexes = 0..len;

    parse_quote!({
        let fields = plutus_ledger_api::plutus_data::parse_list(plutus_data)?;
        let fields = plutus_ledger_api::plutus_data::parse_constr_with_tag::parse_fixed_len_plutus_data_list::<#len>(fields.as_slice())?;

        Ok(Self (
            #(plutus_ledger_api::plutus_data::IsPlutusData::from_plutus_data(fields[#indexes])?),*
        ))
    })
}

#[derive(Debug, thiserror::Error)]
enum ConstrStrategyError {
    #[error("Union types are supported by constr strategy")]
    UnexpectedDataVariant,
}

fn get_constr_encoder_decoder(input: &DeriveInput) -> Result<(Block, Block)> {
    todo!()
}
