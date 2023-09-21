import json
import os
from pathlib import Path


def is_special_directory(path):
    return path.name.startswith(".") or path.name in ["template", "target"]


if __name__ == "__main__":
    ROOT_PATH = Path(__file__).parent.parent
    TEST_CASES_PATH = ROOT_PATH / "test-cases"

    # Get list of all directories
    subfolders = [f for f in os.scandir(TEST_CASES_PATH) if f.is_dir()]

    # Filter out special directories
    subfolders = [f for f in subfolders if not is_special_directory(f)]

    # Get list of all test cases
    testcases = [f.name for f in subfolders]
    testcases.sort()

    # Print list as JSON
    print(json.dumps(testcases))
