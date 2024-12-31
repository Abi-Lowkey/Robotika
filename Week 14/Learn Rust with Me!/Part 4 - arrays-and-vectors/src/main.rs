fn main() {

    // Array: Sekumpulan nilai dengan tipe data yang sama, disusun secara berurutan di memori
    // Kita dapat membuat array dengan memberikan nilai awal, dan kompilator akan menghitung panjangnya
    let working_days = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];

    // Array juga bisa dibuat dengan mendeklarasikan panjangnya dan memberikan nilai default
    let working_days_num = [0; 5];

    // Elemen dalam array dapat diakses menggunakan indeks posisinya
    println!("{}", working_days[0]);

    // Vector: Koleksi nilai dengan tipe data yang sama, tetapi ukurannya bisa berubah
    // Vector dapat dibuat dengan nilai awal tertentu
    let nephews_age = vec![14, 9, 0];
    println!("Usia keponakan: {:?}", nephews_age);

    // Vector juga bisa dibuat dengan mendefinisikan panjangnya dan nilai default
    let zeroes = vec![0; 5];
    println!("Zeroes: {:?}", zeroes);

    // Elemen dapat ditambahkan atau dihapus dari vector dengan metode seperti push atau pop
    let mut names = Vec::new();

    names.push("Will");
    names.push("Isaac");
    names.push("Sam");

    println!("Nama: {:?}", names);

    names.pop();
    println!("Nama: {:?}", names);

    // Nilai dalam vector juga dapat diakses atau diubah berdasarkan indeks posisinya
    let mut fruit = vec!["Apple", "Melon", "Orange"];
    let orange = fruit[2];
    fruit[0] = "Strawberry";
    println!("Buah: {:?}, Orange = {}", fruit, orange);
    
}
