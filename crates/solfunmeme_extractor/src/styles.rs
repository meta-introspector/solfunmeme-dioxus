//mod styles {
pub const STYLE: &str = r#"
.code-extractor {
    max-width: 1200px;
    margin: 0 auto;
    padding: 20px;
    font-family: system-ui, -apple-system, sans-serif;
}

.upload-area {
    border: 2px dashed #ccc;
    border-radius: 8px;
    padding: 40px;
    text-align: center;
    margin: 20px 0;
    transition: all 0.3s ease;
}

.upload-area.drag-over {
    border-color: #007bff;
    background-color: #f8f9fa;
}

.file-input {
    margin: 10px 0;
}

.processing-indicator {
    border: 2px solid #007bff;
    border-radius: 8px;
    padding: 20px;
    margin: 20px 0;
    background: #f8f9fa;
}

.progress-bar {
    width: 100%;
    height: 20px;
    background: #e9ecef;
    border-radius: 10px;
    overflow: hidden;
    margin: 10px 0;
}

.progress-fill {
    height: 100%;
    background: #007bff;
    transition: width 0.3s ease;
}

.snippet-container {
    border: 1px solid #dee2e6;
    border-radius: 8px;
    margin: 15px 0;
    overflow: hidden;
}

.snippet-header {
    background: #f8f9fa;
    padding: 10px 15px;
    border-bottom: 1px solid #dee2e6;
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.snippet-content {
    max-height: 400px;
    overflow-y: auto;
}

.snippet-code {
    margin: 0;
    padding: 15px;
    background: #f6f8fa;
    font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
    font-size: 14px;
    line-height: 1.4;
    white-space: pre-wrap;
    word-wrap: break-word;
}

.copy-btn {
    background: #007bff;
    color: white;
    border: none;
    padding: 5px 12px;
    border-radius: 4px;
    cursor: pointer;
    font-size: 12px;
    transition: background 0.2s;
}

.copy-btn:hover {
    background: #0056b3;
}

.copy-btn.copied {
    background: #28a745;
}

.summary-stats {
    background: #e9ecef;
    padding: 15px;
    border-radius: 8px;
    margin: 20px 0;
}

.clear-btn {
    background: #dc3545;
    color: white;
    border: none;
    padding: 8px 16px;
    border-radius: 4px;
    cursor: pointer;
    margin: 10px 0;
}

.clear-btn:hover {
    background: #c82333;
}

.download-btn {
    background: #17a2b8;
    color: white;
    border: none;
    padding: 8px 16px;
    border-radius: 4px;
    cursor: pointer;
    margin: 10px 5px;
}

.download-btn:hover {
    background: #138496;
}

.button-group {
    display: flex;
    gap: 10px;
    flex-wrap: wrap;
}
"#;

// const STYLE: &str = r#"
// .code-extractor {
//     max-width: 1200px;
//     margin: 0 auto;
//     padding: 20px;
//     font-family: system-ui, -apple-system, sans-serif;
// }

// .upload-area {
//     border: 2px dashed #ccc;
//     border-radius: 8px;
//     padding: 40px;
//     text-align: center;
//     margin: 20px 0;
//     transition: all 0.3s ease;
// }

// .upload-area.drag-over {
//     border-color: #007bff;
//     background-color: #f8f9fa;
// }

// .file-input {
//     margin: 10px 0;
// }

// .processing-indicator {
//     border: 2px solid #007bff;
//     border-radius: 8px;
//     padding: 20px;
//     margin: 20px 0;
//     background: #f8f9fa;
// }

// .progress-bar {
//     width: 100%;
//     height: 20px;
//     background: #e9ecef;
//     border-radius: 10px;
//     overflow: hidden;
//     margin: 10px 0;
// }

// .progress-fill {
//     height: 100%;
//     background: #007bff;
//     transition: width 0.3s ease;
// }

// .snippet-container {
//     border: 1px solid #dee2e6;
//     border-radius: 8px;
//     margin: 15px 0;
//     overflow: hidden;
// }

// .snippet-header {
//     background: #f8f9fa;
//     padding: 10px 15px;
//     border-bottom: 1px solid #dee2e6;
//     display: flex;
//     justify-content: space-between;
//     align-items: center;
// }

// .snippet-content {
//     max-height: 400px;
//     overflow-y: auto;
// }

// .snippet-code {
//     margin: 0;
//     padding: 15px;
//     background: #f6f8fa;
//     font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
//     font-size: 14px;
//     line-height: 1.4;
//     white-space: pre-wrap;
//     word-wrap: break-word;
// }

// .copy-btn {
//     background: #007bff;
//     color: white;
//     border: none;
//     padding: 5px 12px;
//     border-radius: 4px;
//     cursor: pointer;
//     font-size: 12px;
//     transition: background 0.2s;
// }

// .copy-btn:hover {
//     background: #0056b3;
// }

// .copy-btn.copied {
//     background: #28a745;
// }

// .summary-stats {
//     background: #e9ecef;
//     padding: 15px;
//     border-radius: 8px;
//     margin: 20px 0;
// }

// .clear-btn {
//     background: #dc3545;
//     color: white;
//     border: none;
//     padding: 8px 16px;
//     border-radius: 4px;
//     cursor: pointer;
//     margin: 10px 0;
// }

// .clear-btn:hover {
//     background: #c82333;
// }
// "#;
