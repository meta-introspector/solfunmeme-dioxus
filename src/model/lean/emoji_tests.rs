#[cfg(test)]
mod tests {
    use crate::model::lean::emojis::json_to_emoji;
    use serde_json::json;

    //use super::*;

    #[test]
    fn test_json_to_emoji_forall_const_sort() {
        let json = json!({
            "kind": "AsyncConstB",
            "cnstInfB": {
                "sig": {
                    "type": {
            "ForallE" : {
                            //"type": "forallE",
                            "forbndrTypB": null,
                            "forbndrTyp": null,
                            "forbdB": null,
                            "forbd": null,
                            "binderName": "x",
                            "binderInfo": "default"
            }
                    }
                },
                "name": "foo",
                "levelParams": [],
                "kind": { "value": "def", "kind": "constant" },
                "cnstInf": null
            }
        });
        let json_str = serde_json::to_string(&json).unwrap();
        let emoji = json_to_emoji(&json_str).unwrap();
        assert!(emoji.contains("∀ x (default:"));
    }

    #[test]
    fn test_json_to_emoji_const_with_levels() {
        let json = json!({
            "kind": "AsyncConstB",
            "cnstInfB": {
                "sig": {
                    "type": {
                        // "type": "const",
            "Const" : {
                            "levels": [
                { "Param" :  "u"  },
                { "Param" : "v"  },
                            ],
                            "declName": "bar"
            }
                    }
                },
                "name": "bar",
                "levelParams": [],
                "kind": { "value": "def", "kind": "constant" },
                "cnstInf": null
            }
        });
        let json_str = serde_json::to_string(&json).unwrap();
        let emoji = json_to_emoji(&json_str).unwrap();
        assert!(emoji.contains("🔖 bar [u,v]"));
    }

    #[test]
    fn test_json_to_emoji_const_no_levels() {
        let json = json!({
                        "kind": "AsyncConstB",
                        "cnstInfB": {
                            "sig": {
                                "type": {
                        "Const": {
        //                            "type": "const",
                                    "levels": [],
                        "declName": "no_levels"
                        }
                                }
                            },
                            "name": "no_levels",
                            "levelParams": [],
                            "kind": { "value": "def", "kind": "constant" },
                            "cnstInf": null
                        }
                    });
        let json_str = serde_json::to_string(&json).unwrap();
        let emoji = json_to_emoji(&json_str).unwrap();
        assert!(emoji.contains("🔖 no_levels []"));
    }

    #[test]
    fn test_json_to_emoji_lam_with_type_and_body() {
        let json = json!({
            "kind": "AsyncConstB",
            "cnstInfB": {
                "sig": {
                    "type": {
            "Lam" : {
            "binderName": "a",
            "binderType": {
                "Sort" : {
                "level": { "Param": "l" }
                }
            },
            "body": {
                                "BVar" :{}
            },

            "binderInfo": "default"
                        }
        }
                },
        "name": "lam_with_type",
                "levelParams": [],
                "kind": { "value": "def", "kind": "constant" },
                "cnstInf": null
            }
        });
        let json_str = serde_json::to_string(&json).unwrap();
        //println!("{}",json_str);
        println!("DEBUG:\n{}", json_str);
        let emoji = json_to_emoji(&json_str).unwrap();
        assert!(emoji.contains("λ a (default:"));
        assert!(emoji.contains("📏 l"));
        assert!(emoji.contains("📍"));
    }

    #[test]
    fn test_json_to_emoji_app_nested() {
        let json = json!({
            "kind": "AsyncConstB",
            "cnstInfB": {
                "sig": {
                    "type": {
            "App" :  {
            "fn": {
                                "App": {
                "fn": {
                    "BVar": {}
                },
                "arg": {
                    "BVar" : {}
                }
                }
            },
            "arg": {
                                "BVar": {}
            }
            }
                    }
                },
                "name": "app_nested",
                "levelParams": [],
                "kind": { "value": "def", "kind": "constant" },
                "cnstInf": null
            }
        });
        let json_str = serde_json::to_string(&json).unwrap();
        let emoji = json_to_emoji(&json_str).unwrap();
        assert!(emoji.matches("➡️").count() >= 1);
        assert!(emoji.matches("📍").count() >= 3);
    }

    // #[test]
    // fn test_json_to_emoji_with_multiple_rules() {
    //     let json = json!({
    //         "kind": "AsyncConstB",
    //         "cnstInfB": {
    //             "sig": {
    //                 "type": {
    //                     "type": "sort",
    //                     "level": { "level": "l", "kind": "param" }
    //                 }
    //             },
    //             "name": "multi_rules",
    //             "levelParams": [],
    //             "kind": { "value": "def", "kind": "constant" },
    //             "cnstInf": {
    //                 "type": {
    // 		    "Sort":
    // 		    {
    // 			"level": { "Param": "l" },
    // 			"numParams": 2,
    // 			"numMotives": 1,
    // 			"numMinors": 0,
    // 			"numIndices": 0,
    // 			"name": "multi_rules",
    // 			"levelParams": [],
    // 			"kind": "def",
    // 			"k": false,
    // 			"isUnsafe": false,
    // 			"all": [],
    // 			"Rules": [
    // 			    {
    // 				"rhs": { "type": "bvar" },
    // 				"nfields": 1,
    // 				"name": "ruleA",
    // 				"kind": "rule"
    // 			    },
    // 			    {
    // 				"rhs": { "type": "sort", "level": { "level": "m", "kind": "param" } },
    // 				"nfields": 3,
    // 				"name": "ruleB",
    // 				"kind": "rule"
    // 			    }
    // 			]
    // 		    }
    // 		}
    // 	    }
    // 	}
    //     });
    //     let json_str = serde_json::to_string(&json).unwrap();
    //     println!("DEBUG:\n{}",json_str);
    //     let emoji = json_to_emoji(&json_str).unwrap();
    //     assert!(emoji.contains("📜 Rules:"));
    //     assert!(emoji.contains("📋 ruleA (fields: 1)"));
    //     assert!(emoji.contains("📋 ruleB (fields: 3)"));
    //     assert!(emoji.contains("📏 m"));
    // }

    #[test]
    fn test_json_to_emoji_sort() {
        let json = json!({
            "kind": "AsyncConstB",
            "cnstInfB": {
                "sig": {
                    "type": {
                        //"type": "sort",
            "Sort" : {
                            "level": { "Param":  "l"  }
            }
                    }
                },
                "name": "baz",
                "levelParams": [],
                "kind": { "value": "def", "kind": "constant" },
                "cnstInf": null
            }
        });
        let json_str = serde_json::to_string(&json).unwrap();
        let emoji = json_to_emoji(&json_str).unwrap();
        assert!(emoji.contains("📏 l"));
    }

    // TODO: we can regenerate valid examples ourselves later
    // #[test]
    // fn test_json_to_emoji_app_lam() {
    //     let json = json!({
    //         "kind": "AsyncConstB",
    //         "cnstInfB": {
    //             "sig": {
    //                 "type": {
    // 			"App": {
    //                     "fn": {
    // 			    "Lam": {
    // 				"body": {},
    // 				"binderName": "y",
    // 				"binderInfo": "default"
    // 			    }
    //                     },
    //                     "arg": {
    //                         "BVar" : {}
    //                     }
    // 			}
    //                 }
    //             },
    //             "name": "qux",
    //             "levelParams": [],
    //             "kind": { "value": "def", "kind": "constant" },

    //         }
    //     });
    //     let json_str = serde_json::to_string(&json).unwrap();
    // 	println!("DEBUG:\n{}",json_str);
    //     let emoji = json_to_emoji(&json_str).unwrap();
    //     assert!(emoji.contains("➡️"));
    //     assert!(emoji.contains("λ y (default:"));
    //     assert!(emoji.contains("📍"));
    // }

    // #[test]
    // fn test_json_to_emoji_with_rules() {
    //     let json = json!({
    //         "kind": "AsyncConstB",
    //         "cnstInfB": {
    //             "sig": {
    //                 "type": {
    //                     "Sort": {
    //                         "level": { "Param": "l" }
    // 			}
    //                 }
    //             },
    //             "name": "with_rules",
    //             "levelParams": [],
    //             "kind": { "value": "def", "kind": "constant" },
    //             "cnstInf": {
    //                 "type": {
    // 			"Sort" : {
    // 			    "level": { "Param": "l"} ,
    // 			    "numParams": 1,
    // 			    "numMotives": 1,
    // 			    "numMinors": 0,
    // 			    "numIndices": 0,
    // 			    "name": "with_rules",
    // 			    "levelParams": [],
    // 			    "kind": "def",
    // 			    "k": false,
    // 			    "isUnsafe": false,
    // 			    "all": [],
    // 			    "Rules": [
    // 				{
    // 				    "rhs": {
    // 					"type": "bvar"
    // 				    },
    // 				    "nfields": 2,
    // 				    "name": "rule1",
    // 				    "kind": "rule"
    // 				}
    // 			    ]
    // 			}
    // 		    }
    // 		}
    // 	    }
    //     });
    //     let json_str = serde_json::to_string(&json).unwrap();
    // 	println!("DEBUG:\n{}",json_str);
    //     let emoji = json_to_emoji(&json_str).unwrap();
    //     assert!(emoji.contains("📜 Rules:"));
    //     assert!(emoji.contains("📋 rule1 (fields: 2)"));
    //     assert!(emoji.contains("📍"));
    // }

    #[test]
    fn test_json_to_emoji_forall_with_nested_lam() {
        let json = json!({
            "kind": "AsyncConstB",
            "cnstInfB": {
                "sig": {
                    "type": {
            "ForallE": {
                        "forbndrTypB": null,
                        "forbndrTyp": null,
                        "forbdB": {
                            "type": "lam",
                            "lambndrTpB": null,
                            "lambndrTp": null,
                            "lambdB": null,
                            "lambd": null,
                            "binderName": "z",
                            "binderInfo": "implicit"
                        },
                        "forbd": null,
                        "binderName": "x",
                        "binderInfo": "default"
                        }
        }
                },
                "name": "nested_forall_lam",
                "levelParams": [],
                "kind": { "value": "def", "kind": "constant" },
                "cnstInf": null
            }
        });
        let json_str = serde_json::to_string(&json).unwrap();
        let emoji = json_to_emoji(&json_str).unwrap();
        println!("DEBUG:\n{}", emoji);
        //            assert!(emoji.contains("∀ x (default:"));
        //assert!(emoji.contains("λ z (implicit:"));
    }
}
