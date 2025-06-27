#!/usr/bin/env python3

import re
from collections import defaultdict

def parse_log_file(filename):
    """
    Parse the log file and extract error information.

    Returns:
        tuple: (module_type_counts, total_errors)
    """
    with open(filename, 'r') as f:
        content = f.read()

    # Split into lines
    lines = content.split('\n')

    # Find all error blocks
    error_blocks = []
    current_block = []

    for line in lines:
        if line.startswith('error'):
            # If we have a current block, save it
            if current_block:
                error_blocks.append(current_block)
            # Start new block
            current_block = [line]
        elif current_block:  # We're inside an error block
            current_block.append(line)

    # Don't forget the last block
    if current_block:
        error_blocks.append(current_block)

    # Process each error block to extract module and error type
    module_type_counts = defaultdict(lambda: defaultdict(int))

    for block in error_blocks:
        if len(block) < 2:
            continue

        # Extract error type from first line
        error_type = block[0].strip()

        # Find the path line (should be second line)
        path_line = block[1].strip()

        # Look for the pattern "--> path"
        match = re.search(r'-->\s+([^:]+)', path_line)
        if match:
            full_path = match.group(1)

            # Extract module: substring after first occurrence up to third '/'
            parts = full_path.split('/')
            if len(parts) >= 3:
                # Take the first 3 parts to get the module
                module = '/'.join(parts[:3])
            else:
                # Fallback for shorter paths
                module = full_path

            module_type_counts[module][error_type] += 1

    return module_type_counts, len(error_blocks)

def main():
    filename = 'log.txt'

    try:
        module_type_counts, total_errors = parse_log_file(filename)

        print("Errors grouped by MODULE and TYPE:")
        print("=" * 50)

        # Sort modules by total error count in decreasing order
        modules_with_totals = []
        for module in module_type_counts.keys():
            type_counts = module_type_counts[module]
            module_total = sum(type_counts.values())
            modules_with_totals.append((module, module_total, type_counts))

        # Sort by total errors (ascending - increasing order)
        modules_with_totals.sort(key=lambda x: x[1], reverse=False)

        for module, module_total, type_counts in modules_with_totals:
            print(f"\n{module} ({module_total} errors):")

            # Sort error types and only show those with counts > 0
            for error_type in sorted(type_counts.keys()):
                count = type_counts[error_type]
                if count > 0:  # Only print types with errors
                    print(f"  {error_type}: {count}")

        print("\n" + "=" * 50)
        print(f"Total number of errors: {total_errors}")

    except FileNotFoundError:
        print(f"Error: Could not find file '{filename}'")
    except Exception as e:
        print(f"Error processing file: {e}")

if __name__ == "__main__":
    main()
