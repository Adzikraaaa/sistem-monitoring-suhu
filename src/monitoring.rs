// ============================================================
//  monitoring.rs — Objek MonitoringSystem
//  Mengintegrasikan Sensor + Controller + Komputasi Numerik
// ============================================================

use crate::sensor::Sensor;
use crate::controller::{Controller, StatusSuhu};
use crate::numerik::{get_latest_ma, rata_rata, nilai_min, nilai_max, simpangan_standar, error_pengukuran};

#[derive(Debug)]
pub struct DataLog {
    pub sesi: u32,
    pub zona: String,
    pub suhu_raw: f64,
    pub suhu_ma: f64,
    pub status: String,
}

pub struct MonitoringSystem {
    pub nama_perusahaan: String,
    pub sensors: Vec<Sensor>,
    pub controller: Controller,
    pub log: Vec<DataLog>,
    pub sesi: u32,
    pub ma_window: usize,  // ukuran window moving average
}

impl MonitoringSystem {
    /// Buat sistem monitoring baru
    pub fn new(nama: &str, ctrl: Controller, window: usize) -> Self {
        MonitoringSystem {
            nama_perusahaan: nama.to_string(),
            sensors: Vec::new(),
            controller: ctrl,
            log: Vec::new(),
            sesi: 0,
            ma_window: window,
        }
    }

    /// Tambah sensor ke sistem
    pub fn add_sensor(&mut self, sensor: Sensor) {
        println!("  ✅ Sensor '{}' (Zona {}) berhasil ditambahkan.", sensor.name, sensor.zone);
        self.sensors.push(sensor);
    }

    /// Jalankan satu sesi monitoring (baca semua sensor, evaluasi, alarm)
    pub fn run_session(&mut self, inputs: Vec<f64>) {
        self.sesi += 1;
        self.print_header();
        println!("  Sesi Monitoring #{}", self.sesi);
        println!("{}", "─".repeat(75));

        // Header tabel
        println!(
            "  {:<5} {:<20} {:<10} {:>10} {:>10} {:<16}",
            "ID", "Nama Sensor", "Zona", "Suhu(°C)", "MA(°C)", "Status"
        );
        println!("{}", "─".repeat(75));

        let mut any_alarm = false;

        for (i, sensor) in self.sensors.iter_mut().enumerate() {
            // Set nilai baru dari input
            if i < inputs.len() {
                sensor.set_value(inputs[i]);
            }

            // Hitung moving average dari history sensor
            let ma = get_latest_ma(&sensor.history, self.ma_window);

            // Cek status menggunakan nilai MA (lebih stabil dari raw)
            let status = {
                let ctrl = &self.controller;
                ctrl.check_status(ma, sensor.is_valid() && sensor.is_active)
            };

            // Cetak baris tabel
            println!(
                "  {:<5} {:<20} {:<10} {:>9.2} {:>9.2}  {}",
                sensor.id,
                sensor.name,
                sensor.zone,
                sensor.value,
                ma,
                status.label()
            );

            // Simpan ke log
            self.log.push(DataLog {
                sesi: self.sesi,
                zona: sensor.zone.clone(),
                suhu_raw: sensor.value,
                suhu_ma: ma,
                status: status.label().trim().to_string(),
            });

            // Tampilkan alarm jika bukan normal
            if status != StatusSuhu::Normal {
                any_alarm = true;
            }

            // Trigger alarm & kontrol
            let zona = sensor.zone.clone();
            self.controller.trigger_alarm(&zona, ma, &status);
        }

        println!("{}", "─".repeat(75));
        if !any_alarm {
            println!("  ✅ Semua zona dalam kondisi NORMAL.");
        }

        self.print_statistics();
    }

    /// Cetak statistik akhir sesi
    fn print_statistics(&self) {
        let values: Vec<f64> = self.sensors.iter().map(|s| s.value).collect();
        if values.is_empty() {
            return;
        }

        println!("\n  📊 Statistik Sesi #{}", self.sesi);
        println!("{}", "─".repeat(45));
        println!("  Rata-rata semua zona : {:.2}°C", rata_rata(&values));
        println!("  Suhu tertinggi       : {:.2}°C", nilai_max(&values));
        println!("  Suhu terendah        : {:.2}°C", nilai_min(&values));
        println!("  Simpangan standar    : {:.4}°C", simpangan_standar(&values));
        println!("  Total alarm aktif    : {} kali", self.controller.alarm_count);
        println!("{}", "─".repeat(45));
    }

    /// Tampilkan laporan lengkap seluruh log
    pub fn print_full_report(&self) {
        self.print_header();
        println!("  📋 LAPORAN MONITORING LENGKAP");
        println!("{}", "═".repeat(75));
        println!(
            "  {:<6} {:<12} {:>10} {:>10} {:<16}",
            "Sesi", "Zona", "Raw(°C)", "MA(°C)", "Status"
        );
        println!("{}", "─".repeat(75));

        for entry in &self.log {
            println!(
                "  {:<6} {:<12} {:>9.2} {:>9.2}  {}",
                entry.sesi, entry.zona, entry.suhu_raw, entry.suhu_ma, entry.status
            );
        }

        println!("{}", "─".repeat(75));

        // Statistik keseluruhan
        let all_raw: Vec<f64> = self.log.iter().map(|l| l.suhu_raw).collect();
        if !all_raw.is_empty() {
            println!("\n  📈 RINGKASAN KESELURUHAN ({} data)", all_raw.len());
            println!("  Rata-rata  : {:.2}°C", rata_rata(&all_raw));
            println!("  Minimum    : {:.2}°C", nilai_min(&all_raw));
            println!("  Maksimum   : {:.2}°C", nilai_max(&all_raw));
            println!("  Std Dev    : {:.4}°C", simpangan_standar(&all_raw));
            println!("  Total Alarm: {} kali", self.controller.alarm_count);
        }
        println!("{}", "═".repeat(75));
    }

    /// Demo komputasi numerik: tampilkan detail MA dan error pengukuran
    pub fn print_numerik_demo(&self) {
        self.print_header();
        println!("  🔢 DEMO KOMPUTASI NUMERIK");
        println!("{}", "═".repeat(65));

        for sensor in &self.sensors {
            if sensor.history.len() < 2 {
                continue;
            }
            println!("\n  Sensor: {} (Zona {})", sensor.name, sensor.zone);
            println!("  History data  : {:?}", sensor.history);

            let ma = crate::numerik::moving_average(&sensor.history, self.ma_window);
            println!("  Moving Average (window={}): ", self.ma_window);
            for (i, v) in ma.iter().enumerate() {
                println!("    Data ke-{}: {:.4}°C", i + 1, v);
            }

            let ma_val = get_latest_ma(&sensor.history, self.ma_window);
            let err = error_pengukuran(sensor.value, ma_val);
            println!("  Nilai terkini : {:.2}°C", sensor.value);
            println!("  Nilai MA      : {:.4}°C", ma_val);
            println!("  Error relatif : {:.4}%", err);
        }
        println!("{}", "═".repeat(65));
    }

    /// Header tampilan program
    pub fn print_header(&self) {
        println!("\n{}", "═".repeat(75));
        println!("  SISTEM MONITORING SUHU STORAGE INDUSTRI");
        println!("  {}", self.nama_perusahaan);
        println!(
            "  Threshold: {:.1}°C — {:.1}°C  |  Window MA: {}  |  Sensor: {} unit",
            self.controller.normal_min,
            self.controller.normal_max,
            self.ma_window,
            self.sensors.len()
        );
        println!("{}", "═".repeat(75));
    }

    /// Tampilkan daftar sensor terdaftar
    pub fn list_sensors(&self) {
        self.print_header();
        println!("  Daftar Sensor Terdaftar:");
        println!("{}", "─".repeat(65));
        for s in &self.sensors {
            s.display();
        }
        println!("{}", "─".repeat(65));
    }

    /// Tampilkan konfigurasi controller
    pub fn show_config(&self) {
        self.print_header();
        println!("  Konfigurasi Controller:");
        println!("{}", "─".repeat(45));
        self.controller.display_config();
        println!("{}", "─".repeat(45));
    }
}
