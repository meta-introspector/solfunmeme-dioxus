;;; Emacs Lisp code for refactoring Cargo.toml to include solfunmeme_http_client

(defun create-and-update-solfunmeme-http-client ()
  "Creates the solfunmeme_http_client crate and updates Cargo.toml."
  (interactive)

  ;; Define paths
  (let* ((project-root (file-name-directory (buffer-file-name)))
         (crate-dir (expand-file-name "crates/solfunmeme_http_client/" project-root))
         (src-dir (expand-file-name "src/" crate-dir))
         (cargo-toml (expand-file-name "Cargo.toml" crate-dir))
         (lib-rs (expand-file-name "lib.rs" src-dir))
         (root-cargo-toml (expand-file-name "Cargo.toml" project-root)))

    ;; 1. Create crate directory
    (make-directory src-dir t)
    (message "Created directory: %s" src-dir)

    ;; 2. Create Cargo.toml for the new crate
    (with-temp-file cargo-toml
      (insert "[package]\n")
      (insert "name = \"solfunmeme_http_client\"\n")
      (insert "version = \"0.1.0\"\n")
      (insert "edition = \"2021\"\n\n")
      (insert "[dependencies]\n")
      (insert "reqwest = { version = \"0.12.2\", features = [\"blocking\", \"json\"] }\n"))
    (message "Created file: %s" cargo-toml)

    ;; 3. Create lib.rs for the new crate
    (with-temp-file lib-rs
      (insert "pub use reqwest;\n"))
    (message "Created file: %s" lib-rs)

    ;; 4. Update root Cargo.toml
    (with-current-buffer (find-file-noselect root-cargo-toml)
      (goto-char (point-min))
      ;; Remove old reqwest dependency
      (when (re-search-forward "^reqwest = { version = \"0\\.12\\.2\", features = \[\"blocking\", \"json\"\] }\\s-*$" nil t)
        (replace-match "" nil t))
      ;; Add new solfunmeme_http_client dependency
      (goto-char (point-min))
      (when (re-search-forward "^solfunmeme_utility_deps = { path = \"crates/solfunmeme_utility_deps\" }\\s-*$" nil t)
        (replace-match "solfunmeme_utility_deps = { path = \"crates/solfunmeme_utility_deps\" }\nsolfunmeme_http_client = { path = \"crates/solfunmeme_http_client\" }" nil t))
      ;; Add to workspace members
      (goto-char (point-min))
      (when (re-search-forward "^\\s*\"crates/solfunmeme_rdf_utils\",\\s-*$" nil t)
        (replace-match "    \"crates/solfunmeme_rdf_utils\",\n    \"crates/solfunmeme_http_client\"," nil t))
      (save-buffer))
    (message "Updated file: %s" root-cargo-toml)
    (message "Refactoring for solfunmeme_http_client complete.")))

;; To run this code in Emacs:
;; 1. Save it to a file (e.g., refactor.el).
;; 2. Open Emacs.
;; 3. M-x load-file RET /path/to/refactor.el RET
;; 4. M-x create-and-update-solfunmeme-http-client RET
