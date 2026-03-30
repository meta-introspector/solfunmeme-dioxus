use serde::{Deserialize, Serialize};
//Ideas:
//#1. gcc asts, metacoq, lean4 dumper and rust procedureal macros and rust syn parsers produce expressions
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MetaMemes {
    Gcc,
    LLVM,
    MetaCoq,
    Haskell,
    Coq,
    Ocaml,
    Lean4,
    Rust,
    MetaMeme,
}
//2. we can import those asts as "memes" or meta-memes. They are syntactic forms.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Meme {
    typ: MetaMemes,
    value: &'static str,
}

impl Meme {
    #[allow(dead_code)]
    pub fn get_value(&self) -> &str {
        self.value
    }
}

// 3. the interpretation of the expressions as a group of memes is supported (import etc)
#[allow(dead_code)]
pub const META_MEME: Meme = Meme {
    typ: MetaMemes::MetaMeme,
    value: "See MetaMemes enum above",
};
// 4. writing proofs and validating interpretations.
#[allow(dead_code)]
pub fn interpret(_m: Meme) {
    // #FIXME #8 continue
}
#[allow(dead_code)]
pub fn prove(_m: Meme) {}
// add two memes for context
#[allow(dead_code)]
pub fn context(_cx: Meme, _value: Meme) {}

// 6. the memes can be solana pdas, github issues, codeberg issues,
// tweets, basically any rdf that we can construct.
#[allow(dead_code)]
pub enum MemeProviders {
    GitHub,
    CodeBerg,
    Discord,
    GitGud,
    GitLab,
    GitTea,
    Forjo,
    AwsCodeCommit,
    LocalGit,
    Twitter,
    Telegram,
    Matrix,
    Irc,
}
#[allow(dead_code)]
pub fn fetch(_pr: MemeProviders, _url: String) -> Meme {
    // TODO: implement fetching logic
    unimplemented!()
}

// 5. creating llm content from the memes, expanding them.
#[allow(dead_code)]
pub enum LLMProviders {
    GitHub,
    OpenAI,
    AWSBedrock,
    Google,
    XAi,
    Groq,
    Ollama,
    OpenLightLLM,
}

#[allow(dead_code)]
pub fn expand(_prov: LLMProviders, _m: Meme) -> Meme {
    Meme {
        typ: MetaMemes::MetaMeme,
        value: "fixme",
    }
}

// 7. this structure should only have 8 layers and loop back on itself like bott periodicity.

#[allow(dead_code)]
pub const ZOS1: Meme = Meme {
    typ: MetaMemes::MetaMeme,
    value: "ZOS1=[0,1,2,3,5,7,11,13]",
};

#[allow(dead_code)]
pub const ZOS1_ARRAY: [i32; 8] = [0, 1, 2, 3, 5, 7, 11, 13];

//

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MemeUniverse {
    // Variants from MetaMemes (AST/syntactic forms)
    Gcc,
    LLVM,
    MetaCoq,
    Haskell,
    Coq,
    Ocaml,
    Lean4,
    Rust,
    MetaMeme,

    // Variants from MemeProviders (sources of memes)
    GitHub,
    CodeBerg,
    Discord,
    GitGud,
    GitLab,
    GitTea,
    Forjo,
    AwsCodeCommit,
    LocalGit,
    Twitter,
    Telegram,
    Matrix,
    Irc,

    // Variants from LLMProviders (LLM content expansion)
    LLMGitHub,
    OpenAI,
    AWSBedrock,
    Google,
    XAi,
    Groq,
    Ollama,
    OpenLightLLM,

    // Meme struct as a variant (capturing type and value)
    Meme {
        typ: Box<MemeUniverse>, // Recursive reference to MemeUniverse
        value: &'static str,
    },

    // Constants as variants
    ZOS1 {
        typ: Box<MemeUniverse>,
        value: &'static str,
    },
    ZOS1Array([i32; 8]),
}

impl MemeUniverse {
    #[allow(dead_code)]
    pub fn get_value(&self) -> Option<&str> {
        match self {
            MemeUniverse::Meme { value, .. } => Some(value),
            MemeUniverse::ZOS1 { value, .. } => Some(value),
            _ => None,
        }
    }
}

// // Constants defined using the enum
// #[allow(dead_code)]
// pub const META_MEME2: MemeUniverse = MemeUniverse::Meme {
//     typ: Box::from(MemeUniverse::MetaMeme),
//     value: "See MemeUniverse enum above",
// };

// #[allow(dead_code)]
// pub const ZOS12: MemeUniverse = MemeUniverse::ZOS1 {
//     typ: Box::from(MemeUniverse::MetaMeme),
//     value: "ZOS1=[0,1,2,3,5,7,11,13]",
// };

#[allow(dead_code)]
pub const ZOS1_ARRAY2: MemeUniverse = MemeUniverse::ZOS1Array([0, 1, 2, 3, 5, 7, 11, 13]);
