// ============================================================
//  sensor.rs — Objek Sensor Suhu
//  Merepresentasikan sensor fisik di tiap zona storage
// ============================================================

#[derive(Debug, Clone)]
pub struct Sensor {
    pub id: u32,
    pub name: String,
    pub zone: String,
    pub value: f64,        // nilai suhu terbaca (°C)
    pub min_range: f64,    // batas bawah spesifikasi alat
    pub max_range: f64,    // batas atas spesifikasi alat
    pub is_active: bool,
    pub history: Vec<f64>, // riwayat pembacaan untuk moving average
}

impl Sensor {
    /// Membuat sensor baru
    pub fn new(id: u32, name: &str, zone: &str, min_range: f64, max_range: f64) -> Self {
        Sensor {
            id,
            name: name.to_string(),
            zone: zone.to_string(),
            value: 0.0,
            min_range,
            max_range,
            is_active: true,
            history: Vec::new(),
        }
    }

    /// Set nilai suhu baru dan simpan ke history
    pub fn set_value(&mut self, val: f64) {
        self.value = val;
        self.history.push(val);
        // Batasi history hanya 10 data terakhir
        if self.history.len() > 10 {
            self.history.remove(0);
        }
    }

    /// Cek apakah nilai suhu dalam range valid sensor
    pub fn is_valid(&self) -> bool {
        self.value >= self.min_range && self.value <= self.max_range
    }

    /// Tampilkan informasi sensor
    pub fn display(&self) {
        println!(
            "  [{:>3}] {:<20} | Zona: {:<8} | Suhu: {:>6.2}°C | Status Alat: {}",
            self.id,
            self.name,
            self.zone,
            self.value,
            if self.is_active { "AKTIF" } else { "MATI" }
        );
    }

    /// Nonaktifkan sensor
    pub fn deactivate(&mut self) {
        self.is_active = false;
        println!("  ⚠  Sensor {} ({}) dinonaktifkan.", self.id, self.name);
    }
}
