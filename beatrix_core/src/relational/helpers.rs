use syn::Field;

pub(crate) fn format_name(name: &str) -> String {
    let mut new_name: String = "".into();

    for (index, char) in name.chars().enumerate() {
        if char.is_uppercase() {
            if index == 0 {
                new_name.push(char.to_ascii_lowercase());
            } else {
                new_name.push_str(&format!("_{}", char.to_ascii_lowercase()));
            }
        } else {
            new_name.push(char);
        }
    }

    new_name
}

pub(crate) fn field_name(field: Field) -> String {
    let field_name = field.ident.unwrap();
    let mut name = field_name.to_string();

    for attr in field.attrs.iter() {
        if attr.path.segments[0].ident == "name" {
            let value = attr.tokens.to_string();
            let value = value.split_once("=").unwrap();
            name = value.1.trim().replacen("\"", "", 2);
        }
    }

    name
}
