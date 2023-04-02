/// The `graphql` attribute as a `syn::Path`.
fn path_to_match() -> syn::Path {
    syn::parse_str("loader").expect("`loader` is a valid path")
}

/// Extract an configuration parameter specified in the `graphql` attribute.
pub fn extract_attr(ast: &syn::DeriveInput, attr: &str) -> Result<String, syn::Error> {
    let attributes = &ast.attrs;
    let loader_path = path_to_match();
    let attribute = attributes
        .iter()
        .find(|attr| attr.path().is_ident(loader_path))
        .ok_or_else(|| syn::Error::new_spanned(ast, "The loader attribute is missing"))?;
    if let syn::Meta::List(items) = &attribute.parse_meta().expect("Attribute is well formatted") {
        for item in items.nested.iter() {
            if let syn::NestedMeta::Meta(syn::Meta::NameValue(name_value)) = item {
                let syn::MetaNameValue { path, lit, .. } = name_value;
                if let Some(ident) = path.get_ident() {
                    if ident == attr {
                        if let syn::Lit::Str(lit) = lit {
                            return Ok(lit.value());
                        }
                    }
                }
            }
        }
    }

    Err(syn::Error::new_spanned(
        ast,
        format!("Attribute `{}` not found", attr),
    ))
}
