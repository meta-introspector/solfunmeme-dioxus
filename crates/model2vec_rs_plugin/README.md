# `model2vec_rs_plugin`

This crate provides a wrapper around the `model2vec-rs` library, offering functionalities for converting models or data into vector representations.

## Purpose

It enables the Solfunmeme project to embed various entities into a continuous vector space, facilitating machine learning tasks such as similarity search, clustering, and visualization.

## Core Functionalities

-   **Train Model**: Train a model to generate vector embeddings from input data.
-   **Infer Vector**: Generate a vector embedding for a given input.

## Usage (Conceptual)

```rust
use model2vec_rs_plugin::Model2VecRsPlugin;

fn main() {
    let mut plugin = Model2VecRsPlugin::new().expect("Failed to create plugin");
    let data = vec!["text1", "text2", "text3"];

    // Example: Train the model
    // plugin.train_model(&data).expect("Failed to train model");

    // Example: Infer a vector
    // let vector = plugin.infer_vector("some text").expect("Failed to infer vector");
    // println!("Inferred vector: {:?}", vector);
}
```
