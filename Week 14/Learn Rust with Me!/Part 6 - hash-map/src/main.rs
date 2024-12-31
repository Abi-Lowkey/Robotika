fn main() {
    
    // Keyword 'use' digunakan untuk mengimpor HashMap dari modul 'collections' dalam pustaka standar Rust.
    use std::collections::HashMap;
    
    // Membuat sebuah hash map kosong menggunakan metode HashMap::new().
    let mut items: HashMap<String, String> = HashMap::new();

    // Menambahkan elemen ke dalam hash map dengan memanggil metode insert(<key>, <value>).
    items.insert(String::from("One"), String::from("Book"));
    items.insert(String::from("Two"), String::from("Keyboard"));
    items.insert(String::from("Three"), String::from("Sunglasses"));

    // Mengambil nilai dari hash map menggunakan kunci dengan metode get(<key>).
    let keyboard = items.get("Two");
    println!("{:?}", keyboard);

    // Menghapus elemen dari hash map dengan metode .remove(<key>).
    // Jika mencoba mengakses elemen yang sudah dihapus, maka akan mengembalikan nilai 'None'.
    items.remove("Three");
    
    println!("{:?}", items.get("Three"));
}
