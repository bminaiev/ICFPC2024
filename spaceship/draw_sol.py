import matplotlib.pyplot as plt
import sys
import os


def read_input(filename):
    with open(filename, 'r') as file:
        data = file.read().strip().split()
    return list(map(int, data))


def main(test_num):
    input_filename = f'spaceship{test_num}.viz'
    output_filename = f'spaceship{test_num}.viz.png'

    if not os.path.exists(input_filename):
        print(f"Input file {input_filename} does not exist")
        return

    data = read_input(input_filename)

    # Read number of n points
    index = 0
    n = data[index]
    index += 1

    red_points = []
    for _ in range(n):
        x = data[index]
        y = data[index + 1]
        red_points.append((x, y))
        index += 2

    # Read number of m points
    m = data[index]
    index += 1

    blue_points = []
    for _ in range(m):
        x = data[index]
        y = data[index + 1]
        blue_points.append((x, y))
        index += 2

    # Plotting
    fig, ax = plt.subplots(figsize=(10, 8),
                           dpi=200)  # Increase size and resolution

    if blue_points:
        blue_xs, blue_ys = zip(*blue_points)
        ax.plot(blue_xs, blue_ys, color='blue', marker='o', linestyle='-')

    if red_points:
        red_xs, red_ys = zip(*red_points)
        ax.scatter(red_xs, red_ys, color='red', s=10,
                   zorder=5)  # Smaller dots with size 10

    ax.set_xlabel('X')
    ax.set_ylabel('Y')
    ax.set_title(f'Test #{test_num}; points to visit = {n}; sol len = {m - 1}'
                 )  # Dynamic title with details

    plt.savefig(output_filename, bbox_inches='tight')
    plt.show()


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python script.py test_num")
    else:
        test_num = sys.argv[1].zfill(2)
        main(test_num)
