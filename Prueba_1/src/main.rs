use mpi::topology::Communicator;
use mpi::collective::SystemOperation;
use mpi::collective::Root;
use mpi::traits::*;
use rand::Rng;
use std::time::Instant;

// =====================
// Estructura del Grafo
// =====================
struct Graph {
    edges: Vec<(usize, usize)>,
    #[allow(dead_code)]
    vertices: usize,
}

// =============================================
// Función: Partición del Grafo entre procesos
// =============================================
// Divide el grafo en subgrafos (por aristas) para distribuir la carga entre procesos.
fn parallel_partition(graph: &Graph, rank: usize, size: usize) -> Vec<(usize, usize)> {
    let chunk_size = (graph.edges.len() + size - 1) / size;
    let start = rank * chunk_size;
    let end = usize::min(start + chunk_size, graph.edges.len());
    graph.edges[start..end].to_vec()
}

// ===================================
// Función: Recolección de resultados 
// ===================================
// Se utiliza una operación colectiva para obtener los resultados locales de todos los procesos.
fn gather_results<T: mpi::topology::Communicator>(world: &T, local_results: Vec<f64>) -> Vec<f64> {
    let mut global_results = vec![0.0; local_results.len() * world.size() as usize];
    world.all_gather_into(&local_results, &mut global_results);
    global_results
}

// ==============================
// Función: Generación del Grafo
// ==============================
// Genera un grafo artificial con 'num_edges' aristas y 'num_vertices' vértices.
fn generate_graph(num_edges: usize, num_vertices: usize) -> Graph {
    let mut rng = rand::thread_rng();
    let edges = (0..num_edges)
        .map(|_| (rng.gen_range(0..num_vertices), rng.gen_range(0..num_vertices)))
        .collect();
    Graph {
        edges,
        vertices: num_vertices,
    }
}

// ===============
// Función: Main
// ===============
fn main() {
    // Inicializar el entorno MPI.
    let universe = mpi::initialize().expect("Error al inicializar MPI");
    let world = universe.world();
    let rank = world.rank() as usize;
    let size = world.size() as usize;

    let mut graph = if rank == 0 {
        println!("--------------------------------------------------");
        println!("Generando grafo con 200 mil vertice y aristas...");
        generate_graph(200_000, 200_000)
    } else {
        Graph {
            edges: vec![],
            vertices: 0,
        }
    };

    // Broadcast del número de vértices y del número total de aristas.
    let root_process = world.process_at_rank(0);
    root_process.broadcast_into(&mut graph.vertices);
    let mut num_edges = graph.edges.len();
    root_process.broadcast_into(&mut num_edges);

    // Ajustar el vector de aristas en procesos que no son rank 0.
    if rank != 0 {
        graph.edges.resize(num_edges, (0, 0));
    }

    // Broadcast de los datos del grafo: se envía el vector plano de aristas.
    let mut flat_edges: Vec<usize> = graph.edges.iter().flat_map(|&(u, v)| vec![u, v]).collect();
    root_process.broadcast_into(&mut flat_edges);

    // Restaurar la estructura del grafo en procesos no raíz.
    if rank != 0 {
        flat_edges.resize(num_edges * 2, 0); // Asegurar tamaño correcto antes del broadcast
        graph.edges = flat_edges.chunks(2).map(|chunk| (chunk[0], chunk[1])).collect();
    }

    // ========================
    // Medición: Cálculo Local
    // ========================
    let start_compute = Instant::now();
    let local_partition = parallel_partition(&graph, rank, size);
    // Simulación de procesamiento local: asigna 1.0 a cada arista en la partición.
    let local_results: Vec<f64> = local_partition.iter().map(|_| 1.0).collect();
    let compute_duration = start_compute.elapsed();

    // ==================================================
    // Medición: Comunicación Global con all_gather_into
    // ==================================================
    let start_communication = Instant::now();
    let global_results = gather_results(&world, local_results.clone());
    let communication_duration = start_communication.elapsed();

    // ========================================
    // Medición: Benchmark con all_reduce_into
    // ========================================    
    let start_bechmark= Instant::now();
    let mut global_results_bechmark = vec![0.0; local_results.len() * world.size() as usize];
    world.all_reduce_into(&local_results, &mut global_results_bechmark, SystemOperation::sum());
    let communication_duration_bechmark = start_bechmark.elapsed();

    // ====================================
    // Medición: Latencia y Ancho de Banda
    // ====================================
    // NOTA: La latencia se estima dividiendo el tiempo total de reducción por el número de procesos.
    let latency = communication_duration_bechmark.as_secs_f64() / (size as f64);
    // El ancho de banda se calcula en MB/s considerando el tamaño total de datos transmitidos.
    let bandwidth = (num_edges as f64 * std::mem::size_of::<usize>() as f64) / communication_duration_bechmark.as_secs_f64() / 1e6;

    // ========================
    // Impresión de Resultados
    // ========================
    // Cada proceso imprime sus resultados para facilitar la comparación individual.
    // Se muestra la cantidad de aristas procesadas, tiempos medidos, latencia y ancho de banda.
    println!("--------------------------------------------------");
    let num_results = global_results.len().min(local_partition.len());
    println!("Cantidad de Grafos: {})", num_results);
    // Se muestra el Grafo generado.
    //println!("Grafos: [{:?}])", &global_results[0..num_results]); // Quitar los comentario que esta al principio de esta linea de codigo
    println!("P{} - Resultados globales (Numero de aristas procesadas: {})", rank, local_partition.len());
    println!("Proceso {}:", rank);
    println!("P{} - Tiempo de computo local: {:?} s", rank, compute_duration.as_secs_f64());
    println!("P{} - Tiempo de comunicacion global (all_gather_into): {:?} s", rank, communication_duration.as_secs_f64());
    println!("P{} - Tiempo de comunicacion con benchmark (all_reduce_into): {:?} s", rank, communication_duration_bechmark.as_secs_f64());
    println!("P{} - Latencia estimada: {:.6} s", rank, latency);
    println!("P{} - Ancho de banda: {:.2} MB/s", rank, bandwidth);
    println!("--------------------------------------------------");
}
