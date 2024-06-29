import sys
import matplotlib.pyplot as plt

def read_points(file_path):
    points = []
    with open(file_path, 'r') as file:
        for line in file:
            if line.strip():
                x, y = map(int, line.split())
                points.append((x, y))
    return points

def read_path(file_path):
    with open(file_path, 'r') as file:
        path = file.read().strip()
    return path

def compute_path(path_sequence):
    moves = {
        '1': (-1, -1),
        '2': (0, -1),
        '3': (1, -1),
        '4': (-1, 0),
        '5': (0, 0),  # No movement, usually this is not used in paths
        '6': (1, 0),
        '7': (-1, 1),
        '8': (0, 1),
        '9': (1, 1)
    }

    vx, vy = 0, 0
    x, y = 0, 0
    path = [(x, y)]

    for move in path_sequence:
        if move in moves:
            dx, dy = moves[move]
            vx += dx
            vy += dy
            x += vx
            y += vy
            path.append((x, y))

    return path

def plot_points_and_path(points, path, output_file):
    fig, ax = plt.subplots(figsize=(12, 12))
    points_x, points_y = zip(*points)
    path_x, path_y = zip(*path)

    ax.plot(points_x, points_y, 'ro', label='Points to Visit', markersize=10)
    ax.plot(path_x, path_y, 'b-', label='Spaceship Path', linewidth=1)
    ax.plot(path_x, path_y, 'bo', markersize=4)

    ax.set_xlabel('X')
    ax.set_ylabel('Y')
    ax.legend()
    ax.grid(True)
    ax.set_aspect('equal', 'box')

    plt.savefig(output_file, dpi=300)
    plt.close()

def main():
    if len(sys.argv) != 3:
        print("Usage: python script.py <points_file> <path_file>")
        sys.exit(1)

    points_file = sys.argv[1]
    path_file = sys.argv[2]
    output_file = path_file + ".png"

    points = read_points(points_file)
    path_sequence = read_path(path_file)
    path = compute_path(path_sequence)

    plot_points_and_path(points, path, output_file)

if __name__ == "__main__":
    main()

