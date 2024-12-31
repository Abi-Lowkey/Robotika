// Struct klasik dengan nama properti dan tipe datanya
struct Student {
    name: String,
    level: u8,
    remote: bool
}

// Tuple struct hanya terdiri dari tipe data tanpa nama properti
struct Grades(char, char, char, f32);

// Fungsi utama aplikasi, diawali dengan keyword 'fn' untuk mendefinisikan fungsi
fn main() {

    // println! adalah macro bawaan Rust yang digunakan untuk mencetak teks ke konsol
    println!("Hello, {} {}!", "Will", "Velida");

    // Secara default, variabel di Rust bersifat immutable (tidak dapat diubah)
    // Tambahkan keyword 'mut' agar nilai variabel bisa diperbarui
    let mut age = 33;
    let birth_year = 1991;

    println!("I am {} years old", age);

    // Variabel dapat di-shadowing (menggantikan nilai variabel dengan nama yang sama)
    let birth_year = birth_year - 1;
    
    age = 34;

    println!("I am now {} years old", age);
    println!("I was born in {}", birth_year);

    // Rust adalah bahasa dengan sistem tipe data statis. Tipe data harus diketahui saat kompilasi
    let nephew_age: u32 = 14;
    println!("My nephew is {} years old", nephew_age);

    // Angka desimal (floating-point)
    let float: f32 = 4.0;
    
    println!("1 x 2 = {}", 1 * 2);

    // Boolean hanya dapat bernilai true atau false
    let is_bigger_num = 2 < 4;
    println!("Is 2 < 4: {}", is_bigger_num);

    // Karakter: tipe data untuk satu simbol
    let first_char: char = 'W';
    let last_char: char = 'l';

    let second_char = 'i';

    // String adalah koleksi dari karakter
    let my_name = "Will";

    println!(
        "{} is the first character, {} is the last character, {} is the second character of my name {}", 
        first_char, last_char, second_char, my_name
    );

    // Tuple: kombinasi beberapa nilai dengan tipe data berbeda. Panjang tuple tetap
    let my_dog = ("Toby", 15, false);

    println!(
        "My dog's name was {}, he was {} years old, is he alive? {}", 
        my_dog.0, my_dog.1, my_dog.2
    );
    
    // Membuat instance struct dengan memberikan nilai pada masing-masing properti
    let student_1 = Student {
        name: String::from("Will Velida"),
        remote: true,
        level: 5
    };

    let grades = Grades('A', 'A', 'B', 3.5);

    // Struct klasik: akses nilai properti menggunakan nama
    println!(
        "{}, is a level {} programmer. Does he work remotely: {}", 
        student_1.name, student_1.level, student_1.remote
    );

    // Tuple struct: akses nilai properti menggunakan indeks
    println!(
        "{},{},{},GPA = {}", 
        grades.0, grades.1, grades.2, grades.3
    );
}
