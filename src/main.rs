// src/main.rs
use dspy_rs::{
    ChatAdapter, LM, Module, Predict, Signature, configure, example
};
use anyhow::Result;
use secrecy::SecretString;

// 1. Define a "Signature"
// This is a typed contract for what the LLM should do.
// The doc comment becomes the main instruction.
#[Signature]
/// Answer questions about geography.
struct GeographyQA {
    #[input]
    question: String,
    #[output]
    answer: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // 2. Configure the Language Model (one-time setup)
    let api_key = std::env::var("OPENAI_API_KEY")
        .expect("OPENAI_API_KEY must be set in your .env file");
    
    configure(
        LM::builder()
            .api_key(SecretString::from(api_key))
            .build(),
        ChatAdapter::default(),
    );
    
    // 3. Create a "Predictor" module for our signature
    let qa_predictor = Predict::new(GeographyQA::new());
    
    // 4. Prepare an "Example" with our input data
    let user_question = example! {
        "question": "input" => "What is the highest mountain in North America?",
    };
    
    // 5. Run the prediction
    println!("Asking the LLM...");
    let result = qa_predictor.forward(user_question).await?;
    
    // 6. Use the strongly-typed "Prediction" output
    println!("\nAnswer: {}", result.data.get("answer").unwrap());
    
    Ok(())
}