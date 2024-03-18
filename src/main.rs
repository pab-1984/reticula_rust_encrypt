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
        // En una implementación real, aquí aplicarías el algoritmo de reducción de retícula.
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
