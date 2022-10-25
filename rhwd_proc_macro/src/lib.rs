use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();
    let new_str = match ident_str.as_str() {
        "Canlyniad" => "Result",
        "Dim" => "None",
        "Geiriadur" => "HashMap",
        "Gwall" => "Error",
        "Hunan" => "Self",
        "Iawn" => "Ok",
        "Llinyn" => "String",
        "Opsiwn" => "Option",
        "Rhagosodedig" => "Default",
        "Rhai" => "Some",
        "Wps" => "Err",
        "allanol" => "extern",
        "anghydamseredig" => "async",
        "anniogel" => "unsafe",
        "aros" => "await",
        "brintln" => "println",
        "cachu" | "ffyc" => "panic",
        "caniatáu" => "allow",
        "cod_anghyraeddadwy" => "unreachable_code",
        "crât" => "crate",
        "cyfateb" => "match",
        "cyfeiriad" => "ref",
        "cyhoeddus" => "pub",
        "cyson" => "const",
        "dadlapio" => "unwrap",
        "defnydd" => "use",
        "deinamig" | "dein" => "dyn", // Does dein work?
        "disgwyl" => "expect",
        "dolen" => "loop",
        "dychwelyd" => "return",
        "ereill" => "else",
        "fel" => "as",
        "fel_cyfeiriad" => "as_ref",
        "ffug" => "false",
        "ffwythiant" => "fn",
        "gadael" => "let",
        "gweithredu" => "impl",
        "gwir" => "true",
        "hunan" => "self",
        "i_mewn" => "into",
        "lle" => "where",
        "mewn" => "in",
        "mewnosod" => "insert",
        "modiwl" => "mod",
        "newydd" => "new",
        "nodwedd" => "trait",
        "os" => "if",
        "prif" => "main",
        "rhag" => "from",
        "rhagosodedig" => "default",
        "rhiant" => "super",
        "rhifol" => "enum",
        "statig" => "static",
        "strwythur" => "struct",
        "symud" => "move",
        "torri" => "break",
        "tra" => "while",
        "treigladwy" => "mut",
        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn rhwd(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
