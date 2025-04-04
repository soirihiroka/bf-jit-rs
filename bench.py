import subprocess
import time
import json
import statistics
from datetime import datetime

def run_command_benchmark(command):
    timings = []

    for _ in range(100):
        start_time = time.time()
        try:
            # Run the command as a subprocess
            subprocess.run(command, shell=True, check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        except subprocess.CalledProcessError as e:
            print(f"Error while executing command: {e}")
        end_time = time.time()

        # Calculate the duration
        timings.append(end_time - start_time)

    # Calculate statistics
    timings.sort()
    min_time = min(timings)
    max_time = max(timings)
    mean_time = statistics.mean(timings)
    median_time = statistics.median(timings)
    p10_time = timings[int(len(timings) * 0.1)]
    p90_time = timings[int(len(timings) * 0.9)]
    # top_25_percent = timings[int(len(timings) * 0.75):]
    # bottom_25_percent = timings[:int(len(timings) * 0.25)]

    # Prepare JSON data
    results = {
        "command": command,
        "timings": timings,
        "statistics": {
            "min": min_time,
            "max": max_time,
            "mean": mean_time,
            "median": median_time,
            "10th_percentile": p10_time,
            "90th_percentile": p90_time,
            # "top_25_percent": top_25_percent,
            # "bottom_25_percent": bottom_25_percent
        }
    }

    # Generate a timestamped filename
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    filename = f"benchmark_results_{timestamp}.json"

    # Write to JSON file
    with open(filename, "w") as json_file:
        json.dump(results, json_file, indent=4)

    print(f"Benchmark results saved to {filename}")

if __name__ == "__main__":
    import sys
    if len(sys.argv) != 2:
        print("Usage: python script.py <command>")
    else:
        command = sys.argv[1]
        run_command_benchmark(command)
