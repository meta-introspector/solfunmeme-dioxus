use emoji_workflow_macro::emoji_workflow;

#[emoji_workflow("📜🔗")]
fn my_test_workflow() {
    println!("This is my test workflow function!");
}

fn main() {
    my_test_workflow();
}
