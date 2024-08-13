use num::Integer; // Import traits needed for modular arithmetic

fn main() {
    let n = 5;
    let m = 7;

    // Generate elements of Z_n x Z_m*
    let units_mod_m: Vec<i32> = (1..m).filter(|x| x.gcd(&m) == 1).collect();
    let elements: Vec<(i32, i32)> = (0..n)
        .flat_map(|a| units_mod_m.iter().map(move |&b| (a, b)))
        .collect();

    println!("Direct product of Z_{} and Z_{}^*:", n, m);
    for elem in &elements {
        println!("{:?}", elem);
    }

    // Compute product of two elements
    let (a, b) = elements[0];
    let (e, f) = elements[1];
    let product = ((a + e) % n, (b * f) % m);
    println!("Product of ({}, {}) and ({}, {}) mod ({}, {}): {:?}", a, b, e, f, n, m, product);

    // Find inverse of an element
    let (a, b) = elements[1];
    let inverse_b = units_mod_m.iter().find(|&&x| (b * x) % m == 1).unwrap();
    let inverse = ((-a).rem_euclid(n), *inverse_b);
    println!("Inverse of ({}, {}) in Z_{} x Z_{}^* is {:?}", a, b, n, m, inverse);
}


