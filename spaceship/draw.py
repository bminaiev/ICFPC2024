import matplotlib.pyplot as plt
import glob

# Function to read coordinates from a file
def read_coordinates(file_path):
    coordinates = []
    with open(file_path, 'r') as file:
        for line in file:
            if line.strip():
                x, y = map(int, line.split())
                coordinates.append((x, y))
    return coordinates

# Function to plot the coordinates and save the plot as an image
def plot_and_save_coordinates(coordinates, output_file):
    x_values = [point[0] for point in coordinates]
    y_values = [point[1] for point in coordinates]

    plt.figure(figsize=(10, 6))
    plt.scatter(x_values, y_values, color='b')
    plt.xlabel('X Coordinate')
    plt.ylabel('Y Coordinate')
    plt.title('Plot of Coordinates')
    plt.grid(True)
    plt.savefig(output_file)
    plt.close()
"""
def plot_and_save_coordinates(coordinates, output_file):
    x_values = [point[0] for point in coordinates]
    y_values = [point[1] for point in coordinates]

    plt.figure(figsize=(10, 6))
    plt.plot(x_values, y_values, marker='o', linestyle='-', color='b')
    plt.xlabel('X Coordinate')
    plt.ylabel('Y Coordinate')
    plt.title('Plot of Coordinates')
    plt.grid(True)
    plt.savefig(output_file)
    plt.close()
"""
# Process all files matching the pattern "spaceshipX.in"
files = glob.glob("spaceship*.in")

for file_path in files:
    try:
        print(file_path)
        coordinates = read_coordinates(file_path)
        output_file = file_path.replace(".in", ".P.png")
        plot_and_save_coordinates(coordinates, output_file)
        print("ok")
    except:
        print("fail")

print("Plots have been saved for all files.")

