Sistem Monitoring Suhu Storage Industri

Aplikasi monitoring suhu berbasis terminal menggunakan bahasa Rust.  
ETS Algoritma dan Pemrograman — Teknik Instrumentasi ITS 2025/2026

**Anggota Kelompok**
- Ibeth Novvalent Hizbu Syawalullah Santos — NRP 2042251063
- Adzikra Maulana Fabiano Ramadhan — NRP 2042251069

**Deskripsi**
Sistem ini memonitor suhu cold room storage industri farmasi  
dengan 4 zona sensor, dilengkapi alarm otomatis dan moving average filter.

**Threshold Suhu**
| Status | Range |
| Normal | 2.0°C — 8.0°C |
| Warning | 0.0°C — 2.0°C atau 8.0°C — 10.0°C |
| Danger | < 0.0°C atau > 10.0°C |

Cara Menjalankan
**Clone repository**
git clone https://github.com/USERNAME/sistem-monitoring-suhu.git

**Masuk folder**
cd sistem-monitoring-suhu

**Jalankan program**
cargo run

**Fitur**
- Input data sensor manual
- Simulasi otomatis 5 skenario
- Moving Average filter (window = 5)
- Alarm otomatis Warning & Danger
- Kalibrasi sensor
- Laporan lengkap multi-sesi
- Statistik: rata-rata, min, max, simpangan standar

**Bukti Screenshot**
<img width="963" height="287" alt="image" src="https://github.com/user-attachments/assets/26b67c4a-d18e-431f-b8d5-36aeb6a00ecf" />
