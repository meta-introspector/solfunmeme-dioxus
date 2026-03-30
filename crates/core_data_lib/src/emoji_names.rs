use std::collections::HashMap;

pub fn get_emoji_names() -> HashMap<String, (String, String)> {
    let mut map = HashMap::new();
    let emoji_type_map: &[(&str, &str, &str)] = &[
        // Rust Core
        ("fn", "🦀⚙️", "Rust Core"),
        ("struct", "🏛️🧱", "Rust Core"),
        ("enum", "🎲", "Rust Core"),
        ("mod", "📦", "Rust Core"),
        ("use", "🔗", "Rust Core"),
        ("impl", "🔨", "Rust Core"),
        ("trait", "🧩", "Rust Core"),
        ("const", "🔒", "Rust Core"),
        ("static", "🪨", "Rust Core"),
        ("type", "🏷️", "Rust Core"),
        ("ident", "🆔", "Rust Core"),
        ("attrs", "🎨", "Rust Core"),
        ("fields", "🌱", "Rust Core"),
        ("meta", "🧠", "Rust Core"),
        ("path", "🛤️", "Rust Core"),
        ("lit", "💡", "Rust Core"),
        ("tokens", "🎟️", "Rust Core"),
        ("expr", "🧮", "Rust Core"),
        ("block", "🧱", "Rust Core"),
        ("call", "📞", "Rust Core"),
        ("method", "🔧", "Rust Core"),
        ("macro", "🪄", "Rust Core"),
        ("trait_object", "🦋", "Rust Core"),
        ("item", "📜", "Rust Core"),
        ("items", "📚", "Rust Core"),
        ("field", "🌿", "Rust Core"),
        ("inputs", "➡️", "Rust Core"),
        ("output", "⬅️", "Rust Core"),
        ("receiver", "📡", "Rust Core"),
        ("generics", "🔣", "Rust Core"),
        ("lifetime", "⏳", "Rust Core"),
        ("where_clause", "❓", "Rust Core"),
        ("tuple", "🤝", "Rust Core"),
        ("tuple_struct", "🏗️", "Rust Core"),
        ("array", "🔢", "Rust Core"),
        ("int", "#️⃣", "Rust Core"),
        ("float", "💧", "Rust Core"),
        ("bool", "✅", "Rust Core"),
        ("char", "🔤", "Rust Core"),
        ("str", "📝", "Rust Core"),
        ("closure", "🕸️", "Rust Core"),
        ("let", "📌", "Rust Core"),
        ("match", "🎯", "Rust Core"),
        ("if", "❓", "Rust Core"),
        ("else_branch", "🔄", "Rust Core"),
        ("then_branch", "➡️", "Rust Core"),
        ("for_loop", "🔁", "Rust Core"),
        ("while", "🔂", "Rust Core"),
        ("loop", "♾️", "Rust Core"),
        ("return", "↩️", "Rust Core"),
        ("break", "⛔", "Rust Core"),
        ("continue", "▶️", "Rust Core"),
        ("assign", "📝", "Rust Core"),
        ("op", "⚙️", "Rust Core"),
        ("unary", "➖", "Rust Core"),
        ("binary", "➗", "Rust Core"),
        ("cast", "🔀", "Rust Core"),
        ("index", "📍", "Rust Core"),
        ("range", "↔️", "Rust Core"),
        ("slice", "🍰", "Rust Core"),
        ("macro_rules", "📐", "Rust Core"),
        ("group", "👥", "Rust Core"),
        ("delim", "🚧", "Rust Core"),
        ("punct", "‼️", "Rust Core"),
        ("paren", "( )", "Rust Core"),
        ("bracket", "[ ]", "Rust Core"),
        ("brace", "{ }", "Rust Core"),
        ("attr", "🖼️", "Rust Core"),
        ("name_value", "🔑", "Rust Core"),
        ("value", "💎", "Rust Core"),
        ("style", "🎨", "Rust Core"),
        ("method_call", "📲", "Rust Core"),
        ("dyn", "🌀", "Rust Core"),
        ("mut", "🔄", "Rust Core"),
        ("ref", "🔗", "Rust Core"),
        ("self_ty", "🆔", "Rust Core"),
        ("super", "🌟", "Rust Core"),
        ("crate", "🚚", "Rust Core"),
        ("macro_input", "📥", "Rust Core"),
        ("macro_output", "📦", "Rust Core"),
        ("params", "⚙️", "Rust Core"),
        ("args", "📢", "Rust Core"),
        ("arguments", "🎙️", "Rust Core"),
        ("arm", "🛡️", "Rust Core"),
        ("arms", "🛠️", "Rust Core"),
        ("variant", "🎭", "Rust Core"),
        ("variants", "🔣", "Rust Core"),
        ("fields_named", "🏷️", "Rust Core"),
        ("fields_unnamed", "🌿", "Rust Core"),
        ("pat", "🖼️", "Rust Core"),
        ("stmt", "🖋️", "Rust Core"),
        ("stmts", "📜", "Rust Core"),
        ("ty", "🔖", "Rust Core"),
        ("bound", "⛓️", "Rust Core"),
        ("bounds", "🔗", "Rust Core"),
        ("vis", "👀", "Rust Core"),
        ("list", "✅", "Rust Core"),
        ("token", "🎟️", "Rust Core"),
        ("tree", "🌳", "Rust Core"),
        ("segment", "🧩", "Rust Core"),
        ("segments", "🧩", "Rust Core"),
        ("assoc_type", "🔗", "Rust Core"),
        ("async", "⏩", "Rust Core"),
        ("await", "⏳", "Rust Core"),
        ("base", "🏁", "Rust Core"),
        ("body", "🏃", "Rust Core"),
        ("colon_token", ":", "Rust Core"),
        ("delimiter", "🚧", "Rust Core"),
        ("angle_bracketed", "⟨⟩", "Rust Core"),
        ("cond", "❓", "Rust Core"),
        ("func", "🦀", "Rust Core"),
        ("init", "🚦", "Rust Core"),
        ("right", "👉", "Rust Core"),
        ("semi", ";", "Rust Core"),
        ("semi_token", ";", "Rust Core"),
        ("spacing", "↔️", "Rust Core"),
        ("start", "🔜", "Rust Core"),
        ("stream", "🌊", "Rust Core"),
        ("try", "🤞", "Rust Core"),
        ("bare_fn", "🦀", "Rust Core"),
        ("bounded_ty", "📏", "Rust Core"),
        ("byte_str", "💾", "Rust Core"),
        ("cases", "🎭", "Rust Core"),
        ("dot2_token", "•", "Rust Core"),
        ("elem", "📦", "Rust Core"),
        ("elems", "📦", "Rust Core"),
        ("end", "🔚", "Rust Core"),
        ("impl_trait", "🧩", "Rust Core"),
        ("left", "👈", "Rust Core"),
        ("len", "📏", "Rust Core"),
        ("limits", "📏", "Rust Core"),
        ("move", "🚚", "Rust Core"),
        ("named", "🏷️", "Rust Core"),
        ("or", "🔀", "Rust Core"),
        ("parenthesized", "( )", "Rust Core"),
        ("reference", "🔗", "Rust Core"),
        ("rename", "📝", "Rust Core"),
        ("repeat", "🔁", "Rust Core"),
        ("rest", "🔁", "Rust Core"),
        ("restricted", "🚫", "Rust Core"),
        ("turbofish", "🐟", "Rust Core"),
        ("typed", "🏷️", "Rust Core"),
        ("unnamed", "🏷️", "Rust Core"),
        ("unsafe", "☢️", "Rust Core"),
        // Web/CSS
        ("px", "📏", "Web/CSS"), ("deg", "🧭", "Web/CSS"), ("em", "🔠", "Web/CSS"), ("rem", "🔡", "Web/CSS"), ("vh", "📐", "Web/CSS"), ("vw", "📏", "Web/CSS"), ("s", "⏱️", "Web/CSS"), ("ms", "⏲️", "Web/CSS"),
        ("animation", "🎞️", "Web/CSS"), ("transition", "🔄", "Web/CSS"), ("absolute", "📐", "Web/CSS"), ("align", "📏", "Web/CSS"), ("app", "📱", "Web/CSS"), ("app_state", "🗄️", "Web/CSS"), ("accessibility", "♿", "Web/CSS"),
        ("adapter", "🔌", "Web/CSS"), ("actions", "🎬", "Web/CSS"), ("action", "🎬", "Web/CSS"), ("active", "🔥", "Web/CSS"),
        // Crypto/Security/Systems
        ("aead", "🔒", "Crypto"), ("aeads", "🔒", "Crypto"), ("aes", "🔑", "Crypto"), ("argon2", "🧂", "Crypto"), ("arc", "🧲", "Crypto"), ("addr2line", "📍", "Crypto"), ("aarch64", "📦", "Crypto"), ("amd64", "💻", "Crypto"), ("armv8", "💪", "Crypto"),
        ("crypto", "🔒", "Crypto"), ("curve25519", "➰", "Crypto"), ("ed25519", "📝", "Crypto"), ("elliptic", "➰", "Crypto"), ("fiat", "💵", "Crypto"), ("cbor", "📦", "Crypto"),
        // Project-specific
        ("agave", "🌵", "Project-Specific"), ("helius", "🌞", "Project-Specific"),
        // Internationalization
        ("icu4x", "🌐", "Internationalization"), ("cldr", "🌍", "Internationalization"), ("chinese", "🀄", "Internationalization"), ("hebrew", "✡️", "Internationalization"), ("coptic", "⛪", "Internationalization"), ("ethiopic", "🌄", "Internationalization"), ("calendar", "📅", "Internationalization"), ("datetime", "⏰", "Internationalization"),
        // Testing/Benchmarking
        ("criterion", "⏱️", "Testing"), ("benches", "🏋️", "Testing"), ("fuzz", "🧪", "Testing"), ("examples", "📚", "Testing"), ("docs", "📖", "Testing"),
        // Misc/General
        ("algebra", "➗", "General"), ("analysis", "🔍", "General"), ("analyze", "🔬", "General"), ("account", "👤", "General"), ("accounts", "👥", "General"),
        // Suffixes for versioning/hashes
        ("zm", "🧬", "Versioning"), ("h", "⏳", "Versioning"), ("v", "🔢", "Versioning"),
        // Color codes (hex)
        ("ff", "🎨", "Color"), ("00", "⚫", "Color"), ("ffffff", "⬜", "Color"), ("000000", "⬛", "Color"),
        // Numbers (for fun)
        ("0", "0️⃣", "Numbers"), ("1", "1️⃣", "Numbers"), ("2", "2️⃣", "Numbers"), ("3", "3️⃣", "Numbers"), ("4", "4️⃣", "Numbers"), ("5", "5️⃣", "Numbers"), ("6", "6️⃣", "Numbers"), ("7", "7️⃣", "Numbers"), ("8", "8️⃣", "Numbers"), ("9", "9️⃣", "Numbers"), ("10", "🔟", "Numbers"), ("100", "💯", "Numbers"), ("255", "🟧", "Numbers"),
        // Emoji codepoints
        ("1f3a8", "🎨", "Emoji"), ("1f4dd", "📝", "Emoji"), ("1f680", "🚀", "Emoji"), ("1f4a9", "💩", "Emoji"),
        // Heuristic/structural
        ("byte", "💾", "Numbers"), ("parenthes", "( )", "Rust Core"), ("case", "🎭", "Rust Core"), ("dot", "•", "General"), ("colon", ":", "General"), ("bounded", "📏", "General"),
        ("_", "⬜", "Rust Core"), ("colon2_token", ":", "Rust Core"), ("cond", "❓", "Rust Core"), ("content", "📦", "General"), ("if", "❓", "Rust Core"), ("where_clause", "📜", "Rust Core"),
    ];

    for &(_name, emoji, category) in emoji_type_map {
        map.insert(emoji.to_string(), (_name.to_string(), category.to_string()));
    }
    map
}

pub fn get_rust_core_sub_category(name: &str) -> &'static str {
    match name {
        // 1. Declarations & Structure
        "fn" | "struct" | "enum" | "mod" | "use" | "impl" | "trait" | "const" | "static" | "type" |
        "ident" | "attrs" | "fields" | "meta" | "path" | "item" | "items" | "field" | "inputs" |
        "output" | "receiver" | "generics" | "lifetime" | "where_clause" | "tuple" | "tuple_struct" |
        "array" | "bare_fn" | "bounded_ty" | "byte_str" | "cases" | "elem" | "elems" | "impl_trait" |
        "named" | "parenthesized" | "reference" | "rename" | "repeat" | "rest" | "restricted" |
        "typed" | "unnamed" | "unsafe" | "vis" | "list" | "token" | "tree" | "segment" | "segments" |
        "assoc_type" | "body" | "params" | "args" | "arguments" | "arm" | "arms" | "variant" |
        "variants" | "fields_named" | "fields_unnamed" | "pat" | "stmt" | "stmts" | "ty" | "bound" |
        "bounds" => "Rust Core: Declarations & Structure",

        // 2. Literals & Expressions
        "lit" | "tokens" | "expr" | "int" | "float" | "bool" | "char" | "str" | "value" => "Rust Core: Literals & Expressions",

        // 3. Control Flow
        "let" | "match" | "if" | "else_branch" | "then_branch" | "for_loop" | "while" | "loop" |
        "return" | "break" | "continue" | "cond" | "try" => "Rust Core: Control Flow",

        // 4. Macros & Attributes
        "macro" | "macro_rules" | "attr" | "name_value" | "macro_input" | "macro_output" => "Rust Core: Macros & Attributes",

        // 5. Syntax & Delimiters
        "group" | "delim" | "punct" | "paren" | "bracket" | "brace" | "colon_token" | "delimiter" |
        "angle_bracketed" | "semi" | "semi_token" | "spacing" | "dot2_token" | "colon2_token" => "Rust Core: Syntax & Delimiters",

        // 6. Concurrency & Ownership
        "async" | "await" | "dyn" | "mut" | "ref" | "move" => "Rust Core: Concurrency & Ownership",

        // 7. Module & Path Resolution
        "self_ty" | "super" | "crate" | "func" | "init" | "right" | "left" | "start" | "stream" |
        "turbofish" => "Rust Core: Module & Path Resolution",

        _ => "Rust Core: Other", // Fallback for any unclassified Rust Core types
    }
}
