use dspy_rs::{ChatAdapter, LM, Module, Predict, Signature, configure};
use secrecy::SecretString;

#[cfg(test)]
mod tests {
    use super::*;

    #[Signature]
    struct TestSignature {
        #[input]
        question: String,
        #[output]
        answer: String,
    }

    #[test]
    fn test_signature_creation() {
        // Test that we can create a signature
        let sig = TestSignature::new();
        // Just verify it compiles and can be created
        let _ = sig;
        assert!(true);
    }

    #[test]
    fn test_predictor_creation() {
        // Test that we can create a predictor
        let _predictor = Predict::new(TestSignature::new());
        // Just verify it compiles and can be created
        assert!(true);
    }

    #[tokio::test]
    async fn test_lm_configuration() {
        // Test that we can configure the LM (without actually calling it)
        let api_key = std::env::var("OPENAI_API_KEY")
            .unwrap_or_else(|_| "test-key".to_string());
        
        configure(
            LM::builder()
                .api_key(SecretString::from(api_key))
                .build(),
            ChatAdapter::default(),
        );
        
        // If we get here, configuration worked
        assert!(true);
    }

    #[test]
    fn test_example_macro() {
        // Test the example! macro
        let ex = dspy_rs::example! {
            "question": "input" => "What is 2+2?",
        };
        
        assert!(ex.data.contains_key("question"));
        assert_eq!(ex.data.get("question").unwrap(), "What is 2+2?");
    }
}