use vulkano::{
    instance::{Instance, InstanceCreateInfo, InstanceExtensions},
    VulkanLibrary,
};

fn main() {
    println!("Hello World from Rust + Vulkano!");
    
    let library = match VulkanLibrary::new() {
        Ok(lib) => {
            println!("Vulkan library loaded successfully");
            lib
        },
        Err(e) => {
            println!("Failed to load Vulkan library: {}", e);
            println!("This might be because Vulkan drivers are not installed or available.");
            println!("Hello World application completed (without Vulkan initialization).");
            return;
        }
    };
    
    // Try with portability enumeration for MoltenVK
    let instance = match Instance::new(
        library,
        InstanceCreateInfo {
            enabled_extensions: InstanceExtensions {
                khr_portability_enumeration: true,
                ..InstanceExtensions::empty()
            },
            flags: vulkano::instance::InstanceCreateFlags::ENUMERATE_PORTABILITY,
            ..Default::default()
        },
    ) {
        Ok(instance) => {
            println!("Vulkan instance created with portability enumeration!");
            instance
        },
        Err(e) => {
            println!("Failed with portability enumeration: {}", e);
            println!("Trying without portability enumeration...");
            
            // Try without portability enumeration
            match Instance::new(
                VulkanLibrary::new().unwrap(),
                InstanceCreateInfo {
                    ..Default::default()
                },
            ) {
                Ok(instance) => {
                    println!("Vulkan instance created without portability!");
                    instance
                },
                Err(e2) => {
                    println!("Also failed without portability: {}", e2);
                    println!("Hello World application completed (without Vulkan instance).");
                    return;
                }
            }
        }
    };

    println!("Vulkan instance created successfully!");
    println!("Available layers: {:?}", instance.enabled_layers());
    println!("Available extensions: {:?}", instance.enabled_extensions());
    println!("Vulkano Hello World completed successfully!");
}
