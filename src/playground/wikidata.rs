//use serde_wasm_bindgen;
// async fn fetch_wikidata_graph() -> Result<Value, String> {
//     let query = r#"
//         SELECT ?item ?itemLabel ?property ?propertyLabel ?value ?valueLabel WHERE {
//             VALUES ?item { wd:Q21168966 wd:Q107027826 wd:Q11900058 wd:Q607862 wd:Q1751856 }
//             ?item ?prop ?value .
//             ?property wikibase:directClaim ?prop .
//             SERVICE wikibase:label { bd:serviceParam wikibase:language "en". }
//         }
//         LIMIT 50
//     "#;
//     let mut opts = RequestInit::new();
//     opts.method("GET");
//     opts.mode(RequestMode::Cors);
//     let url = format!("https://query.wikidata.org/sparql?query={}&format=json", encode(query));
//     let request = Request::new_with_str_and_init(&url, &opts)
//         .map_err(|e| format!("Failed to create request: {:?}", e))?;
//     let window = window().ok_or("No global `window` exists")?;
//     let resp = JsFuture::from(window.fetch_with_request(&request))
//         .await
//         .map_err(|e| format!("Fetch failed: {:?}", e))?;
//     let resp: Response = resp.dyn_into().map_err(|e| format!("Invalid response: {:?}", e))?;
//     let json = JsFuture::from(resp.json().map_err(|e| format!("Failed to get JSON: {:?}", e))?)
//         .await
//         .map_err(|e| format!("JSON parse failed: {:?}", e))?;
//     let json_value: Value = serde_wasm_bindgen::from_value(json)
//         .map_err(|e| format!("Deserialization failed: {:?}", e))?;
//     Ok(json_value)
// }
