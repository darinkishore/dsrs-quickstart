use dspy_rs::DummyPredict;
use dspy_rs::Module;
use dspy_rs::Signature;
use dspy_rs::data::example::Example;
use dspy_rs::hashmap;

#[allow(dead_code)]
#[Signature]
struct QASignature {
    /// You are a helpful assistant.

    #[input]
    pub question: String,

    #[output]
    pub answer: String,
}

#[cfg_attr(miri, ignore)] // Miri doesn't support tokio's I/O driver
#[tokio::test]
async fn test_predictor() {
    let predictor = DummyPredict {};
    let inputs = Example::new(
        hashmap! {
            "question".to_string() => "What is the capital of France?".to_string().into(),
            "answer".to_string() => "Paris".to_string().into(),
        },
        vec!["question".to_string()],
        vec!["answer".to_string()],
    );

    let outputs = predictor.forward(inputs.clone()).await.unwrap();

    assert_eq!(outputs.get("answer", None), "Paris");
}
