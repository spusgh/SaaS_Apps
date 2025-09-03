# Package Sorting System

## Overview

This system implements an automated package sorting function for Thoughtful's robotic automation factory. The robotic arm uses this function to dispatch packages to the correct stack based on their physical dimensions and mass.

## Problem Statement

The robotic arm needs to sort packages into three different stacks based on whether they are "bulky" or "heavy":

- **STANDARD**: Packages that can be handled normally
- **SPECIAL**: Packages requiring special handling  
- **REJECTED**: Packages that cannot be processed

## Sorting Criteria

### Bulky Package Definition
A package is considered **bulky** if:
- Its volume (Width × Height × Length) ≥ 1,000,000 cm³, **OR**
- Any single dimension ≥ 150 cm

### Heavy Package Definition  
A package is considered **heavy** if:
- Its mass ≥ 20 kg

### Stack Assignment Rules
- **STANDARD**: Packages that are neither bulky nor heavy
- **SPECIAL**: Packages that are either bulky OR heavy (but not both)
- **REJECTED**: Packages that are both bulky AND heavy

## Function Specification

```python
def sort(width, height, length, mass):
    """
    Sorts packages based on dimensions and mass.
    
    Args:
        width (float): Width in centimeters
        height (float): Height in centimeters  
        length (float): Length in centimeters
        mass (float): Mass in kilograms
    
    Returns:
        str: "STANDARD", "SPECIAL", or "REJECTED"
    """
```

## Usage Examples

```python
# Standard package (small and light)
sort(50, 30, 20, 15)  # Returns: "STANDARD"

# Special package (bulky due to dimension)
sort(200, 30, 20, 15)  # Returns: "SPECIAL"

# Special package (heavy but not bulky)
sort(50, 30, 20, 25)  # Returns: "SPECIAL"

# Rejected package (both bulky and heavy)
sort(200, 30, 20, 25)  # Returns: "REJECTED"
```

## Test Cases

The implementation includes comprehensive test cases covering:

- Standard packages (neither bulky nor heavy)
- Bulky packages due to volume threshold
- Bulky packages due to dimension threshold  
- Heavy packages
- Rejected packages (both bulky and heavy)
- Edge cases at exact threshold values

## Running the Code

### Prerequisites
- Python 3.6 or higher
- No external dependencies required

### Execution
```bash
# Run the main script with built-in tests
python package_sorter.py

# Import and use the function in your code
from package_sorter import sort

result = sort(100, 50, 40, 15)
print(result)  # Output: "STANDARD"
```

## Implementation Details

The function follows this logical flow:

1. Calculate the package volume: `width × height × length`
2. Check if package is bulky:
   - Volume ≥ 1,000,000 cm³ OR any dimension ≥ 150 cm
3. Check if package is heavy:
   - Mass ≥ 20 kg
4. Apply sorting rules:
   - If bulky AND heavy → "REJECTED"
   - If bulky OR heavy → "SPECIAL" 
   - Otherwise → "STANDARD"

## Edge Cases Handled

- Packages with dimensions exactly at 150 cm threshold
- Packages with mass exactly at 20 kg threshold
- Packages with volume exactly at 1,000,000 cm³ threshold
- Very small packages (all dimensions and mass well below thresholds)
- Packages that are bulky due to volume vs. individual dimensions

## Quality Assurance

The implementation includes:
- Comprehensive unit tests with 10 test cases
- Clear documentation and comments
- Input validation through type hints
- Edge case coverage
- Example usage demonstrations

## Factory Integration

This function is designed to be integrated into Thoughtful's robotic automation system where:
- The robotic arm measures package dimensions and weighs each package
- The measurements are passed to this `sort()` function
- The returned stack designation directs the robotic arm to place the package in the correct location
- The system can handle high-throughput package processing with reliable classification

## Performance Considerations

- **Time Complexity**: O(1) - constant time for all operations
- **Space Complexity**: O(1) - no additional data structures needed
- **Scalability**: Function can handle thousands of packages per minute
- **Reliability**: Simple logic reduces chance of sorting errors