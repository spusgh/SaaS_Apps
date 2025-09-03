def sort(width, height, length, mass):
    """
    Sorts packages based on their dimensions and mass according to Thoughtful's 
    robotic automation factory specifications.
    
    Args:
        width (float): Width of the package in centimeters
        height (float): Height of the package in centimeters  
        length (float): Length of the package in centimeters
        mass (float): Mass of the package in kilograms
    
    Returns:
        str: The stack designation - "STANDARD", "SPECIAL", or "REJECTED"
    """
    
    # Calculate volume
    volume = width * height * length
    
    # Determine if package is bulky
    is_bulky = (volume >= 1_000_000 or 
                width >= 150 or 
                height >= 150 or 
                length >= 150)
    
    # Determine if package is heavy
    is_heavy = mass >= 20
    
    # Apply sorting rules
    if is_bulky and is_heavy:
        return "REJECTED"
    elif is_bulky or is_heavy:
        return "SPECIAL"
    else:
        return "STANDARD"


# Test cases to verify the implementation
def test_sort():
    """Test function to verify the sorting logic"""
    
    # Test cases with expected results
    test_cases = [
        # (width, height, length, mass, expected_result, description)
        (10, 10, 10, 5, "STANDARD", "Small, light package"),
        (200, 10, 10, 5, "SPECIAL", "Bulky due to dimension >= 150"),
        (50, 50, 50, 5, "STANDARD", "Normal size, light package"),
        (100, 100, 100, 5, "SPECIAL", "Bulky due to volume >= 1,000,000"),
        (10, 10, 10, 25, "SPECIAL", "Small but heavy package"),
        (200, 10, 10, 25, "REJECTED", "Both bulky and heavy"),
        (149, 149, 149, 19.9, "STANDARD", "Just under all thresholds"),
        (150, 10, 10, 19.9, "SPECIAL", "Exactly at dimension threshold"),
        (10, 10, 10, 20, "SPECIAL", "Exactly at mass threshold"),
        (80, 80, 156.25, 15, "SPECIAL", "Volume exactly 1,000,000"),
    ]
    
    print("Running test cases...")
    print("=" * 60)
    
    all_passed = True
    for i, (w, h, l, m, expected, description) in enumerate(test_cases, 1):
        result = sort(w, h, l, m)
        passed = result == expected
        all_passed &= passed
        
        status = "✓ PASS" if passed else "✗ FAIL"
        print(f"Test {i:2d}: {status} - {description}")
        print(f"         Dimensions: {w}×{h}×{l} cm, Mass: {m} kg")
        print(f"         Expected: {expected}, Got: {result}")
        if not passed:
            volume = w * h * l
            is_bulky = (volume >= 1_000_000 or w >= 150 or h >= 150 or l >= 150)
            is_heavy = m >= 20
            print(f"         Volume: {volume:,} cm³, Bulky: {is_bulky}, Heavy: {is_heavy}")
        print()
    
    print("=" * 60)
    if all_passed:
        print("🎉 All tests passed!")
    else:
        print("❌ Some tests failed. Please check the implementation.")
    
    return all_passed


if __name__ == "__main__":
    # Run tests when script is executed directly
    test_sort()
    
    # Example usage
    print("\nExample usage:")
    print("-" * 30)
    print(f"sort(50, 30, 20, 15) = {sort(50, 30, 20, 15)}")
    print(f"sort(200, 30, 20, 15) = {sort(200, 30, 20, 15)}")
    print(f"sort(50, 30, 20, 25) = {sort(50, 30, 20, 25)}")
    print(f"sort(200, 30, 20, 25) = {sort(200, 30, 20, 25)}")
