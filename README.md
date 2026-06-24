# RH Markup (.rhm) Engine

[![Version](https://img.shields.io/github/v/release/Rh-dt/RH-Markup?include_prereleases&color=blue&label=version)](https://github.com/Rh-dt/RH-Markup/releases)
[![Last Commit](https://img.shields.io/github/last-commit/Rh-dt/RH-Markup?color=brightgreen)](https://github.com/Rh-dt/RH-Markup/commits/home)
[![Repo Size](https://img.shields.io/github/repo-size/Rh-dt/RH-Markup?color=orange)](https://github.com/Rh-dt/RH-Markup)

RH Markup (.rhm) adalah bahasa markup berbasis teks generasi baru yang dirancang secara spesifik untuk kecepatan, efisiensi pengetikan, dan eksekusi komputasi langsung di terminal (Native CLI Rendering). 

Pada rilis versi 1.0, mesin ini telah terintegrasi dengan Computer Algebra System (CAS), utilitas pembacaan variabel sistem operasi (OS Bridge), dan telemetri dokumen waktu nyata. Mesin ini dibangun menggunakan Rust dengan arsitektur Zero-Allocation AST untuk menjamin konsumsi memori yang absolut minimum.

---

## Spesifikasi Teknis Utama

- Native CLI Rendering: File dieksekusi dan diformat langsung ke standar output terminal tanpa ketergantungan peramban web.
- Zero-Allocation Abstract Syntax Tree: Pemanfaatan referensi memori statis untuk memangkas duplikasi data pada RAM, menghasilkan kompleksitas memori yang efisien.
- Embedded Computer Algebra System: Evaluasi matematika tingkat lanjut (Trigonometri, Logaritma, Kalkulus Numerik) yang dijalankan secara mandiri.
- Sistem Telemetri Dinamis: Analitik resolusi tinggi untuk penghitungan baris, kata, karakter, dan waktu eksekusi kompilasi (milidetik).
- Strict Pure-Text Mode: Lingkungan terminal yang disanitasi dari karakter non-standar untuk kompatibilitas penuh dengan server headless.

---

## Panduan Instalasi

Terdapat beberapa metode instalasi yang didukung secara resmi:

### Instalasi Global via Cargo (Direkomendasikan)
Kompilasi dan instalasi otomatis melalui jaringan repositori Git:
```bash
cargo install --git https://github.com/Rh-dt/RH-Markup

```
### Kompilasi Manual dari Kode Sumber
```bash
git clone https://github.com/Rh-dt/RH-Markup.git
cd RH-Markup/rhm_engine
cargo build --release
sudo cp target/release/rhm_engine /usr/local/bin/rhm

```
## Referensi Sintaks (.rhm)
### 1. Struktur Dokumen (Block Elements)
Berikan satu spasi setelah penanda awal baris.
| Penanda | Fungsi Struktural | Contoh Penulisan |
|---|---|---|
| #  | Tajuk Tingkat 1 | # Laporan Analisis |
| ##  | Tajuk Tingkat 2 | ## Metodologi |
| ###  | Tajuk Tingkat 3 | ### Parameter Uji |
| !  | Peringatan Sistem | ! Data belum divalidasi |
| >  | Blok Kutipan | > Referensi arsitektur |
| -  | Daftar Tidak Berurutan | - Modul A |
| +  | Daftar Tugas | + Verifikasi subsistem |
| --- | Garis Separator | --- (Tanpa spasi) |
| ;  | Komentar Kode | ; Baris ini diabaikan oleh parser |

### 2. Format Tipografi (Inline Elements)
| Penanda Leksikal | Output Terminal |
|---|---|
| <code>*teks*</code> | Cetak tebal (Cyan) |
| <code>_teks_</code> | Cetak miring |
| <code>'teks'</code> | Sorotan latar belakang (Highlight) |
| <code>`teks`</code> | Monospace format kode |
| <code>~teks~</code> | Teks dicoret |

### 3. Format Nilai Tukar (Currency Shortcodes)
Gunakan simbol dolar diikuti kode angka regional dan satu spasi sebelum nominal.
 * $ 50000 menjadi IDR 50000
 * $1 150 menjadi USD 150
 * $2 200 menjadi EUR 200
 * Regional Asia: CNY ($6), JPY ($3), SGD ($9), MYR ($8)
 * Regional Eropa/Global: GBP ($4), RUB ($7), SAR ($5)

### 4. Eksekusi Komputasi dan Antarmuka Sistem
Gunakan awalan $=  pada awal baris untuk mengeksekusi operasi matematika, manipulasi teks, atau perintah sistem operasi.

**A. Matematika Dasar dan Logaritma**
 * $= 10 + 5 * 2 ^ 3 (Aritmatika presisi)
 * $= log(10, 1000) (Logaritma)
 * $= fact(5) (Faktorial)
 * $= 100 % 3 (Modulo)

**B. Trigonometri dan Akar**
 * $= sin(1.57) / cos(0) / tan(1)
 * $= sqrt(144) / cbrt(27)
 * $= abs(-50.5)

**C. Statistika dan Data**
 * $= max(10, 50) / min(10, 50)
 * $= mean(10, 20, 30, 40)
 * $= round(10.6) / floor(10.9) / ceil(10.1)

**D. Manipulasi String**
 * $= str:len(Hitung panjang kalimat ini)
 * $= str:upper(teks kapital)
 * $= str:lower(TEKS KECIL)
 * $= str:reverse(Dibalik)

**E. Antarmuka Sistem (OS Bridge)**
 * $= sys:os_info (Membaca jenis kernel OS)
 * $= sys:arch (Membaca arsitektur CPU)
 * $= env:USER (Membaca variabel environment)
 * $= sys: ls -la (Mengeksekusi perintah shell OS)

**F. Geometri dan Fisika**
 * $= geom:luas_lingkaran(7)
 * $= geom:volume_kubus(5)
 * $= geom:luas_segitiga(10, 5)
 * $= phys:kecepatan(100, 2)

© RH
