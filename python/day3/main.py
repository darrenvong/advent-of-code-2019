from typing import List, Dict, Tuple

DIRECTIONS = { "U": (0, 1), "D": (0, -1), "L": (-1, 0), "R": (1, 0) }

def get_wire_span(path: List[str]):
    wire_span = {}

    current_x = 0
    current_y = 0
    step_count = 0

    for move in path:
        direction = move[0]
        speed = int(move[1:])

        for _ in range(speed):
            x_delta, y_delta = DIRECTIONS[direction]
            current_x += x_delta
            current_y += y_delta
            step_count += 1
            wire_span[(current_x, current_y)] = step_count


    return wire_span

def get_intersections(wire1: Dict[Tuple, int], wire2: Dict[Tuple, int]):
    return wire1.keys() & wire2.keys()

def calculate_closest_intersection(wire1, wire2):
    intersections = get_intersections(wire1, wire2)
    return min(map(lambda coord: abs(coord[0]) + abs(coord[1]), intersections))

def calculate_closest_combined(wire1: Dict[Tuple, int], wire2: Dict[Tuple, int]):
    intersections = get_intersections(wire1, wire2)
    return min(map(lambda coord: wire1[coord] + wire2[coord], intersections))

if __name__ == "__main__":
    with open("input.txt") as input:
        path1, path2 = input.readlines()

    path1 = path1.strip().split(",")
    path2 = path2.strip().split(",")
    
    wire1_span = get_wire_span(path1)
    wire2_span = get_wire_span(path2)

    closest_intersection_dist = calculate_closest_intersection(wire1_span, wire2_span)
    closest_combined_dist = calculate_closest_combined(wire1_span, wire2_span)

    print("=" * 15, "Part 1", "=" * 15)
    print(f"The closest Manhattan distance to the central port: {closest_intersection_dist}")
    
    print()

    print("=" * 15, "Part 2", "=" * 15)
    print(f"The closest combined steps: {closest_combined_dist}")
