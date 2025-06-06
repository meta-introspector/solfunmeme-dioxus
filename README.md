[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/meta-introspector/solfunmeme-dioxus)

## Dioxus Template with Tailwind CSS

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
