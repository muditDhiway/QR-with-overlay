// use image::Luma;
// use qrcode::{EcLevel, QrCode, Version};
// use sha2::{Digest, Sha256};

// fn generate_qr_with_overlay(version: u8, message: &str) -> Result<(), String> {
//     if version < 3 || version > 40 {
//         return Err("Version must be between 3 and 40".to_string());
//     }

//     // Generate base QR
//     let code = QrCode::with_version(message, Version::Normal(version.into()), EcLevel::H)
//         .map_err(|_| "Failed to generate QR code")?;
//     let mut qr_image = code
//         .render::<Luma<u8>>()
//         .module_dimensions(1, 1)
//         .quiet_zone(false)
//         .build();

//     let mut width = qr_image.width(); 
//     let finder_size = 7; // Finder pattern is always 7x7 modules

//     // Hash the message and convert to bit string
//     let mut hasher = Sha256::new();
//     hasher.update(message.as_bytes());
//     let hash = hasher.finalize();
//     let bit_string: Vec<u8> = hash
//         .iter()
//         .flat_map(|byte| (0..8).rev().map(move |bit| (byte >> bit) & 1))
//         .collect();
//     let overlay_bits = &bit_string[0..45]; // First 45 bits

//     // Closure to overlay bits in the separator
//     let mut overlay_bits_on_separator =
//         |bits: &[u8], start_x: u32, start_y: u32, stop_x: u32, stop_y: u32, direction: &str| {
//             let mut bit_index = 0;
//             if direction == "vertical" {
//                 if start_y <= stop_y {
//                     // Top to bottom
//                     for y in start_y..=stop_y {
//                         let x = start_x; // Vertical line means constant x
//                         if bit_index < bits.len() {
//                             let overlay_color = if bits[bit_index] == 1 { 180 } else { 255 };
//                             qr_image.put_pixel(x, y, Luma([overlay_color]));
//                             bit_index += 1;
//                         }
//                     }
//                 } else {
//                     // Bottom to top
//                     for y in (stop_y..=start_y).rev() {
//                         let x = start_x;
//                         if bit_index < bits.len() {
//                             let overlay_color = if bits[bit_index] == 1 { 180 } else { 255 };
//                             qr_image.put_pixel(x, y, Luma([overlay_color]));
//                             bit_index += 1;
//                         }
//                     }
//                 }
//             } else if direction == "horizontal" {
//                 if start_x <= stop_x {
//                     // Left to right
//                     for x in start_x..=stop_x {
//                         let y = start_y; // Horizontal line means constant y
//                         if bit_index < bits.len() {
//                             let overlay_color = if bits[bit_index] == 1 { 180 } else { 255 };
//                             qr_image.put_pixel(x, y, Luma([overlay_color]));
//                             bit_index += 1;
//                         }
//                     }
//                 } else {
//                     // Right to left
//                     for x in (stop_x..=start_x).rev() {
//                         let y = start_y;
//                         if bit_index < bits.len() {
//                             let overlay_color = if bits[bit_index] == 1 { 180 } else { 255 };
//                             qr_image.put_pixel(x, y, Luma([overlay_color]));
//                             bit_index += 1;
//                         }
//                     }
//                 }
//             }
//         };

//     // as coordinates end at (width-1) and not width
//     width -= 1;

//     // Call the function for each separator region
//     // Right of bottom left finder pattern
//     overlay_bits_on_separator(
//         &overlay_bits[0..8],
//         finder_size as u32,
//         width as u32,
//         finder_size as u32,
//         (width - finder_size) as u32,
//         "vertical",
//     );
//     // top of bottom left finder pattern
//     overlay_bits_on_separator(
//         &overlay_bits[8..15],
//         (finder_size - 1) as u32,
//         (width - finder_size) as u32,
//         0,
//         (width - finder_size) as u32,
//         "horizontal",
//     );
//     // bottom of top left finder pattern
//     overlay_bits_on_separator(
//         &overlay_bits[15..23],
//         0,
//         finder_size as u32,
//         finder_size as u32,
//         finder_size as u32,
//         "horizontal",
//     );
//     // right of top left finder pattern
//     overlay_bits_on_separator(
//         &overlay_bits[23..30],
//         finder_size as u32,
//         (finder_size - 1) as u32,
//         finder_size as u32,
//         0,
//         "vertical",
//     );
//     // left of top right finder pattern
//     overlay_bits_on_separator(
//         &overlay_bits[30..38],
//         (width - finder_size) as u32,
//         0,
//         (width - finder_size) as u32,
//         finder_size as u32,
//         "vertical",
//     );
//     // bottom of top right finder pattern
//     overlay_bits_on_separator(
//         &overlay_bits[38..45],
//         (width - finder_size + 1) as u32,
//         finder_size as u32,
//         width as u32,
//         finder_size as u32,
//         "horizontal",
//     );

//     // Save the QR code
//     qr_image
//         .save("qr_with_separator_overlay.png")
//         .map_err(|_| "Failed to save the QR code with overlay".to_string())?;

//     println!("QR code with separator overlay saved as 'qr_with_separator_overlay.png'.");
//     Ok(())
// }

// fn main() {
//     // Enter QR message here
//     let message = "Your QR message here";
//     // Enter QR version here
//     let qr_version = 5;

//     // generate QR with overlay
//     match generate_qr_with_overlay(qr_version, message) {
//         Ok(_) => println!("QR code generation succeeded ✅"),
//         Err(e) => println!("Error: {} ❌", e),
//     }
// }

use image::Luma;
use qrcode::{EcLevel, QrCode, Version};
use sha2::{Digest, Sha256};

fn generate_base_qr(version: u8, message: &str) -> Result<QrCode, String> {
    if version < 3 || version > 40 {
        return Err("Version must be between 3 and 40".to_string());
    }
    QrCode::with_version(message, Version::Normal(version.into()), EcLevel::H)
        .map_err(|_| "Failed to generate QR code".to_string())
}

fn calculate_overlay_bits(message: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(message.as_bytes());
    let hash = hasher.finalize();
    hash.iter()
        .flat_map(|byte| (0..8).rev().map(move |bit| (byte >> bit) & 1))
        .collect()
}

fn overlay_bits_on_separator(
    qr_image: &mut image::GrayImage,
    bits: &[u8],
    start_x: u32,
    start_y: u32,
    stop_x: u32,
    stop_y: u32,
    direction: &str,
) {
    let mut bit_index = 0;
    if direction == "vertical" {
        if start_y <= stop_y {
            for y in start_y..=stop_y {
                if bit_index >= bits.len() {
                    break;
                }
                let overlay_color = if bits[bit_index] == 1 { 180 } else { 255 };
                qr_image.put_pixel(start_x, y, Luma([overlay_color]));
                bit_index += 1;
            }
        } else {
            for y in (stop_y..=start_y).rev() {
                if bit_index >= bits.len() {
                    break;
                }
                let overlay_color = if bits[bit_index] == 1 { 180 } else { 255 };
                qr_image.put_pixel(start_x, y, Luma([overlay_color]));
                bit_index += 1;
            }
        }
    } else if direction == "horizontal" {
        if start_x <= stop_x {
            for x in start_x..=stop_x {
                if bit_index >= bits.len() {
                    break;
                }
                let overlay_color = if bits[bit_index] == 1 { 180 } else { 255 };
                qr_image.put_pixel(x, start_y, Luma([overlay_color]));
                bit_index += 1;
            }
        } else {
            for x in (stop_x..=start_x).rev() {
                if bit_index >= bits.len() {
                    break;
                }
                let overlay_color = if bits[bit_index] == 1 { 180 } else { 255 };
                qr_image.put_pixel(x, start_y, Luma([overlay_color]));
                bit_index += 1;
            }
        }
    }
}

fn add_overlay_to_qr(qr_image: &mut image::GrayImage, overlay_bits: &[u8], width: u32) {
    let finder_size = 7;

    // Overlay bits for each separator region
    overlay_bits_on_separator(
        qr_image,
        &overlay_bits[0..8],
        finder_size as u32,
        width as u32,
        finder_size as u32,
        (width - finder_size) as u32,
        "vertical",
    );
    overlay_bits_on_separator(
        qr_image,
        &overlay_bits[8..15],
        (finder_size - 1) as u32,
        (width - finder_size) as u32,
        0,
        (width - finder_size) as u32,
        "horizontal",
    );
    overlay_bits_on_separator(
        qr_image,
        &overlay_bits[15..23],
        0,
        finder_size as u32,
        finder_size as u32,
        finder_size as u32,
        "horizontal",
    );
    overlay_bits_on_separator(
        qr_image,
        &overlay_bits[23..30],
        finder_size as u32,
        (finder_size - 1) as u32,
        finder_size as u32,
        0,
        "vertical",
    );
    overlay_bits_on_separator(
        qr_image,
        &overlay_bits[30..38],
        (width - finder_size) as u32,
        0,
        (width - finder_size) as u32,
        finder_size as u32,
        "vertical",
    );
    overlay_bits_on_separator(
        qr_image,
        &overlay_bits[38..45],
        (width - finder_size + 1) as u32,
        finder_size as u32,
        width as u32,
        finder_size as u32,
        "horizontal",
    );
}

fn generate_qr_with_overlay(version: u8, message: &str) -> Result<(), String> {
    let qr_code = generate_base_qr(version, message)?;
    let mut qr_image = qr_code
        .render::<Luma<u8>>()
        .module_dimensions(1, 1)
        .quiet_zone(false)
        .build();

    let width = qr_image.width() - 1;
    let overlay_bits = calculate_overlay_bits(message);

    add_overlay_to_qr(&mut qr_image, &overlay_bits, width);

    qr_image
        .save("qr_with_separator_overlay.png")
        .map_err(|_| "Failed to save the QR code with overlay".to_string())?;
    println!("QR code with separator overlay saved as 'qr_with_separator_overlay.png'.");
    Ok(())
}

fn main() {
    let message = "Enter message to encode in QR";
    let qr_version = 6;

    match generate_qr_with_overlay(qr_version, message) {
        Ok(_) => println!("QR code generation succeeded ✅"),
        Err(e) => println!("Error: {} ❌", e),
    }
}