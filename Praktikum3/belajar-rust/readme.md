cargo run:
Perintah cargo run digunakan untuk membangun dan menjalankan proyek Rust secara langsung dengan satu perintah.

cargo build:
Perintah cargo build digunakan untuk membangun proyek Rust tanpa menjalankannya.
Ini berguna ketika Anda ingin memeriksa hasil kompilasi atau jika Anda ingin menjalankan aplikasi secara terpisah menggunakan perintah lain.

crates.io:
repositori resmi untuk pustaka-pustaka (crates) yang ditulis dalam bahasa pemrograman Rust yang menjadi sumber utama bagi pengembang Rust untuk menemukan, berbagi, dan menggunakan pustaka-pustaka yang telah dibuat oleh komunitas Rust.

library:
Membuat kumpulan kode yang dapat digunakan kembali oleh proyek-proyek lain.
Library biasanya terdiri dari beberapa fungsi, struktur data, atau fitur-fitur lain.
Menggunakan library dengan cara menambahkan dependensinya di Cargo.toml.
cargo new nama-proyek --lib

file utama:
Tempat di mana Anda menulis kode untuk mengatur alur program, memanggil fungsi-fungsi dari library, atau melakukan tugas-tugas lain yang diperlukan oleh aplikasi atau pustaka Anda.
Di dalam file utama, Anda biasanya akan menulis fungsi main() jika proyek Anda adalah aplikasi, atau menulis definisi pustaka dan fungsi-fungsi ekspos jika proyek Anda adalah pustaka.
Jika proyek Anda adalah aplikasi, Anda akan memiliki file main.rs sebagai file utama. Jika proyek Anda adalah pustaka (library), Anda akan memiliki file lib.rs sebagai file utama.