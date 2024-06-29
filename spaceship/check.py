import sys

def read_coordinates(file_path):
    coordinates = set()
    with open(file_path, 'r') as file:
        for line in file:
            if line.strip():
                x, y = map(int, line.split())
                coordinates.add((x, y))
    return coordinates

def check_subset(file1, file2):
    coords1 = read_coordinates(file1)
    coords2 = read_coordinates(file2)

    if coords1.issubset(coords2):
        return True, []
    else:
        missing_coords = coords1 - coords2
        return False, missing_coords

if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("Usage: python check_subset.py <file1> <file2>")
        sys.exit(1)

    file1 = sys.argv[1]  # File containing subset coordinates
    file2 = sys.argv[2]  # File containing all coordinates

    is_subset, missing_coords = check_subset(file1, file2)

    if is_subset:
        print("All coordinates in file1 are present in file2.")
    else:
        print("Not all coordinates in file1 are present in file2.")
        print("Missing coordinates:")
        for coord in missing_coords:
            print(coord)

