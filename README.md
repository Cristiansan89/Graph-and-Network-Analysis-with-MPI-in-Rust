# Análisis de Grafos y Redes con MPI

<h2>*Proyecto para la materia "Paradigmas y Lenguajes de Programación"*</h2>

## Implementación en Rust

El código fuente en Rust permite generar grafos de distintos tamaños. Las configuraciones se encuentran separadas de la siguiente manera:
<br>
 *Prueba_1: Genera un grafo de 250.000 vértices y aristas.<br>
 *Prueba_2: Genera un grafo de 500.000 vértices y aristas.<br>
 *Prueba_3: Genera un grafo de 1 millón de vértices y aristas.<br>

### Instrucciones de Ejecución (Rust)

1. Abra una terminal de línea de comandos (Se recomienda en Windows `CMD` o `Windows PowerShell` y en Linux `Terminal`).
2. (Opcional) Si la implementación de MPI lo requiere, ejecute la terminal en `modo administrador`.
3. Para obtener mediciones de rendimiento precisas, se recomienda cerrar todas las aplicaciones que no sean esenciales para evitar congestión y consumo de recursos durante el cálculo.
4. Ejecute el siguiente comando para compilar y correr el programa con 4 procesos:

```bash* mpiexec -n 4 cargo run --release *```

## Implementación en C

El archivo grafo_mpi.c contiene el código fuente para generar un grafo de 1 millón de vértices y aristas.

### Instrucciones de Ejecución (C)

1. Abra una terminal de `Windows PowerShell`.
2. (Opcional) Si la implementación de MPI lo requiere, ejecute la terminal en `modo administrador`.
3. Para obtener mediciones de rendimiento precisas, se recomienda cerrar todas las aplicaciones que no sean esenciales. Esto ayuda a evitar interferencias en los tiempos de cómputo y comunicación.
4. Compilar el código con el siguiente comando:

```bash** mpicc -o grafo_mpi.exe grafo_mpi.c **```
   
6. Ejecute el siguiente comando para lanzar el programa precompilado (grafo_mpi.exe) con 4 procesos:

```bash** mpiexec -n 4 grafo_mpi.exe **```
