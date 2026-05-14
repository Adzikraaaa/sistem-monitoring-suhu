// ============================================================
//  numerik.rs — Komputasi Numerik
//  Moving Average, Statistik, Kalibrasi, Error Pengukuran
// ============================================================

/// Hitung Moving Average dari slice data
/// Window size = jumlah data yang dipakai untuk rata-rata
pub fn moving_average(data: &[f64], window: usize) -> Vec<f64> {
    if data.is_empty() || window == 0 {
        return vec![];
    }

    let mut result = Vec::new();
    for i in 0..data.len() {
        if i + 1 < window {
            // Belum cukup data, pakai rata-rata dari data yang ada
            let sum: f64 = data[..=i].iter().sum();
            result.push(sum / (i + 1) as f64);
        } else {
            let sum: f64 = data[(i + 1 - window)..=i].iter().sum();
            result.push(sum / window as f64);
        }
    }
    result
}

/// Ambil nilai moving average terakhir dari history sensor
pub fn get_latest_ma(history: &[f64], window: usize) -> f64 {
    if history.is_empty() {
        return 0.0;
    }
    let ma = moving_average(history, window);
    *ma.last().unwrap_or(&0.0)
}

/// Hitung rata-rata dari slice data
pub fn rata_rata(data: &[f64]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    data.iter().sum::<f64>() / data.len() as f64
}

/// Hitung nilai minimum
pub fn nilai_min(data: &[f64]) -> f64 {
    data.iter().cloned().fold(f64::INFINITY, f64::min)
}

/// Hitung nilai maksimum
pub fn nilai_max(data: &[f64]) -> f64 {
    data.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
}

/// Hitung simpangan standar (standard deviation)
pub fn simpangan_standar(data: &[f64]) -> f64 {
    if data.len() < 2 {
        return 0.0;
    }
    let mean = rata_rata(data);
    let variance = data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / data.len() as f64;
    variance.sqrt()
}

/// Kalibrasi sensor sederhana menggunakan offset dan gain
/// Rumus: nilai_terkalibrasi = (nilai_raw * gain) + offset
pub fn kalibrasi(nilai_raw: f64, offset: f64, gain: f64) -> f64 {
    (nilai_raw * gain) + offset
}

/// Hitung error pengukuran relatif (%)
/// error = |nilai_sensor - nilai_referensi| / nilai_referensi * 100
pub fn error_pengukuran(nilai_sensor: f64, nilai_referensi: f64) -> f64 {
    if nilai_referensi == 0.0 {
        return 0.0;
    }
    ((nilai_sensor - nilai_referensi).abs() / nilai_referensi.abs()) * 100.0
}

/// Interpolasi linear antara dua titik
/// Digunakan untuk estimasi suhu antara dua titik kalibrasi
pub fn interpolasi_linear(x: f64, x0: f64, y0: f64, x1: f64, y1: f64) -> f64 {
    if (x1 - x0).abs() < 1e-10 {
        return y0;
    }
    y0 + (x - x0) * (y1 - y0) / (x1 - x0)
}

// ──── Unit Test ────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moving_average() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let ma = moving_average(&data, 3);
        // window 3: [1, 1.5, 2, 3, 4]
        assert!((ma[2] - 2.0).abs() < 1e-9);
        assert!((ma[4] - 4.0).abs() < 1e-9);
    }

    #[test]
    fn test_rata_rata() {
        let data = vec![2.0, 4.0, 6.0];
        assert!((rata_rata(&data) - 4.0).abs() < 1e-9);
    }

    #[test]
    fn test_kalibrasi() {
        // gain=1, offset=0.5 → 10.0 + 0.5 = 10.5
        assert!((kalibrasi(10.0, 0.5, 1.0) - 10.5).abs() < 1e-9);
    }
}
