use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, AngleBracketedGenericArguments, Attribute, Data, DeriveInput, Expr, ExprLit,
    Fields, GenericArgument, Ident, Lit, Meta, MetaNameValue, Path, PathArguments, Type, TypePath,
};

#[proc_macro_derive(Entity, attributes(collection_name))]
pub fn insertable(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let mut collection_name: Option<String> = None;
    for attr in ast.attrs.clone().into_iter() {
        if let Attribute {
            meta:
                Meta::NameValue(MetaNameValue {
                    value:
                        Expr::Lit(ExprLit {
                            lit: Lit::Str(lit), ..
                        }),
                    ..
                }),
            ..
        } = attr
        {
            collection_name = Some(lit.value())
        }
    }

    let ident: Ident;
    let fields: Fields;

    if let DeriveInput {
        data: Data::Struct(ds),
        ..
    } = ast
    {
        ident = ast.ident;
        fields = ds.fields;
    } else {
        panic!("Entity can only be derived for structs");
    }

    // This code tests that the struct we are dering for
    // has an id field with a type of Option<u32>
    let mut has_id: bool = false;
    for field in fields.into_iter() {
        if field.ident.unwrap().to_string() == "id" {
            if let Type::Path(TypePath {
                path: Path { segments, .. },
                ..
            }) = field.ty
            {
                let last_segment = segments.last().unwrap();
                if last_segment.ident.to_string() == "Option" {
                    if let PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                        args,
                        ..
                    }) = last_segment.clone().arguments
                    {
                        if let GenericArgument::Type(Type::Path(TypePath {
                            path: Path { segments, .. },
                            ..
                        })) = args.first().unwrap()
                        {
                            let last_segment = segments.last().unwrap().ident.to_string();
                            has_id = last_segment == "ObjectId" || last_segment == "u32";
                        }
                    }
                }
            }
        }
    }

    if !has_id {
        panic!("All entities must have an id field with type Option<u32>");
    }

    if collection_name.is_none() {
        collection_name = Some(ident.to_string());
    }

    let collection_name = collection_name.unwrap();

    let insert = quote! {
        async fn insert(&self, db: &mongodb::Database) -> std::result::Result<mongodb::results::InsertOneResult, mongodb::error::Error> {
            db.collection::<#ident>(#collection_name).insert_one(self, None).await
        }
    };

    let delete = quote! {
        async fn delete(self, db: &mongodb::Database) -> std::result::Result<mongodb::results::DeleteResult, mongodb::error::Error> {
            db.collection::<#ident>(#collection_name).delete_one(bson::to_document(&self).unwrap(), None).await
        }
    };

    quote! {
        #[async_trait::async_trait]
        impl Entity for #ident {
            type Output = Self;

            #insert
            #delete
        }
    }
    .into()
}
