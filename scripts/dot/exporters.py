import extractors as extract

graph_colors = ["blue", "green"]


def full_response(input_file, solution_files):
    graph = extract.input_graph(input_file)
    output_file = input_file + ".dot"

    with open(output_file, 'w') as file:
        file.write("digraph {\n")
        file.write(f"label = \"{input_file}\"\n")
        export_input_graph(graph, file)
        export_solutions(solution_files, file)
        export_legend(solution_files, file)
        file.write("}\n")


def export_input_graph(graph, file):
    biggest = 0
    for _node, arcs in graph.items():
        for arc in arcs:
            crime_factor = float(arc[2])

            if crime_factor > biggest:
                biggest = crime_factor

    #set_arcs_color("gray80", file)
    file.write('edge[fontcolor = "gray30"]\n')
    for _node, arcs in graph.items():
        for arc in arcs:
            origin, destiny, crime_factor, distance = arc

            arc_color = build_arc_color(biggest, crime_factor)
            label = f'[label = "fc={crime_factor}\nd={distance}" color = "{arc_color}"]'
            line = f"{origin} -> {destiny} {label}\n"
            file.write(line)


def build_arc_color(biggest_factor, arc_factor):
    biggest_factor, arc_factor = float(biggest_factor), float(arc_factor)
    percent = arc_factor / biggest_factor
    minimum = 0.1
    if percent < minimum:
        percent = minimum

    intensity = hex(int(percent * 255))[2:]

    if len(intensity) < 2:
        intensity = f"0{intensity}"

    return f"#ff0000{intensity}"


def export_solutions(solution_files, output_file):
    for i in range(len(solution_files)):
        solution = extract.solution(solution_files[i])
        export_solution(solution, graph_colors[i], output_file)


def export_solution(solution, color, file):
    set_arcs_color(color, file)
    export_graph_routes(solution['graph_routes'], file)
    export_zones_routes(solution['zones_routes'], file)
    export_fixed_units(solution['fixed_units'], color, file)


def set_arcs_color(color, file):
    file.write(f"edge[color = {color}]\n")


def export_graph_routes(routes, file):
    map_graph_edges(file, routes, graph_writer)


def graph_writer(file, origin, destiny):
    file.write(f"{origin} -> {destiny}\n")


def map_graph_edges(file, routes, mapper):
    for route in routes:
        for (origin, destiny) in route:
            mapper(file, origin, destiny)


def export_zones_routes(routes, file):
    map_zones_edges(file, routes, zones_writer)


def zones_writer(file, origin, destiny, zone_number):
    file.write(f"{origin} -> {destiny}\n")


def map_zones_edges(file, routes, mapper):
    for zone_number, zone in routes.items():
        for route in zone:
            for (origin, destiny) in route:
                mapper(file, origin, destiny, zone_number)


def export_fixed_units(units, color, file):
    for unit in units:
        file.write(
            f"{unit} [shape = box style = filled fillcolor = {color}]\n")


def export_legend(solutions_file, file):
    file.write("subgraph cluster_01 {\n")
    file.write('label = "Legend"\n')
    set_arcs_color("black", file)
    for i in range(len(solutions_file)):
        solution = solutions_file[i]
        color = graph_colors[i]
        file.write(f'{color} [style = filled fillcolor = {color}]')
        file.write(f'{color} -> "{solution}"\n')
    file.write("}\n")
