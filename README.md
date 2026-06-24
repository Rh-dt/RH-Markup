# 🚀 RH Markup (.rhm) Engine

[![Version](https://img.shields.io/github/v/release/Rh-dt/RH-Markup?include_prereleases&color=blue&label=version)](https://github.com/Rh-dt/RH-Markup/releases)
[![Last Commit](https://img.shields.io/github/last-commit/Rh-dt/RH-Markup?color=brightgreen)](https://github.com/Rh-dt/RH-Markup/commits/home)
[![Repo Size](https://img.shields.io/github/repo-size/Rh-dt/RH-Markup?color=orange)](https://github.com/Rh-dt/RH-Markup)

Selamat datang di **RH Markup (.rhm)** Ini adalah bahasa *markup* berbasis teks generasi baru yang dirancang khusus untuk kecepatan, efisiensi pengetikan, dan eksekusi langsung di terminal secara mandiri (*Native CLI Rendering*). 

Kini di versi BETA, `.rhm` tidak hanya sekadar merender visual, tetapi juga bertindak sebagai **Computer Algebra System (CAS)** ringan yang mampu mengevaluasi matematika rumit secara *real-time* langsung di dalam dokumen!

---

## ✨ Fitur Unggulan

- **Native CLI Rendering:** Tidak butuh HTML atau Browser. File `.rhm` dieksekusi dan diwarnai langsung di terminal Anda!
- **Zero-Panic Architecture:** Memori dialokasikan secara sadar $O(1)$ untuk memastikan program anti-*crash* dari kesalahan pengetikan.
- **Auto-Currency Formatting:** Injeksi format mata uang pintar (contoh: `$1 50000` menjadi `$50000` dengan warna hijau tebal).
- **Embedded Math Evaluator:** Hitung Logaritma, Eksponensial, hingga Geometri tanpa perlu berpindah ke kalkulator atau aplikasi lain.

---

## 🛠️ Instalasi & Setup

Pilih salah satu metode instalasi di bawah ini:

### Instalasi Cepat via Cargo Git (Rekomendasi)
Jika Anda memiliki Rust, Anda dapat langsung menginstal *engine* ini secara global:
```bash
cargo install --git https://github.com/Rh-dt/RH-Markup
```

### Build Manual dari Source Code
```bash
git clone https://github.com/Rh-dt/RH-Markup.git
cd RH-Markup/rhm_engine
cargo build --release
sudo cp target/release/rhm_engine /usr/local/bin/rhm
```
## 📖 Cara Menulis .rhm
Buat file berekstensi .rhm (contoh: catatan.rhm), lalu jalankan dengan perintah: rhm catatan.rhm

### 1. Struktur Dokumen
Berikan **satu spasi** setelah simbol di awal baris.
| Simbol | Fungsi | Contoh Penulisan |
|---|---|---|
| #  | Header 1 (Judul Utama) | # Laporan Keuangan |
| ##  | Header 2 (Sub Judul) | ## Pemasukan Bulan Ini |
| ###  | Header 3 (Sub-sub Judul) | ### Detail Tambahan |
| !  | Alert / Peringatan | ! Segera lakukan backup data! |
| >  | Kutipan / Quotes | > "Code is poetry." |
| -  | Bullet List | - Kopi hitam |
| +  | Checklist (Tugas) | + Selesaikan fitur parser |
| --- | Garis Pembatas (Rule) | --- (Tanpa spasi) |
| ;  | Komentar Tersembunyi | ; Catatan ini tidak akan dirender |

### 2. Format Teks
Apit kata/kalimat di mana saja menggunakan simbol berikut.
| Sintaks | Fungsi & Output di Terminal |
|---|---|
| <code>*teks*</code> | Cetak tebal (**Bold** warna biru cyan) |
| <code>_teks_</code> | Cetak miring (*Italic*) |
| <code>'teks'</code> | Latar belakang kuning (*Highlight*) |
| <code>`teks`</code> | Teks gaya kode (*Monospace* warna magenta) |
| <code>~teks~</code> | Teks dicoret (~~Strikethrough~~) |

### 3. Keajaiban Kurs Mata Uang (Shortcut)
Ketik dolar ($) + kode opsional + **SPASI SATU KALI** + angka. Spasi otomatis hilang dan menjadi teks hijau tebal!
 * $ 50000 ➡️ **Rp50000**
 * $1 150 ➡️ **$150** (USD)
 * $2 200 ➡️ **€200** (Euro)
 * $3 5000 ➡️ **¥5000** (Yen)
 * $4 80 ➡️ **£80** (Pound)
 * $5 120 ➡️ **﷼120** (Riyal)

### 4. Matematika & Logika (BETA)
Tidak perlu buka kalkulator! Gunakan sintaks $=  di awal baris untuk mengevaluasi rumus secara otomatis. Mesin akan merendernya dalam format khusus berwarna kuning di terminal.
| Modul Komputasi | Sintaks Dokumen .rhm | Output Evaluasi Terminal |
|---|---|---|
| **Eksponensial (Pangkat)** | $= 2 ^ 10 | 2 ^ 10 = 1024.0000 |
| **Logaritma** (basis, nilai) | $= log(10, 1000) | log_base_10(1000) = 3.0000 |
| **Geometri: Luas Lingkaran** | $= geom: luas_lingkaran(7) | Luas Lingkaran (r=7): 153.94 |
| **Geometri: Volume Kubus** | $= geom: volume_kubus(5) | Volume Kubus (s=5): 125.00 |
| **Logika Cryptarithm** | $= crypt: SEND+MORE=MONEY | S=9, E=5, N=6... -> 9567+1085=10652 |
| **Hampiran Limit** | $= limit: x->0 | *Output aproksimasi numerik* |

## 💻 Contoh Penggunaan Fitur Logika
Tulis ini di tugas.rhm:
```text
# Laporan Pengerjaan PR Matematika
---
! Pastikan semua rumus sudah dievaluasi oleh RHM Engine.

## Hitungan Volume Wadah
Berapa volume air yang bisa ditampung kubus dengan sisi 15 cm?
$= geom: volume_kubus(15)

## Hitungan Logaritma Kompleks
$= log(2, 256)

Anggaran untuk membeli alat tulis hari ini:
Beli penggaris baru: $ 15000
Beli kalkulator: $1 12.50

```
Jalankan: rhm tugas.rhm
*Diciptakan dengan penuh dedikasi oleh RH.*
