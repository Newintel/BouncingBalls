use proc_macro::TokenStream;
use syn::{
    __private::quote::quote, parse_macro_input, Data, DeriveInput, Field, Ident, Lit, Meta,
    NestedMeta, Type,
};

#[proc_macro_derive(GetSet, attributes(gs, gs_ignore))]
pub fn generate_getters_setters(model: TokenStream) -> TokenStream {
    let input = parse_macro_input!(model as DeriveInput);
    let ident = &input.ident;

    let mut attrs = get_attrs(&input.data);
    let getters = attrs.iter_mut().map(|(id, ty, gs)| {
        if let Some(getter) = &gs.getter {
            let getter = Ident::new(&getter, id.span());
            quote! {
                pub fn #getter(&self) -> #ty {
                    self.#id
                }
            }
        } else {
            quote!()
        }
    });
    let mut attrs = get_attrs(&input.data);
    let setters = attrs.iter_mut().map(|(id, ty, gs)| {
        if let Some(setter) = &gs.setter {
            let setter = Ident::new(&setter, id.span());
            quote! {
                pub fn #setter(&mut self, #id : #ty) -> &mut Self {
                    self.#id = #id;
                    self
                }
            }
        } else {
            quote!()
        }
    });

    quote!(
        impl #ident {
            #(#getters)*

            #(#setters)*
        }
    )
    .into()
}

fn get_attrs(input: &Data) -> Vec<(&Ident, &Type, GSAttributes)> {
    let mut list: Vec<(&Ident, &Type, GSAttributes)> = vec![];

    if let Data::Struct(input) = input {
        let fields = &input.fields;
        for field in fields {
            if let Some(ident) = &field.ident {
                list.push((ident, &field.ty, parse_gs_attributes(field)));
            }
        }
    }

    list
}

#[derive(Default)]
struct GSAttributes {
    getter: Option<String>,
    setter: Option<String>,
}

impl GSAttributes {
    fn new(id: String) -> Self {
        Self {
            setter: Some(format!("set_{}", &id)),
            getter: Some(id),
        }
    }
}

fn parse_gs_attributes(field: &Field) -> GSAttributes {
    let id = &field.ident.as_ref().unwrap().to_string();
    let mut gs_attr = GSAttributes::new(id.to_string());

    if field.attrs.is_empty() == false {
        for attr in &field.attrs {
            let ident = attr
                .path
                .get_ident()
                .and_then(|id| Some(id.to_string()))
                .unwrap_or_default();

            assert!(ident == "gs" || ident == "gs_ignore",);

            match ident.as_str() {
                "gs_ignore" => {
                    if let Ok(meta) = attr.parse_meta() {
                        if let Meta::List(meta) = meta {
                            let error = "gs_ignore attribute must contain either get, set or both (ex : gs_ignore(get))";
                            for element in meta.nested.pairs() {
                                if let NestedMeta::Meta(m) = element.value() {
                                    if let Some(id) = m.path().get_ident() {
                                        match id.to_string().as_str() {
                                            "get" => gs_attr.getter = None,
                                            "set" => gs_attr.setter = None,
                                            _ => panic!("{error}"),
                                        }
                                    }
                                } else {
                                    panic!("{error}")
                                }
                            }
                        } else {
                            gs_attr = GSAttributes::default()
                        }
                    }
                }
                "gs" => {
                    let error =
                        "gs attribute must be gs([get = \"getter_name\"], [set = \"setter_name\"])";
                    if let Ok(meta) = attr.parse_meta() {
                        if let Meta::List(meta) = meta {
                            for element in meta.nested.pairs() {
                                if let NestedMeta::Meta(meta) = &element.value() {
                                    if let Meta::NameValue(attr) = meta {
                                        let id = attr.path.get_ident().unwrap();
                                        if let Lit::Str(val) = &attr.lit {
                                            match id.to_string().as_str() {
                                                "get" => gs_attr.getter.replace(val.value()),
                                                "set" => gs_attr.setter.replace(val.value()),
                                                _ => panic!("{error}"),
                                            };
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        panic!("{error}")
                    }
                }
                _ => panic!("Attribute must be gs or gs_ignore"),
            }
        }
    }

    gs_attr
}
