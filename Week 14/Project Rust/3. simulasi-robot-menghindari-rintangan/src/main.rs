use std::collections::VecDeque;

// Fungsi untuk memeriksa apakah langkah valid
fn is_valid_move(x: i32, y: i32, rows: usize, cols: usize, grid: &Vec<Vec<i32>>, visited: &Vec<Vec<bool>>) -> bool {
    // Mengecek apakah koordinat berada dalam batas grid, sel kosong (0), dan belum dikunjungi
    x >= 0 && y >= 0 && x < rows as i32 && y < cols as i32 && grid[x as usize][y as usize] == 0 && !visited[x as usize][y as usize]
}

fn find_path(grid: Vec<Vec<i32>>) -> Option<Vec<(i32, i32)>> {
    let rows = grid.len(); // Jumlah baris grid
    let cols = grid[0].len(); // Jumlah kolom grid
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)]; // Arah gerakan (kanan, bawah, kiri, atas)
    let mut visited = vec![vec![false; cols]; rows]; // Matriks untuk melacak sel yang sudah dikunjungi
    let mut queue: VecDeque<(i32, i32, Vec<(i32, i32)>)> = VecDeque::new(); // Antrian BFS dengan jalur saat ini

    // Memulai pencarian dari titik (0, 0)
    queue.push_back((0, 0, vec![(0, 0)]));
    visited[0][0] = true;

    while let Some((x, y, path)) = queue.pop_front() {
        // Jika mencapai tujuan (baris terakhir, kolom terakhir)
        if x == (rows as i32 - 1) && y == (cols as i32 - 1) {
            return Some(path);
        }

        // Mengeksplorasi semua arah gerakan
        for (dx, dy) in &directions {
            let nx = x + dx; // Koordinat baris berikutnya
            let ny = y + dy; // Koordinat kolom berikutnya

            if is_valid_move(nx, ny, rows, cols, &grid, &visited) {
                let mut new_path = path.clone();
                new_path.push((nx, ny)); // Menambahkan langkah ke jalur
                queue.push_back((nx, ny, new_path)); // Menyimpan langkah berikutnya ke antrian
                visited[nx as usize][ny as usize] = true; // Menandai sel sebagai sudah dikunjungi
            }
        }
    }

    // Tidak ada jalur ditemukan
    None
}

fn main() {
    let grid = vec![
        vec![0, 0, 0, 0, 0],
        vec![1, 1, 0, 1, 1],
        vec![0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 0],
        vec![0, 0, 0, 0, 0],
    ];

    // Menampilkan grid
    println!("Peta:");
    for row in &grid {
        println!("{:?}", row);
    }

    // Menjalankan fungsi pencarian jalur
    match find_path(grid) {
        Some(path) => {
            println!("Jalur ditemukan:");
            for (i, (x, y)) in path.iter().enumerate() {
                println!("Langkah {}: ({}, {})", i + 1, x, y);
            }
        }
        None => {
            println!("Tidak ada jalur yang ditemukan.");
        }
    }
}
