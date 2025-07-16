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

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/meta-introspector/solfunmeme-dioxus)