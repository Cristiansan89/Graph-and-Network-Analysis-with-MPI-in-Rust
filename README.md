# Paradigmas y Lenguajes de Programación

# Análisis de Grafos y Redes con MPI en Rust

## MPI en el Lenguaje Rust

### Prueba_1

- En prueba 1 se encuentra el código rust que permite generar 250.000 vértices y aristas.

### Prueba_2

- En prueba 1 se encuentra el código rust que permite generar 500.000 vértices y aristas.

### Prueba_3

- En prueba 1 se encuentra el código rust que permite generar 1 millón vértices y aristas.

### Ejecución de Rust

- Lo ideal para ejecutar el código de Rust es hacerlo en una interfaz de línea de comandos, en windows se puede ejecutar en el `Símbolo del Sistema` o también conocido como `CMD`.
- El CMD debe abrirse en `modo administrador`.
- Recuerde que también debe cerrar todas aplicaciones.
- El comando para ejecutar el código de Rust es: `mpiexec -n 4 cargo run --release`.

## MPI en el Lenguaje C

- En el archivo C se encuentra el código de lenguaje C.
- Este código permite generar grafo con 1 millón de vértices y aristas.

### Ejecución de C

- Para ejecutar el código, debe realizarlo en el interpretes de líneas de comandos de Windows ==> `Windows PowerShell`.
- El Windows PowerShell debe abrirse en `modo administrador`.
- Recuerde cerrar todas aplicaciones que no consuma memoria y ancho de banda, esto es para evitar congestión y consumo de recursos en el cálculo de los tiempo de computo y comunicaciones.
- El comando para ejecutar el código de C es: `PS \C> mpiexec -n 4 grafo_mpi.exe`
