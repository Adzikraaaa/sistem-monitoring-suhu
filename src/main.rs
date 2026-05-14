// ============================================================
//  main.rs — Entry Point
//  Sistem Monitoring Suhu Storage Industri
//  ETS Algoritma dan Pemrograman — Teknik Instrumentasi ITS
// ============================================================

#![allow(dead_code)]
mod sensor;
mod controller;
mod monitoring;
mod numerik;

use std::io::{self, Write};
use sensor::Sensor;
use controller::Controller;
use monitoring::MonitoringSystem;
use numerik::kalibrasi;

fn main() {
    // ── Inisialisasi Sistem ──────────────────────────────────
    let ctrl = Controller::new(
        "Cold Room Storage — PT. Industri Indonesia Hebat",
        2.0,   // suhu minimum normal (°C) — standar cold room farmasi
        8.0,   // suhu maksimum normal (°C)
        2.0,   // deviasi warning sebelum danger
    );

    let mut sistem = MonitoringSystem::new(
        "PT. Industri Indonesia Hebat — Gudang Penyimpanan B3",
        ctrl,
        5, // window moving average = 5 data
    );

    // ── Daftarkan Sensor ─────────────────────────────────────
    // Sensor range: -40°C s/d 85°C (spesifikasi sensor industri)
    sistem.add_sensor(Sensor::new(1, "Sensor Zona A-01", "Zona A", -40.0, 85.0));
    sistem.add_sensor(Sensor::new(2, "Sensor Zona B-01", "Zona B", -40.0, 85.0));
    sistem.add_sensor(Sensor::new(3, "Sensor Zona C-01", "Zona C", -40.0, 85.0));
    sistem.add_sensor(Sensor::new(4, "Sensor Zona D-01", "Zona D", -40.0, 85.0));

    println!("\n  ✅ Sistem berhasil diinisialisasi dengan {} sensor.", sistem.sensors.len());

    // ── Loop Menu Utama ──────────────────────────────────────
    loop {
        tampilkan_menu();
        let pilihan = baca_input_string("  Masukkan pilihan: ");

        match pilihan.trim() {
            "1" => menu_monitoring_manual(&mut sistem),
            "2" => menu_simulasi_otomatis(&mut sistem),
            "3" => sistem.print_full_report(),
            "4" => sistem.print_numerik_demo(),
            "5" => menu_kalibrasi(),
            "6" => sistem.list_sensors(),
            "7" => sistem.show_config(),
            "0" => {
                println!("\n  👋 Program dihentikan. Terima kasih.\n");
                break;
            }
            _ => println!("  ❌ Pilihan tidak valid. Coba lagi."),
        }
    }
}

// ── Menu ─────────────────────────────────────────────────────

fn tampilkan_menu() {
    println!("\n{}", "═".repeat(55));
    println!("  MENU UTAMA — MONITORING SUHU STORAGE INDUSTRI");
    println!("{}", "═".repeat(55));
    println!("  [1] Input Data Sensor Manual");
    println!("  [2] Simulasi Otomatis (Skenario Preset)");
    println!("  [3] Lihat Laporan Lengkap");
    println!("  [4] Demo Komputasi Numerik (Moving Average)");
    println!("  [5] Demo Kalibrasi Sensor");
    println!("  [6] Daftar Sensor");
    println!("  [7] Konfigurasi Sistem");
    println!("  [0] Keluar");
    println!("{}", "═".repeat(55));
}

// ── Menu 1: Input Manual ─────────────────────────────────────

fn menu_monitoring_manual(sistem: &mut MonitoringSystem) {
    sistem.print_header();
    println!("  📥 INPUT DATA SENSOR MANUAL");
    println!("{}", "─".repeat(55));

    let mut inputs: Vec<f64> = Vec::new();

    for sensor in &sistem.sensors {
        loop {
            let prompt = format!(
                "  Masukkan suhu {} (Zona {}) [°C]: ",
                sensor.name, sensor.zone
            );
            let raw = baca_input_string(&prompt);
            match raw.trim().parse::<f64>() {
                Ok(val) => {
                    // Validasi range sensor
                    if val < sensor.min_range || val > sensor.max_range {
                        println!(
                            "  ⚠  Nilai {:.2}°C di luar range sensor ({:.0}°C — {:.0}°C). Tetap dimasukkan sebagai error.",
                            val, sensor.min_range, sensor.max_range
                        );
                    }
                    inputs.push(val);
                    break;
                }
                Err(_) => println!("  ❌ Input tidak valid. Masukkan angka desimal (contoh: 4.5)"),
            }
        }
    }

    sistem.run_session(inputs);
}

// ── Menu 2: Simulasi Otomatis ─────────────────────────────────

fn menu_simulasi_otomatis(sistem: &mut MonitoringSystem) {
    sistem.print_header();
    println!("  🤖 SIMULASI OTOMATIS — Pilih Skenario:");
    println!("{}", "─".repeat(55));
    println!("  [1] Kondisi Normal Semua Zona");
    println!("  [2] Warning: Zona B mendekati batas atas");
    println!("  [3] Danger: Zona C terlalu panas");
    println!("  [4] Error Sensor: Zona D bermasalah");
    println!("  [5] Skenario Multi-Sesi (5 putaran otomatis)");
    println!("{}", "─".repeat(55));

    let pilihan = baca_input_string("  Pilih skenario: ");

    match pilihan.trim() {
        "1" => {
            println!("\n  ▶ Skenario: Semua zona NORMAL");
            sistem.run_session(vec![4.2, 5.1, 3.8, 6.0]);
        }
        "2" => {
            println!("\n  ▶ Skenario: Zona B WARNING suhu tinggi");
            sistem.run_session(vec![4.5, 8.3, 4.0, 5.2]);
        }
        "3" => {
            println!("\n  ▶ Skenario: Zona C DANGER suhu terlalu panas");
            sistem.run_session(vec![4.1, 5.0, 12.5, 3.9]);
        }
        "4" => {
            println!("\n  ▶ Skenario: Sensor Zona D ERROR (nilai di luar range)");
            // Nilai 200°C jauh di luar range spesifikasi sensor → error
            sistem.run_session(vec![4.3, 5.5, 4.8, 200.0]);
        }
        "5" => simulasi_multi_sesi(sistem),
        _ => println!("  ❌ Pilihan tidak valid."),
    }
}

/// Simulasi 5 sesi otomatis dengan data bertahap (suhu Zona C naik perlahan)
fn simulasi_multi_sesi(sistem: &mut MonitoringSystem) {
    println!("\n  ▶ Simulasi Multi-Sesi: Zona C mengalami kenaikan suhu bertahap");
    println!("  (Mensimulasikan kerusakan pendingin di Zona C)\n");

    // Data simulasi: suhu Zona C naik dari normal → warning → danger
    let skenario: Vec<Vec<f64>> = vec![
        vec![4.0, 5.0, 4.5, 3.8],   // Sesi 1: semua normal
        vec![4.1, 5.2, 5.8, 4.0],   // Sesi 2: Zona C mulai naik
        vec![4.2, 5.1, 7.2, 3.9],   // Sesi 3: Zona C mendekati batas
        vec![4.0, 5.3, 9.1, 4.1],   // Sesi 4: Zona C warning
        vec![3.9, 5.0, 11.5, 4.2],  // Sesi 5: Zona C danger!
    ];

    for (i, data) in skenario.iter().enumerate() {
        println!("\n  ┌─ Sesi {} dari {} ─────────────────────────────", i + 1, skenario.len());
        sistem.run_session(data.clone());
        println!("  └─────────────────────────────────────────────────");

        if i < skenario.len() - 1 {
            tunggu_enter("  Tekan Enter untuk lanjut ke sesi berikutnya...");
        }
    }

    println!("\n  ✅ Simulasi multi-sesi selesai.");
}

// ── Menu 5: Demo Kalibrasi ────────────────────────────────────

fn menu_kalibrasi() {
    println!("\n{}", "═".repeat(55));
    println!("  🔧 DEMO KALIBRASI SENSOR SEDERHANA");
    println!("{}", "═".repeat(55));
    println!("  Rumus: nilai_terkalibrasi = (nilai_raw × gain) + offset");
    println!("{}", "─".repeat(55));

    // Demo kalibrasi dengan contoh data
    let contoh = vec![
        (2.0_f64, 0.98_f64, 0.1_f64),   // (raw, gain, offset)
        (5.5, 1.02, -0.2),
        (8.0, 0.99, 0.05),
        (10.3, 1.01, -0.1),
    ];

    println!(
        "  {:<12} {:<8} {:<10} {:<14}",
        "Raw (°C)", "Gain", "Offset", "Terkalibrasi"
    );
    println!("{}", "─".repeat(50));

    for (raw, gain, offset) in &contoh {
        let terkalibrasi = kalibrasi(*raw, *offset, *gain);
        println!(
            "  {:<12.2} {:<8.3} {:<10.3} {:.4}°C",
            raw, gain, offset, terkalibrasi
        );
    }

    println!("{}", "─".repeat(50));
    println!("\n  Kalibrasi Interaktif:");
    println!("{}", "─".repeat(40));

    let raw = baca_input_f64("  Masukkan nilai raw sensor (°C): ");
    let gain = baca_input_f64("  Masukkan nilai gain         : ");
    let offset = baca_input_f64("  Masukkan nilai offset (°C)  : ");

    let hasil = kalibrasi(raw, offset, gain);
    let selisih = (hasil - raw).abs();

    println!("{}", "─".repeat(40));
    println!("  Nilai raw          : {:.4}°C", raw);
    println!("  Setelah kalibrasi  : {:.4}°C", hasil);
    println!("  Koreksi            : {:.4}°C", selisih);
    println!("{}", "═".repeat(55));
}

// ── Helper I/O ────────────────────────────────────────────────

fn baca_input_string(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn baca_input_f64(prompt: &str) -> f64 {
    loop {
        let raw = baca_input_string(prompt);
        match raw.trim().parse::<f64>() {
            Ok(val) => return val,
            Err(_) => println!("  ❌ Input tidak valid. Masukkan angka (contoh: 4.5)"),
        }
    }
}

fn tunggu_enter(prompt: &str) {
    baca_input_string(prompt);
}
