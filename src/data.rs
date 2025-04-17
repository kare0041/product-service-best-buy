use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Contoso UltraBass Wireless Earbuds".to_string(),
            price: 79.99,
            description: "Dive into immersive sound with Contoso UltraBass Earbuds. Designed for music lovers, these buds offer crystal-clear audio, touch controls, and a pocket-sized charging case.".to_string(),
            image: "/ultrabass_earbuds.jpg".to_string()
        },
        Product {
            id: 2,
            name: "Salty Sailor Bluetooth Speaker".to_string(),
            price: 49.99,
            description: "Set sail with the Salty Sailor Bluetooth Speaker. Water-resistant and packed with bass, it’s perfect for beach parties and poolside jams.".to_string(),
            image: "/sailor_speaker.jpg".to_string()
        },
        Product {
            id: 3,
            name: "Mermaid's Glow Smart Lamp".to_string(),
            price: 29.99,
            description: "Light up your space with the Mermaid's Glow Smart Lamp. Featuring voice control, mood settings, and RGB colors—it’s ambiance on command.".to_string(),
            image: "/glow_lamp.jpg".to_string()
        },
        Product {
            id: 4,
            name: "Ocean Explorer VR Headset".to_string(),
            price: 199.99,
            description: "Experience the deep blue with Ocean Explorer VR. Whether you're gaming or touring virtual reefs, this headset brings breathtaking visuals to life.".to_string(),
            image: "/vr_headset.jpg".to_string()
        },
        Product {
            id: 5,
            name: "Pirate Parrot Drone".to_string(),
            price: 129.99,
            description: "Take flight with the Pirate Parrot Drone. With built-in 4K camera and obstacle avoidance, it’s your trusty aerial scout for every adventure.".to_string(),
            image: "/parrot_drone.jpg".to_string()
        },
        Product {
            id: 6,
            name: "Seafarer Charging Dock".to_string(),
            price: 24.99,
            description: "Anchor your devices with the Seafarer Charging Dock. This stylish multi-port hub keeps your gadgets charged and organized, even in stormy schedules.".to_string(),
            image: "/charging_dock.jpg".to_string()
        },
        Product {
            id: 7,
            name: "Seashell Comfort Gaming Chair".to_string(),
            price: 189.99,
            description: "Game in luxury with the Seashell Comfort Chair. Ergonomic design, lumbar support, and a reclining shell-inspired frame for epic sessions.".to_string(),
            image: "/gaming_chair.jpg".to_string()
        },
        Product {
            id: 8,
            name: "Nautical Knot USB-C Cable".to_string(),
            price: 12.99,
            description: "Tired of flimsy cables? The Nautical Knot USB-C is built to last—braided, durable, and perfect for both charging and data transfer.".to_string(),
            image: "/usb_cable.jpg".to_string()
        },
        Product {
            id: 9,
            name: "Contoso Claw Gaming Mouse".to_string(),
            price: 59.99,
            description: "Unleash precision with the Contoso Claw Gaming Mouse. RGB lighting, adjustable DPI, and a claw-like grip for the fiercest gamers.".to_string(),
            image: "/gaming_mouse.jpg".to_string()
        },
        Product {
            id: 10,
            name: "AhoyTech Waterproof Smartwatch".to_string(),
            price: 89.99,
            description: "Stay smart and splash-proof with the AhoyTech Watch. Track steps, check messages, and measure heart rate—all from your wrist.".to_string(),
            image: "/smartwatch.jpg".to_string()
        }
        
    ]
}