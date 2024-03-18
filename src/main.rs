use num_bigint::{BigUint, RandBigInt};
use rand::{thread_rng, Rng};

pub struct GGH {
    private_key: Vec<Vec<BigUint>>, // Clave privada: base casi ortogonal
    public_key: Vec<Vec<BigUint>>,  // Clave pública: base perturbada
}

impl GGH {
    // Genera las claves pública y privada
    pub fn generate_keys(size: usize) -> Self {
        let private_key = generate_strong_private_key(size);
        let public_key = perturb_private_key(&private_key);

        Self {
            private_key,
            public_key,
        }
    }

    // Encripta el mensaje
    pub fn encrypt(&self, message: Vec<BigUint>) -> Vec<BigUint> {
        let mut rng = thread_rng();
        let error: Vec<BigUint> = (0..message.len())
            .map(|_| rng.gen_biguint(1)) // Generar un error pequeño
            .collect();

        self.public_key.iter()
            .map(|row| {
                row.iter().zip(&message).map(|(a, b)| a * b).sum::<BigUint>() + rng.gen_biguint(1) // Añadir error al sumar
            })
            .collect()
    }

    // Desencripta el mensaje
    pub fn decrypt(&self, encrypted_message: Vec<BigUint>) -> Vec<BigUint> {
        fn lll_reduction(basis: &DMatrix<f64>) -> DMatrix<f64> {
            let mut b = basis.clone();
            let dim = b.ncols();
            let mut k = 1;
        
            while k < dim {
                for j in (0..k).rev() {
                    let b_k_proj = project_onto(&b.column(k), &b.column(j));
                    if b_k_proj.norm_squared() > 0.5 * b.column(j).norm_squared() {
                        let mu = (b.column(k).dot(&b.column(j)) / b.column(j).dot(&b.column(j))).round();
                        let b_k = b.column(k) - b.column(j) * mu;
                        b.set_column(k, &b_k);
                    }
                }
        
                if b.column(k).norm_squared() >= (0.75 - b.column(k - 1).dot(&b.column(k)).powi(2) / b.column(k - 1).norm_squared()) * b.column(k - 1).norm_squared() {
                    k += 1;
                } else {
                    b.swap_columns(k, k - 1);
                    k = std::cmp::max(k - 1, 1);
                }
            }
        
            b
        }
        
        fn project_onto(u: &DVector<f64>, v: &DVector<f64>) -> DVector<f64> {
            let scalar_proj = u.dot(v) / v.norm_squared();
            v * scalar_proj
        }
        
        fn main() {
            let basis = DMatrix::from_row_slice(3, 3, &[
                1.0, 1.0, 1.0,
                -1.0, 0.0, 2.0,
                3.0, 5.0, 6.0,
            ]);
        
            let reduced_basis = lll_reduction(&basis);
            println!("Reduced basis:\n{}", reduced_basis);
        }
        encrypted_message // Esto es solo un marcador de posición.
    }
}

// Genera una clave privada fuerte y casi ortogonal
fn generate_strong_private_key(size: usize) -> Vec<Vec<BigUint>> {
    let mut rng = thread_rng();
    let mut private_key = vec![vec![BigUint::from(0u32); size]; size];

    for i in 0..size {
        for j in 0..size {
            if i == j {
                private_key[i][j] = BigUint::from(1u32) + rng.gen_biguint(2);
            } else {
                private_key[i][j] = rng.gen_biguint(1);
            }
        }
    }

    private_key
}

// Perturba la clave privada para generar la clave pública
fn perturb_private_key(private_key: &Vec<Vec<BigUint>>) -> Vec<Vec<BigUint>> {
    let mut rng = thread_rng();
    private_key.iter()
        .map(|row| {
            row.iter()
                .map(|&val| val + rng.gen_biguint(1))
                .collect()
        })
        .collect()
}

fn main() {
    let ggh = GGH::generate_keys(4); // Suponiendo que trabajamos con vectores de tamaño 4
    let message = vec![BigUint::from(1u32), BigUint::from(2u32), BigUint::from(3u32), BigUint::from(4u32)];

    let encrypted_message = ggh.encrypt(message.clone());
    println!("Mensaje encriptado: {:?}", encrypted_message);

    let decrypted_message = ggh.decrypt(encrypted_message);
    println!("Mensaje desencriptado (simulado): {:?}", decrypted_message);
    // Nota: El mensaje desencriptado es solo simulado y no refleja una desencriptación real en esta implementación simplificada.
}
