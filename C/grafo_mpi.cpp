#include <mpi.h>
#include <vector>
#include <utility>
#include <cstdlib>
#include <ctime>
#include <iostream>
#include <chrono>
#include <numeric>

using namespace std;
using namespace std::chrono;

// =====================
// Estructura del Grafo
// =====================
struct Graph {
    vector<pair<size_t, size_t>> edges;
    size_t vertices;
};

// ==============================
// Generación del Grafo
// ==============================
Graph generate_graph(size_t num_edges, size_t num_vertices) {
    Graph graph;
    graph.vertices = num_vertices;
    srand(time(nullptr));
    for (size_t i = 0; i < num_edges; ++i) {
        graph.edges.emplace_back(rand() % num_vertices, rand() % num_vertices);
    }
    return graph;
}

// =============================================
// Partición del Grafo entre procesos
// =============================================
vector<pair<size_t, size_t>> parallel_partition(const Graph& graph, int rank, int size) {
    size_t chunk_size = (graph.edges.size() + size - 1) / size;
    size_t start = rank * chunk_size;
    size_t end = min(start + chunk_size, graph.edges.size());
    return vector<pair<size_t, size_t>>(graph.edges.begin() + start, graph.edges.begin() + end);
}

// ===================================
// Recolección de resultados 
// ===================================
void gather_results(MPI_Comm comm, const vector<double>& local_results, vector<double>& global_results) {
    int size;
    MPI_Comm_size(comm, &size);
    global_results.resize(local_results.size() * size);
    MPI_Allgather(local_results.data(), local_results.size(), MPI_DOUBLE,
                  global_results.data(), local_results.size(), MPI_DOUBLE, comm);
}

// ===============
// Función: Main
// ===============
int main(int argc, char* argv[]) {
    // Inicializar el entorno MPI.
    MPI_Init(&argc, &argv);

    int rank, size;
    MPI_Comm comm = MPI_COMM_WORLD;
    MPI_Comm_rank(comm, &rank);
    MPI_Comm_size(comm, &size);

    Graph graph;
    size_t num_edges = 0;

    if (rank == 0) {
        cout << "--------------------------------------------------\n";
        cout << "Generando grafo con 1 millon vertice y aristas...\n";
        graph = generate_graph(1'000'000, 1'000'000);
        num_edges = graph.edges.size();
    }

    // Broadcast del número de vértices y del número total de aristas.
    MPI_Bcast(&graph.vertices, 1, MPI_UNSIGNED_LONG, 0, comm);
    MPI_Bcast(&num_edges, 1, MPI_UNSIGNED_LONG, 0, comm);

    // Ajustar el vector de aristas en procesos que no son rank 0.
    if (rank != 0) {
        graph.edges.resize(num_edges);
    }

    // Broadcast de los datos del grafo: se envía el vector plano de aristas.
    vector<size_t> flat_edges(num_edges * 2);
    if (rank == 0) {
        for (size_t i = 0; i < num_edges; ++i) {
            flat_edges[2 * i] = graph.edges[i].first;
            flat_edges[2 * i + 1] = graph.edges[i].second;
        }
    }

    MPI_Bcast(flat_edges.data(), num_edges * 2, MPI_UNSIGNED_LONG, 0, comm);

    if (rank != 0) {
        graph.edges.clear();
        for (size_t i = 0; i < num_edges; ++i) {
            graph.edges.emplace_back(flat_edges[2 * i], flat_edges[2 * i + 1]);
        }
    }

    // ========================
    // Medición: Cálculo Local
    // ========================
    auto start_compute = high_resolution_clock::now();
    vector<pair<size_t, size_t>> local_partition = parallel_partition(graph, rank, size);
    vector<double> local_results(local_partition.size(), 1.0);
    auto end_compute = high_resolution_clock::now();

    // ==================================================
    // Medición: Comunicación Global con all_gather_into
    // ==================================================
    auto start_communication = high_resolution_clock::now();
    vector<double> global_results;
    gather_results(comm, local_results, global_results);
    auto end_communication = high_resolution_clock::now();

    // ========================================
    // Medición: Benchmark con all_reduce_into
    // ========================================  
    auto start_benchmark = high_resolution_clock::now();
    vector<double> global_results_benchmark(local_results.size() * size, 0.0);
    MPI_Allreduce(local_results.data(), global_results_benchmark.data(),
                  local_results.size(), MPI_DOUBLE, MPI_SUM, comm);
    auto end_benchmark = high_resolution_clock::now();

    double compute_time = duration<double>(end_compute - start_compute).count();
    double comm_time = duration<double>(end_communication - start_communication).count();
    double benchmark_time = duration<double>(end_benchmark - start_benchmark).count();

    // ====================================
    // Medición: Latencia y Ancho de Banda
    // ====================================
    double latency = benchmark_time / size;
    double bandwidth = (num_edges * sizeof(size_t)) / (benchmark_time * 1e6);

    cout << "--------------------------------------------------\n";
 
    if (!global_results.empty()) {
        cout << "Grafos: [" << global_results[0];
        for (size_t i = 1; i < min(global_results.size(), local_partition.size()); ++i) {
            cout << ", " << global_results[i];
        }
        cout << "]\n";
    }
    cout << "P" << rank << " - Resultados globales (Numero de aristas procesadas: " << local_partition.size() << ")\n";
    cout << "P" << rank << " - Tiempo de computo local: " << compute_time << "s\n";
    cout << "P" << rank << " - Tiempo de comunicacion global (Allgather): " << comm_time << "s\n";
    cout << "P" << rank << " - Tiempo de comunicacion con benchmark (Allreduce): " << benchmark_time << "s\n";
    cout << "P" << rank << " - Latencia estimada: " << latency << "s\n";
    cout << "P" << rank << " - Ancho de banda: " << bandwidth << " MB/s\n";
    cout << "--------------------------------------------------\n";

    MPI_Finalize();
    return 0;
}
