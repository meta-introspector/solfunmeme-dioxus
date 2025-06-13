
//Ideas:
//#1. gcc asts, metacoq, lean4 dumper and rust procedureal macros and rust syn parsers produce expressions
pub enum MetaMemes {
	Gcc,
    LLVM,
	MetaCoq,
    Haskell,
    Coq,
    Ocaml,
    Lean4,
    Rust,    
    MetaMeme
}
//2. we can import those asts as "memes" or meta-memes. They are syntactic forms.
pub struct Meme {
    typ : MetaMemes,
    value : &'static str
}

// 3. the interpretation of the expressions as a group of memes is supported (import etc)
pub const META_MEME: Meme = Meme {
    typ: MetaMemes::MetaMeme,
    value: "See MetaMemes enum above",
};
// 4. writing proofs and validating interpretations.

pub fn interpret(m: Meme) {

}

pub fn prove(m: Meme) {

}

// add two memes for context 
pub fn context(cx: Meme , value: Meme) {}

// 6. the memes can be solana pdas, github issues, codeberg issues,
// tweets, basically any rdf that we can construct.
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

pub fn fetch(pr: MemeProviders, url: String) -> Meme {
    // TODO: implement fetching logic
    unimplemented!()
}

// 5. creating llm content from the memes, expanding them. 
pub enum LLMProviders {
	GitHub,
	OpenAI,
    AWSBedrock,
    Google,
    XAi,
    Groq,
    Ollama,
    OpenLightLLM
}

pub fn expand(prov:LLMProviders, m: Meme) -> Meme {

        Meme {
            typ : MetaMemes::MetaMeme,
            value : "fixme",
        }
}

// 7. this structure should only have 8 layers and loop back on itself like bott periodicity. 

pub const ZOS1 : Meme = Meme {
    typ: MetaMemes::MetaMeme,
    value : "ZOS1=[0,1,2,3,5,7,11,13]"
};


pub const ZOS1_array : [i32; 8] = [0, 1, 2, 3, 5, 7, 11, 13];

// 
