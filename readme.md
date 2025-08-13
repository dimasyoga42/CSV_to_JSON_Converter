# CSV to JSON Converter (Rust)

Program ini adalah CLI sederhana untuk mengonversi file CSV menjadi file JSON dengan pendekatan
**modular** dan **clean code** di Rust.

## 📂 Struktur Proyek

```
src/
├── main.rs        # Entry point, memanggil fungsi `run()`
├── csv_utils.rs   # Modul untuk membaca file CSV
└── json_utils.rs  # Modul untuk konversi CSV -> JSON dan simpan file JSON
Cargo.toml         # Konfigurasi dependencies
```

## ⚙️ Dependencies

- [csv](https://crates.io/crates/csv) → Parsing CSV
- [serde](https://crates.io/crates/serde) → Serialisasi & Deserialisasi data
- [serde_json](https://crates.io/crates/serde_json) → Konversi data ke JSON

Tambahkan di `Cargo.toml`:

```toml
[dependencies]
csv = "1.3.1"
serde = "1.0.219"
serde_json = "1.0.142"
```

## 🚀 Cara Menjalankan

1. Pastikan sudah menginstal [Rust](https://www.rust-lang.org/).
2. Buat project baru:
   ```bash
   cargo new csv_to_json
   cd csv_to_json
   ```
3. Salin file `main.rs`, `csv_utils.rs`, dan `json_utils.rs` sesuai struktur.
4. Jalankan perintah:
   ```bash
   cargo run data.csv output.json
   ```

## 📌 Contoh Output

### Jika berhasil:

```
✅ Konversi berhasil: output.json
```

### Jika file tidak ditemukan:

```
❌ Error: Tidak bisa membuka file 'data.csv': No such file or directory
```

## 📖 Penjelasan Modularitas

- **`main.rs`** → Hanya menangani argumen CLI & eksekusi `run()`.
- **`csv_utils.rs`** → Menyediakan fungsi `baca_csv()` yang membaca file CSV dan mengembalikan
  string.
- **`json_utils.rs`** → Menyediakan fungsi `konversi_ke_json()` dan `simpan_json()`.

## 📜 Lisensi

Proyek ini bebas digunakan untuk belajar Rust.
