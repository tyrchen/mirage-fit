// Remix functionality test example
use mirage_fit::{config::Config, gemini::GeminiClient};
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load config (make sure you have GEMINI_API_KEY set)
    let config = Config::new(None)?;
    let client = GeminiClient::new(config)?;

    // Example 1: Simple text-to-image (no input images)
    println!("Test 1: Text-to-image generation");
    let prompt =
        "Create a stylish black hat on a white background, professional product photography";
    let result = client.remix(prompt, &[]).await?;
    fs::write("test_generated_hat.jpg", result)?;
    println!("✅ Generated hat image saved to test_generated_hat.jpg");

    // Example 2: If you have test images, you can test multi-image remix
    if let (Ok(base_image), Ok(item_image)) = (fs::read("test_base.jpg"), fs::read("test_item.jpg"))
    {
        println!("Test 2: Multi-image remix");
        let remix_prompt = "Take the person from the first image and add the fashion item from the second image to them. Make it look natural and realistic.";
        let images = [base_image.as_slice(), item_image.as_slice()];
        let remix_result = client.remix(remix_prompt, &images).await?;
        fs::write("test_remix_result.jpg", remix_result)?;
        println!("✅ Remix result saved to test_remix_result.jpg");
    } else {
        println!("ℹ️  Test images not found, skipping multi-image remix test");
        println!("   To test multi-image remix, place test_base.jpg and test_item.jpg in the current directory");
    }

    // Example 3: Fashion remix with multiple items (simulated)
    println!("Test 3: Fashion remix prompt (nano-banana style)");
    let fashion_prompt = r#"
Create a photorealistic fashion remix image. Take the person from the first image and apply the following style:
- Add a stylish black hat
- Include trendy glasses
- Apply a modern casual aesthetic
- Maintain natural lighting and realistic proportions
- Ensure the background complements the overall look

The final result should look like a professional fashion photograph suitable for e-commerce use.
    "#.trim();

    // For demo, we'll just generate based on the prompt
    let fashion_result = client.remix(fashion_prompt, &[]).await?;
    fs::write("test_fashion_remix.jpg", fashion_result)?;
    println!("✅ Fashion remix saved to test_fashion_remix.jpg");

    println!("\n🎉 All remix tests completed successfully!");
    println!("Check the generated images in the current directory.");

    Ok(())
}
