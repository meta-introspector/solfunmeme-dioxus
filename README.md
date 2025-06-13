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
