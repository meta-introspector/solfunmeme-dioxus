## Solfunmeme Dioxus 

See:
https://stackoverflow.com/questions/55912871/how-to-work-with-openssl-for-rust-within-a-windows-development-environment


```
   
   $env:VCPKG_ROOT="C:\Users\gentd\OneDrive\Documents\GitHub\vcpkg"
   vcpkg  install openssl 
   vcpkg.exe install openssl:x64-windows
   vcpkg.exe install openssl:x64-windows-static
   vcpkg.exe integrate install
   set VCPKGRS_DYNAMIC=1
  
   $env:OPENSSL_DIR="C:\Users\gentd\OneDrive\Documents\GitHub\vcpkg\installed\x64-windows-static"
   cargo build

or in bash
    export OPENSSL_DIR="/c/Users/gentd/OneDrive/Documents/GitHub/vcpkg/installed/x64-windows-static"
    
```



### Requirements
1. This template relies on Tailwind CSS to generate the stylesheet. 

Install the standalone Tailwind CLI - [https://tailwindcss.com/docs/installation/tailwind-cli](https://tailwindcss.com/docs/installation/tailwind-cli)
2. Install Dioxus cli from official website - [https://dioxuslabs.com/](https://dioxuslabs.com/)


### Running the dev server
1. Start the tailwind CLI within the Root of the directory
    ```sh
    tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --watch
    ```
2. Start the Dioxus CLI
    ```sh
    dx serve
    ```

- Open the browser at default port http://localhost:8080 or the port described by Dioxus CLI in case port `8080` was already in use

- Sometimes there are warning in the browser console, use `dx check` command to find if there are fixes that need to be done.


### Plan

#### Client side interpretation.
#### Server side storage
The first memes are just free text stored as instructions in the blockchain
each one can be seen as a rust program that when executed produces some text or json or url
We can parameterize those. the programs can be updated by creating new versions of them
because we are on a sidechain we can rewrite or garbage collect them.

##### Import Git
we can use git submodules or git urls like package managers do,

###### Import Data via git
we can import telegram, discord, twitter, github, ai chat logs, files into the system.
each sidechain becomes its own meme database with all the transactions around a meme coin ecosystem.
see the time repo for example.

######## A tweet becomes a program or meme, 
an anchor is the code.

#### Forking all repos into our chain
#### Copying all data into our archives
#### Running all code on our chain
#### Proving all code to be valid.
#### Showing execution of all paths of the code
using dfa and parsers for the emoji language. 
each version of the meme state is a new version of the language and library.
we hope to construct that each path in the system is a unique word and a program at the same time that has a meaning in emojis.
#### Mathematical modeling of the structures (groups, hott, etc)
#### Using of AI outside the system, storing results on the chain.
##### AI via client side inferences
###### looking at Invoke AI for example
###### looking at Kobold AI for example
###### calling typescript from rust wasm dioxus (calling eliza?)
#### Tracing the AI inference into the model.


#### Convert this source to json
Createing json  using https://github.com/meta-introspector/syn-serde-rust.git

 ```
    cd .\syn-serde-rust\
    cargo build
    cd .\examples\rust2emoji\
    cargo build
    cargo run ..\..\..\solfunmeme-dioxus\
```

#### Reading in source of code via reflection. splitting into chunks. saving in the blockchain.
#### interpreting the functions as json or emojis
#### linking functions together creating systems as groups of contracts.
#### embedding contrats as vectors


## Multiple visualization
### ai convergence on models
#### PCA
#### Threading of dimensions
#### Area or surface of points of the spaces.
#### connecting solana code to memes to llms

