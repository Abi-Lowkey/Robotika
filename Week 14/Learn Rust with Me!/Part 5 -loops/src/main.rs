fn main() {
    /* 
    Loop tanpa akhir: akan terus berjalan tanpa berhenti
    Gunakan break untuk menghentikan eksekusinya
    */
    /*
    loop {
        println!("Halo!");
    }
    */

    /* 
    Loop dengan break: berhenti ketika kondisi terpenuhi
    Break dapat digunakan untuk menghentikan loop dan mengembalikan nilai
    */
    let mut counter = 1;
    let loop_stop = loop {
        counter *= 4;
        if counter > 100 {
            break counter;
        }
    };

    println!("Hentikan loop pada counter = {}", loop_stop);

    /*
    While loop: eksekusi berulang selama kondisi masih benar
    Akan berhenti begitu kondisi berubah menjadi salah
    */
    let mut num = 0;
    while num < 10 {
        println!("Halo di sana!");
        num = num + 1;
    }

    /*
    For loop: digunakan untuk mengulang item dalam kumpulan data
    Dalam contoh ini, kita menggunakan iterator untuk memproses array
    */
    let shopping_list = ["milk", "cheese", "bread", "apples"];

    // Menggunakan metode iter() untuk mengakses elemen satu per satu
    for item in shopping_list.iter() {
        println!("Item berikutnya dalam daftar belanja saya adalah {}", item);
    }

    /*
    Rentang angka: membuat iterator dari rentang nilai tertentu
    Rentang a..b dimulai dari a hingga (b-1)
    */
    for number in 0..10 {
        println!("Angka {}", number);
    }
}
