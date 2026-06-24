# 🚀 RH Markup (.rhm) Engine

[![Version](https://img.shields.io/github/v/release/Rh-dt/RH-Markup?include_prereleases&color=blue&label=version)](https://github.com/Rh-dt/RH-Markup/releases)
[![Last Commit](https://img.shields.io/github/last-commit/Rh-dt/RH-Markup?color=brightgreen)](https://github.com/Rh-dt/RH-Markup/commits/home)
[![Repo Size](https://img.shields.io/github/repo-size/Rh-dt/RH-Markup?color=orange)](https://github.com/Rh-dt/RH-Markup)

Selamat datang di **RH Markup (.rhm)**! Ini adalah bahasa *markup* berbasis teks generasi baru yang dirancang khusus untuk kecepatan, efisiensi pengetikan, dan eksekusi langsung di terminal secara mandiri (*Native CLI Rendering*). 

Ditulis murni menggunakan **Rust** tanpa dependensi eksternal (*Zero-Dependency*), *engine* `.rhm` mampu memproses dan merender dokumen teks menjadi visual terminal yang indah dalam hitungan milidetik.

---

## ✨ Fitur Unggulan

- **Native CLI Rendering:** Tidak butuh HTML atau Browser. File `.rhm` dieksekusi dan diwarnai langsung di terminal Anda!
- **Auto-Currency Formatting:** Fitur ajaib yang otomatis mengonversi *shortcode* menjadi format mata uang presisi (contoh: `$1 50000` menjadi `$50000` dengan warna hijau tebal).
- **Zero-Panic Architecture:** Dirancang dengan alokasi memori $O(1)$ untuk tag yang tak tertutup, memastikan *engine* tidak akan pernah *crash* seaneh apa pun Anda mengetik.

---

## 🛠️ Instalasi & Setup

RH Markup dapat diinstal di Linux, MacOS, Windows (via WSL), maupun GitHub Codespaces. Pilih salah satu metode instalasi di bawah ini:

### Instalasi via Cargo Git (Direkomendasikan)
Jika Anda sudah memiliki Rust dan Cargo terinstal di sistem, Anda bisa langsung menginstal *engine* ini secara global menggunakan repositori ini sebagai *Source Package*:
```bash
cargo install --git [https://github.com/Rh-dt/RH-Markup](https://github.com/Rh-dt/RH-Markup)

```
### Unduh Binary Lengkap (Bagi User Non-Rust)
Masuk ke tab **Releases**, unduh *file binary* rhm_engine terbaru, lalu jalankan perintah berikut di terminal:
```bash
sudo cp rhm_engine /usr/local/bin/rhm
chmod +x /usr/local/bin/rhm

```
### Build Manual dari Source Code
```bash
git clone [https://github.com/Rh-dt/RH-Markup.git](https://github.com/Rh-dt/RH-Markup.git)
cd RH-Markup/rhm_engine
cargo build --release
sudo cp target/release/rhm_engine /usr/local/bin/rhm

```
## 📖 Cara Menulis .rhm
Buat sebuah file dengan ekstensi .rhm (contoh: catatan.rhm), lalu gunakan sintaks di bawah ini. Untuk melihat hasilnya di layar terminal, cukup jalankan perintah:
```bash
rhm catatan.rhm

```
### 1. Struktur Dokumen (Block Elements)
Pastikan Anda memberikan **satu spasi** setelah simbol sebelum mulai mengetik teks.
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
### 2. Format Teks (Inline Elements)
Gunakan simbol ini untuk mengapit kata atau kalimat di mana saja.
| Sintaks | Fungsi & Output di Terminal |
|---|---|
| *teks* | Cetak tebal (**Bold** warna biru cyan) |
| _teks_ | Cetak miring (*Italic*) |
| 'teks' | Latar belakang kuning (*Highlight*) |
| `teks` | Teks gaya kode (*Monospace* warna magenta) |
| ~teks~ | Teks dicoret (~~Strikethrough~~) |
| ^teks^ | Teks pangkat/eksponensial (warna biru) |
### 3. Keajaiban Kurs Mata Uang (Shortcodes)
Ketik lambang dolar ($), diikuti kode opsional, **WAJIB SPASI SATU KALI**, lalu ketik angkanya. Spasi akan otomatis dihilangkan saat dirender dan teks akan dicetak tebal berwarna hijau!
 * **Otomatis Rupiah:** $ 50000 ➡️ **Rp50000**
 * **USD:** $1 150.50 ➡️ **$150.50**
 * **Euro:** $2 200 ➡️ **€200**
 * **Yen:** $3 5000 ➡️ **¥5000**
 * **Pound:** $4 80 ➡️ **£80**
 * **Riyal:** $5 120 ➡️ **﷼120**
*(Catatan: Mendukung titik dan koma untuk format desimal!)*
## 💻 Contoh Dokumen Penuh
**Tulis ini di demo.rhm:**
```text
# Rapat Tim Pengembangan v0.2
---
! Target rilis bulan depan tidak boleh tertunda.

Agenda hari ini:
- Evaluasi bug
- Pembahasan alokasi memori

Anggaran operasional tim:
Beli server AWS: $1 450
Makan siang tim: $ 350000

> Tetap semangat, mari kita jadikan _project_ ini luar biasa!

```
**Jalankan di terminal:**
```bash
rhm demo.rhm

```
*Diciptakan dengan penuh dedikasi oleh RH.*
