use gamma_vk::VulkanContext;

fn main() {
    println!("Hello World from Gamma-VK!");

    match VulkanContext::new() {
        Ok(context) => {
            println!("Vulkan context created successfully!");
            println!("Available layers: {:?}", context.enabled_layers());
            println!("Available extensions: {:?}", context.enabled_extensions());
            println!("Gamma-VK Hello World completed successfully!");
        }
        Err(e) => {
            println!("Failed to create Vulkan context: {}", e);
            println!("This might be because Vulkan drivers are not installed or available.");
            println!("Hello World application completed (without Vulkan initialization).");
        }
    }
}
