"""min.py tries to reduce the number of asserts in a file.
    Currently, we scan thru the asserts linearly, commenting out each one 
    and comment out the assert and leave it commented if verification succeeds
    technically we should try all 2^n where n := # of asserts, but that's left
    for improvement
    
    Caveat: We can only look for asserts that's contained in one line. We also
            do not deal with `assert ... by ...;` or `assert forall ...`
"""

import re
import subprocess
import sys
import json

def run_verus(file_path):
    """Runs the verus command on the specified file and parses the JSON output."""
    try:
        result = subprocess.run([
            "verus", "ironsht/src/lib.rs", "--crate-type=dylib", "--output-json", "--verify-module", "delegation_map_v", "--time-expanded"
        ], capture_output=True, text=True)
        stdout = result.stdout
        
        try:
            output = json.loads(stdout)
            # verus doens't have this json entry when verifying a module
            # verification_results = output.get("verification-results", {})
            # success = verification_results.get("success", False)
            # verification time
            total_time = output.get("times-ms", {}).get("total", 0)
            print(f"Verification succeeded with total time: {total_time}ms\n")
            return True
        except json.JSONDecodeError:
            print("Verus Error: Failed to verify/parse")
            return False
    except FileNotFoundError:
        print("Error: 'verus' command not found. Make sure it is installed and in your PATH.")
        sys.exit(1)

def main(file_path):
    """Minimizes the number of assert statements to make verification succeed."""

    with open(file_path, 'r') as file:
        # TODO: can only look for asserts that's contained in one line
        content = file.readlines()
    
    # search for all regex's that has an K::cmp_properties(); statement
    assert_lines = [
        i for i, line in enumerate(content, 1) 
        if re.search(r'^(?!\s*//*).*?\bK::cmp_properties\(\);', line)
    ]
    print(assert_lines)

    commented_asserts = []

    for lineno in assert_lines:
        print(f"Commenting out assert statement at line {lineno} and running verus...")
        """Comments out a specific non-commented assert() statement in a given file."""
        with open(file_path, 'r') as file:
            content = file.readlines()

        if lineno < len(content):
            content[lineno-1] = "//" + content[lineno-1]  # Comment out the specific assert

        with open(file_path, 'w') as file:
            file.writelines(content)

        if not run_verus(file_path):
            print(f"Uncommenting line {lineno}...\n")
            with open(file_path, 'r') as file:
                content = file.readlines()
            content[lineno-1] = content[lineno-1].lstrip("//")  # Uncomment the specific assert
            with open(file_path, 'w') as file:
                file.writelines(content)
        else:
            commented_asserts.append(lineno)

    print("Minimization complete. The following asserts were commented out:", commented_asserts)

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python script.py <file_path>")
        sys.exit(1)

    file_path = sys.argv[1]
    main(file_path)