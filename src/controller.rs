// ============================================================
//  controller.rs — Objek Controller
//  Logika pengambilan keputusan, alarm, dan kontrol pendingin
// ============================================================

/// Status kondisi suhu
#[derive(Debug, Clone, PartialEq)]
pub enum StatusSuhu {
    Normal,
    WarningTinggi,
    WarningRendah,
    DangerTinggi,
    DangerRendah,
    SensorError,
}

impl StatusSuhu {
    pub fn label(&self) -> &str {
        match self {
            StatusSuhu::Normal       => "✓ NORMAL      ",
            StatusSuhu::WarningTinggi => "⚠ WARN TINGGI ",
            StatusSuhu::WarningRendah => "⚠ WARN RENDAH ",
            StatusSuhu::DangerTinggi  => "✗ DANGER PANAS",
            StatusSuhu::DangerRendah  => "✗ DANGER DINGIN",
            StatusSuhu::SensorError   => "! SENSOR ERROR",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Controller {
    pub nama_storage: String,
    pub normal_min: f64,      // batas bawah suhu normal (°C)
    pub normal_max: f64,      // batas atas suhu normal (°C)
    pub warning_dev: f64,     // deviasi sebelum danger (°C)
    pub cooling_active: bool, // status pendingin
    pub heater_active: bool,  // status pemanas
    pub alarm_count: u32,     // jumlah alarm yang pernah terjadi
}

impl Controller {
    /// Buat controller baru dengan threshold suhu
    pub fn new(nama: &str, min: f64, max: f64, dev: f64) -> Self {
        Controller {
            nama_storage: nama.to_string(),
            normal_min: min,
            normal_max: max,
            warning_dev: dev,
            cooling_active: false,
            heater_active: false,
            alarm_count: 0,
        }
    }

    /// Evaluasi status suhu berdasarkan nilai moving average
    pub fn check_status(&self, suhu: f64, sensor_valid: bool) -> StatusSuhu {
        if !sensor_valid {
            return StatusSuhu::SensorError;
        }
        if suhu > self.normal_max + self.warning_dev {
            StatusSuhu::DangerTinggi
        } else if suhu > self.normal_max {
            StatusSuhu::WarningTinggi
        } else if suhu < self.normal_min - self.warning_dev {
            StatusSuhu::DangerRendah
        } else if suhu < self.normal_min {
            StatusSuhu::WarningRendah
        } else {
            StatusSuhu::Normal
        }
    }

    /// Tampilkan pesan alarm sesuai status
    pub fn trigger_alarm(&mut self, zona: &str, suhu: f64, status: &StatusSuhu) {
        match status {
            StatusSuhu::WarningTinggi => {
                self.alarm_count += 1;
                println!(
                    "  ⚠  [WARNING] Zona {} — Suhu {:.2}°C mendekati batas atas ({:.1}°C)",
                    zona, suhu, self.normal_max
                );
            }
            StatusSuhu::WarningRendah => {
                self.alarm_count += 1;
                println!(
                    "  ⚠  [WARNING] Zona {} — Suhu {:.2}°C mendekati batas bawah ({:.1}°C)",
                    zona, suhu, self.normal_min
                );
            }
            StatusSuhu::DangerTinggi => {
                self.alarm_count += 1;
                println!(
                    "  🚨 [DANGER]  Zona {} — Suhu {:.2}°C MELEBIHI batas atas! Pendingin diaktifkan!",
                    zona, suhu
                );
                self.control_cooling(true);
            }
            StatusSuhu::DangerRendah => {
                self.alarm_count += 1;
                println!(
                    "  🚨 [DANGER]  Zona {} — Suhu {:.2}°C TERLALU RENDAH! Pemanas diaktifkan!",
                    zona, suhu
                );
                self.control_heater(true);
            }
            StatusSuhu::SensorError => {
                self.alarm_count += 1;
                println!(
                    "  ❌ [ERROR]   Zona {} — Sensor error! Nilai di luar range alat.",
                    zona
                );
            }
            StatusSuhu::Normal => {
                // Matikan semua sistem kontrol jika semua zona normal
                if self.cooling_active {
                    self.control_cooling(false);
                }
                if self.heater_active {
                    self.control_heater(false);
                }
            }
        }
    }

    /// Kendali pendingin otomatis
    pub fn control_cooling(&mut self, on: bool) {
        if self.cooling_active != on {
            self.cooling_active = on;
            println!(
                "  🌡  Sistem Pendingin: {}",
                if on { "DINYALAKAN 🟢" } else { "DIMATIKAN 🔴" }
            );
        }
    }

    /// Kendali pemanas otomatis
    pub fn control_heater(&mut self, on: bool) {
        if self.heater_active != on {
            self.heater_active = on;
            println!(
                "  🔥 Sistem Pemanas: {}",
                if on { "DINYALAKAN 🟢" } else { "DIMATIKAN 🔴" }
            );
        }
    }

    /// Tampilkan konfigurasi threshold
    pub fn display_config(&self) {
        println!("  Storage      : {}", self.nama_storage);
        println!("  Rentang Normal : {:.1}°C  —  {:.1}°C", self.normal_min, self.normal_max);
        println!("  Deviasi Warning: ±{:.1}°C dari batas", self.warning_dev);
        println!(
            "  Danger Zone  : < {:.1}°C  atau  > {:.1}°C",
            self.normal_min - self.warning_dev,
            self.normal_max + self.warning_dev
        );
        println!("  Total Alarm  : {} kali", self.alarm_count);
    }
}
