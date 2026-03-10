# Scripts to get data from input file

def input_graph(input_file):
    with open(input_file, 'r') as file:
        node_number, arc_number = map(int, file.readline().strip().split()[:2])
        node_map = extract_nodes(file, node_number)
        insert_arcs(file, node_map, arc_number)
    return node_map


def extract_nodes(file, number):
    node_map = {}
    for _ in range(number):
        node, _ = file.readline().strip().split()
        node_map[node] = []
    return node_map


def insert_arcs(file, node_map, number):
    for _ in range(number):
        arc = file.readline().strip().split()
        node_map[arc[0]].append(arc)


def solution(input_file):
    with open(input_file, 'r') as file:
        data = {
            'solution_value': solution_value(file),
            'fixed_units': fixed_units(file),
            'graph_routes': graph_routes(file),
            'zones_routes': zones_routes(file)
        }
    return data


def solution_value(file):
    return file.readline().strip().split(" ")[1]


def fixed_units(file):
    line = reach_line(file, 'F')
    units = line.strip().split(' ')[1:]
    return list(map(int, units))


def reach_line(file, line_piece):
    while True:
        previous_position = file.tell()
        line = file.readline()
        if line == '\n' or '---' in line:
            continue
        elif line_piece in line:
            return line
        else:
            file.seek(previous_position)
            return None


def graph_routes(file):
    graph_routes = []

    while line := reach_line(file, 'G'):
        route_data = extract_route_data(line)
        route = make_route_list(route_data)
        graph_routes.append(route)
        
    return graph_routes


def extract_route_number(line):
    return int(line[1:line.find(' ')])


def extract_route_data(line):
    return line[line.find('('):]


def make_route_list(route_line):
    arcs = route_line.strip()[1:-1].split('), (')
    if arcs[-1] == '':
        arcs.pop()
    route_list = []
    for arc in arcs:
        arc = arc.strip().split(', ')
        origin, destiny = map(int, arc)
        route_list.append((origin, destiny))
    return route_list


def zones_routes(file):
    zones = {}

    while line := reach_line(file, 'Z'):
        number = extract_route_number(line)
        
        if not zones.get(number):
            zones[number] = []

        route_data = extract_route_data(line)
        route = make_route_list(route_data)
        zones[number].append(route)
    return zones
