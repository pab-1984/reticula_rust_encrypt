## Descripción Matemática del Esquema GGH

El esquema GGH hace uso de la complejidad computacional de problemas asociados con retículas en espacios de dimensión alta. Una **retícula** es un conjunto de puntos en el espacio euclidiano generados por combinaciones lineales enteras de un conjunto de vectores base. Estos vectores base forman una matriz, conocida como la **base de la retícula**. La seguridad del esquema GGH se basa en la dificultad de encontrar el punto más cercano en una retícula, conocido como el **problema del punto más cercano** (CVP, por sus siglas en inglés).

### Generación de la Clave Privada y Pública

- **Clave Privada**: La clave privada en GGH es una base de la retícula que es casi ortogonal, lo que facilita la solución del CVP para quien posee esta clave. Matemáticamente, si \(B\) es una base de la retícula \(L\), la clave privada es \(B_{priv} = [b_1, b_2, ..., b_n]\), donde los vectores \(b_i\) están cerca de ser ortogonales entre sí.

- **Clave Pública**: La clave pública es una versión perturbada (es decir, no ortogonal y difícil de reducir) de la clave privada. Esto se logra mediante operaciones lineales y la adición de pequeñas perturbaciones a los vectores de la base. Si \(B_{priv}\) es la base privada, la clave pública \(B_{pub}\) puede ser generada como \(B_{pub} = B_{priv}A + E\), donde \(A\) es una matriz de transformación y \(E\) es una matriz que introduce un pequeño error en la transformación.

### Encriptación y Desencriptación

- **Encriptación**: Un mensaje \(m\), representado como un vector, se encripta transformándolo con la clave pública y añadiendo un vector de error pequeño \(e\), resultando en el mensaje cifrado \(c = mB_{pub} + e\).

- **Desencriptación**: La desencriptación utiliza la clave privada para resolver el CVP y recuperar el mensaje original \(m\) a partir de \(c\). La clave privada facilita la aproximación al punto más cercano en la retícula correspondiente al mensaje encriptado, permitiendo la eliminación del error \(e\) y la recuperación de \(m\).

### Matemáticas Adicionales

- **Generación de la Clave Privada Fuerte**: La generación de una base casi ortogonal fuerte implica el uso de técnicas como el algoritmo de Gram-Schmidt para ortogonalizar una matriz generada aleatoriamente, asegurando que la clave privada sea fácil de usar para la desencriptación.

- **Manejo de Errores**: La adición de un vector de error pequeño durante la encriptación es crucial para la seguridad, ya que evita ataques directos basados en la reducción de la retícula.

### Referencias

1. Goldreich O, Goldwasser S, Halevi S. Public-key cryptosystems from lattice reduction problems. En: Proceedings of CRYPTO '97. 1997.
2. Micciancio D, Goldwasser S. Complexity of Lattice Problems: A Cryptographic Perspective. Kluwer Academic Publishers; 2002.
3. Nguyen PQ, Vallée B, editores. The LLL Algorithm: Survey and Applications. Springer; 2010.
