// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use image::{GenericImage, GenericImageView, DynamicImage, Rgba};
use uuid::Uuid;
use std::env::temp_dir;

#[tauri::command]
fn extract_image(input_path: &str, polygon: Vec<(u32, u32)>) -> String {
    let img = image::open(input_path).unwrap();
    let bounds = bounding_box(&polygon);
    let cropped = img.crop_imm(bounds.0, bounds.2, (bounds.1 - bounds.0), (bounds.3 - bounds.2));
    let bounded_polygon = bound_polygon(&polygon, bounds.0, bounds.2);
    let mut output_img = DynamicImage::new_rgba8(bounds.1 - bounds.0, bounds.3 - bounds.2);

    for (x, y, pixel) in cropped.pixels() {
        if point_in_polygon((x, y), &bounded_polygon) {
            output_img.put_pixel(x, y, pixel);
        } else {
            output_img.put_pixel(x, y, Rgba([0, 0, 0, 0]));
        }
    }
    let mut dir = temp_dir();
    let file_name = format!("{}.png", Uuid::new_v4());
    dir.push(file_name);
    output_img.save(dir.clone()).unwrap();
    return dir.as_path().display().to_string();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![extract_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn point_in_polygon(point: (u32, u32), polygon: &[(u32, u32)]) -> bool {
    let mut inside = false;
    let (x, y) = point;
    let mut i = 0;
    let mut j = polygon.len() - 1;

    while i < polygon.len() {
        let (xi, yi) = polygon[i];
        let (xj, yj) = polygon[j];
        if (yi > y) != (yj > y) {
            let intersect = xi as i64 + (xj as i64 - xi as i64) * (y as i64 - yi as i64) / (yj as i64 - yi as i64);
            if (x as i64) < intersect {
                inside = !inside;
            }
        }
        j = i;
        i += 1;
    }

    inside
}

fn bounding_box(polygon: &[(u32, u32)]) -> (u32, u32, u32, u32) {
    let (mut xmin, mut ymin) = polygon[0];
    let (mut xmax, mut ymax) = polygon[0];

    for &point in polygon {
        if point.0 < xmin {
            xmin = point.0;
        }
        if point.1 < ymin {
            ymin = point.1;
        }
        if point.0 > xmax {
            xmax = point.0;
        }
        if point.1 > ymax {
            ymax = point.1;
        }
    }

    (xmin, xmax, ymin, ymax)
}

fn bound_polygon(polygon: &[(u32, u32)], xmin: u32, ymin: u32) -> Vec<(u32, u32)> {
    polygon.iter().map(|&point| (point.0.saturating_sub(xmin), point.1.saturating_sub(ymin))).collect()
}
