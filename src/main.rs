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

fn generate_fake_qr(version: u8, qr_message: &str, overlay_message: &str) -> Result<(), String> {
    let qr_code = generate_base_qr(version, qr_message)?;
    let mut qr_image = qr_code
        .render::<Luma<u8>>()
        .module_dimensions(1, 1)
        .quiet_zone(false)
        .build();

    let width = qr_image.width() - 1;
    let overlay_bits = calculate_overlay_bits(overlay_message);

    add_overlay_to_qr(&mut qr_image, &overlay_bits, width);

    qr_image
        .save("fake_qr_with_separator_overlay.png")
        .map_err(|_| "Failed to save the QR code with overlay".to_string())?;
    println!("Fake QR code with separator overlay saved as 'fake_qr_with_separator_overlay.png'.");
    Ok(())
}

fn main() {
    let message = "Enter message to encode in QR";
    let qr_version = 6;

    match generate_qr_with_overlay(qr_version, message) {
        Ok(_) => println!("QR code generation succeeded ✅"),
        Err(e) => println!("Error: {} ❌", e),
    }

    let qr_message = "Enter message to encode in QR";
    let overlay_message = "Incorrect overlay message";
    let qr_version = 6;
    match generate_fake_qr(qr_version, qr_message, overlay_message) {
        Ok(_) => println!("Fake QR code generation succeeded ✅"),
        Err(e) => println!("Error: {} ❌", e),
    }
}
