use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "ASUS TUF Gaming F15".to_string(),
            price: 1299.99,
            description: "Powerful 11th Gen Intel Core i7 laptop with RTX 3050 Ti, designed for high performance gaming and multitasking.".to_string(),
            image: "/asus_tuf.jpg".to_string()
        },
        Product {
            id: 2,
            name: "MacBook Pro 14-inch M2".to_string(),
            price: 1999.99,
            description: "Apple M2 Pro chip, Liquid Retina XDR display, and up to 18 hours of battery life. Ideal for creative professionals.".to_string(),
            image: "/macbook_pro.jpg".to_string()
        },
        Product {
            id: 3,
            name: "Lenovo Legion 5 Pro".to_string(),
            price: 1599.99,
            description: "AMD Ryzen 7 + NVIDIA RTX 3070, QHD 165Hz display, engineered for competitive gaming and content creation.".to_string(),
            image: "/lenovo_legion.jpg".to_string()
        },
        Product {
            id: 4,
            name: "Dell XPS 13".to_string(),
            price: 1099.99,
            description: "Compact, lightweight, and elegant ultrabook with 11th Gen Intel Evo platform and stunning InfinityEdge display.".to_string(),
            image: "/dell_xps13.jpg".to_string()
        },
        Product {
            id: 5,
            name: "HP Spectre x360 14".to_string(),
            price: 1399.00,
            description: "Versatile 2-in-1 design with OLED touchscreen, Thunderbolt 4, and exceptional battery life for mobile productivity.".to_string(),
            image: "/hp_spectre.jpg".to_string()
        },
        Product {
            id: 6,
            name: "Microsoft Surface Laptop 5".to_string(),
            price: 1299.00,
            description: "Sleek and elegant laptop with 12th Gen Intel Core, PixelSense display, and lightweight design.".to_string(),
            image: "/surface_laptop.jpg".to_string()
        },
        Product {
            id: 7,
            name: "Acer Swift X".to_string(),
            price: 999.00,
            description: "Portable powerhouse with Ryzen 7, RTX 3050, and 100% sRGB display, ideal for editing and AI workloads.".to_string(),
            image: "/acer_swift.jpg".to_string()
        },
        Product {
            id: 8,
            name: "Razer Blade 15".to_string(),
            price: 2199.99,
            description: "Thin and powerful gaming laptop with RTX 4070 and 240Hz QHD display for immersive AAA gameplay.".to_string(),
            image: "/razer_blade.jpg".to_string()
        },
        Product {
            id: 9,
            name: "Framework Laptop DIY Edition".to_string(),
            price: 1049.00,
            description: "Modular and repairable laptop with fully customizable components, made for developers and engineers.".to_string(),
            image: "/framework.jpg".to_string()
        },
        Product {
            id: 10,
            name: "MSI Creator Z16".to_string(),
            price: 1799.00,
            description: "High-end laptop for creators with 16:10 touchscreen, RTX 3060 GPU, and superb color accuracy.".to_string(),
            image: "/msi_creator.jpg".to_string()
        }
    ]
}