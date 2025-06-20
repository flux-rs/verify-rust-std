#!/usr/bin/env python3

from collections import Counter

def process_error_lines(filename):
    """
    Process log file to filter, sort, and count error lines.
    
    Args:
        filename (str): Path to the log file
        
    Returns:
        dict: Dictionary with error lines and their counts, sorted by line content
    """
    error_lines = []
    
    try:
        with open(filename, 'r', encoding='utf-8') as file:
            for line in file:
                line = line.strip()  # Remove leading/trailing whitespace
                if line.lower().startswith('error'):
                    error_lines.append(line)
        
        # Count occurrences of each distinct error line
        line_counts = Counter(error_lines)
        
        # Sort by line content (alphabetically)
        # sorted_lines = dict(sorted(line_counts.items()))
       
        # Sort by count in descending order, then by line content for ties
        sorted_lines = dict(sorted(line_counts.items(), key=lambda x: (-x[1], x[0])))

        return sorted_lines
    
    except FileNotFoundError:
        print(f"Error: File '{filename}' not found.")
        return None
    except IOError:
        print(f"Error: Could not read file '{filename}'.")
        return None

def main():
    filename = "log.txt"
    
    print(f"Processing error lines from: {filename}")
    print("=" * 50)
    
    error_counts = process_error_lines(filename)
    
    if error_counts is not None:
        if error_counts:
            print(f"Found {len(error_counts)} distinct error lines:\n")
            
            for line, count in error_counts.items():
                print(f"Count: {count:3d} | {line}")
            
            total_errors = sum(error_counts.values())
            print("=" * 50)
            print(f"Total error lines: {total_errors}")
        else:
            print("No lines starting with 'error' found in the file.")

if __name__ == "__main__":
    main()
