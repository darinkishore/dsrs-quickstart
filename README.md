# DSRs Quickstart

Start building awesome LLM pipelines in minutes with DSRs (dspy-rs)!

This quickstart project demonstrates a simple but powerful LLM integration using the DSRs framework, with the library vendored for immediate use.

## Prerequisites

- A working Rust toolchain
- An [OpenAI API key](https://platform.openai.com/api-keys)

## Getting Started

### Clone the Repository

```bash
git clone https://github.com/darinkishore/dsrs-quickstart.git
cd dsrs-quickstart
```

### Setup

1. **Set Your API Key**
   
   Edit the `.env` file and add your OpenAI API key:
   ```bash
   OPENAI_API_KEY="sk-your-actual-key-here"
   ```

2. **Run the Example**
   
   ```bash
   cargo run
   ```

   You should see output similar to:
   ```
   Asking the LLM...
   Answer: "The highest mountain in North America is Denali."
   ```

## What's Happening?

This quickstart demonstrates the core DSRs concepts:

1. **Signatures**: Typed contracts that define what the LLM should do
2. **Language Model Configuration**: One-time setup with your API credentials
3. **Predictors**: Modules that execute signatures against the LLM
4. **Examples**: Structured input data for your predictions
5. **Predictions**: Strongly-typed outputs from the LLM

## Project Structure

```
dsrs-quickstart/
├── src/
│   └── main.rs           # Main example code
├── vendor/
│   └── (DSRs library)    # Vendored DSRs dependency
├── .env                  # Your API key (gitignored)
├── .gitignore           # Git ignore rules
├── Cargo.toml           # Rust dependencies
└── README.md            # This file
```

## Next Steps

- Modify the `GeographyQA` signature to ask different types of questions
- Create new signatures for your specific use cases
- Explore more advanced DSRs features in the vendored library documentation

## About DSRs

DSRs (dspy-rs) is a Rust implementation of the DSPy framework, bringing structured, type-safe LLM interactions to the Rust ecosystem. The library is vendored in this project for immediate use without external dependencies.