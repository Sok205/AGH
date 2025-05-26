"""
Linear Sort Algorithms with visualization using Matplotlib.

Install Matplotlib if you haven't already.

'pip install matplotlib'

If you want to learn more about each algorith go to this links:
- Radix Sort: https://www.youtube.com/watch?v=XiuSW_mEn7g
- Counting Sort: https://www.youtube.com/watch?v=OKd534EWcdk
- Bucker Sort: https://www.youtube.com/watch?v=VuXbEb5ywrU
"""

import matplotlib.pyplot as plt
import numpy as np


def test_visualization_bubble(arr) -> None:
    """
    Test function for bubble sort visualization. (Bubble Sort is not a linear sort algorithm, but included for testing purposes.)
    :return:
    """
    n = len(arr)

    x = np.arange(0, n, 1)

    for i in range(n):
        for j in range(0, n-i-1):
            plt.clf()
            plt.bar(x, arr)
            plt.bar(x, arr)
            plt.title(f'Bubble Sort: Iteration {i}, Step {j}')
            plt.xlabel('Index')
            plt.ylabel('Value')
            plt.pause(0.01)

            if arr[j] > arr[j+1]:
                arr[j], arr[j+1] = arr[j+1], arr[j]

    plt.clf()
    plt.bar(x, arr, color='green')
    plt.title('Final Sorted Array')
    plt.draw()
    plt.show()


def counting_sort_radix(arr, exp=1, visualize=True):
    """
    Counting Sort Algorithm modified for Radix Sort with visualization
    :param arr: List of integers to be sorted
    :param exp: Current digit place value (1, 10, 100, etc.)
    :param visualize: Whether to show visualization
    :return: Sorted list of integers
    """
    n = len(arr)
    output = [0] * n
    count = [0] * 10  # Only 0-9 digits

    # Close any existing figures to prevent memory leak
    if visualize:
        plt.close('all')
        fig = plt.figure(figsize=(10, 6))
        x = np.arange(0, n, 1)

        # Show original array
        plt.bar(x, arr, color='blue')
        plt.title(f'Original Array (Sorting by {exp}\'s place)')
        plt.xlabel('Index')
        plt.ylabel('Value')
        plt.draw()
        plt.pause(0.5)

    for i in range(n):
        index = (arr[i] // exp) % 10
        count[index] += 1

        if visualize and i % 3 == 0:
            plt.clf()
            bars = plt.bar(x, arr, color='blue')
            bars[i].set_color('red')
            plt.title(f'Counting digits at {exp}\'s place: processing {i + 1}/{n}')
            plt.draw()
            plt.pause(0.1)

    if visualize:
        plt.clf()
        count_x = np.arange(0, 10, 1)
        plt.bar(count_x, count, color='green')
        plt.title(f'Count Array: Frequency of each digit at {exp}\'s place')
        plt.xlabel('Digit (0-9)')
        plt.ylabel('Count')
        plt.draw()
        plt.pause(0.5)

    for i in range(1, 10):
        count[i] += count[i - 1]

        if visualize:
            plt.clf()
            plt.bar(count_x, count, color='orange')
            plt.title(f'Building Cumulative Sum: Step {i}/9')
            plt.xlabel('Digit (0-9)')
            plt.ylabel('Cumulative Count')
            plt.draw()
            plt.pause(0.1)

    original = arr.copy()
    for i in range(n - 1, -1, -1):
        index = (original[i] // exp) % 10
        output[count[index] - 1] = original[i]
        count[index] -= 1

        if visualize:
            plt.clf()
            bars = plt.bar(x, output, color='lightblue')
            pos = count[index]
            if pos < len(bars):
                bars[pos].set_color('red')
            plt.title(f'Placing {original[i]} (digit {index}) at position {pos}')
            plt.xlabel('Index')
            plt.ylabel('Value')
            plt.draw()
            plt.pause(0.2)

    for i in range(n):
        arr[i] = output[i]

    if visualize:
        plt.clf()
        plt.bar(x, arr, color='green')
        plt.title(f'Array after sorting by {exp}\'s place')
        plt.xlabel('Index')
        plt.ylabel('Value')
        plt.draw()
        plt.pause(0.5)
        plt.close()

    return arr


def radix_sort(arr, visualize=True):
    """
    Radix Sort Algorithm with visualization
    :param arr: List of integers to be sorted
    :param visualize: Whether to show visualization
    :return: Sorted list of integers
    """
    max_val = max(arr)
    exp = 1

    while max_val // exp > 0:
        counting_sort_radix(arr, exp, visualize)
        exp *= 10

    if visualize:
        # Final visualization
        plt.figure(figsize=(10, 6))
        x = np.arange(0, len(arr), 1)
        plt.bar(x, arr, color='purple')
        plt.title('Final Sorted Array (Radix Sort)')
        plt.xlabel('Index')
        plt.ylabel('Value')
        plt.draw()
        plt.show()
        plt.close('all')

    return arr

def insertion_sort(arr, visualize=True):
    """
    Insertion Sort Algorithm with visualization
    :param arr: List of integers to be sorted
    :param visualize: Whether to show visualization
    :return: Sorted list of integers
    """
    n = len(arr)
    if visualize:
        plt.close('all')
        fig = plt.figure(figsize=(10, 6))
        x = np.arange(0, n, 1)

    for i in range(1, n):
        key = arr[i]
        j = i - 1

        while j >= 0 and key < arr[j]:
            arr[j + 1] = arr[j]
            j -= 1

            if visualize:
                plt.clf()
                plt.bar(x, arr, color='blue')
                plt.bar(x, arr)
                plt.title(f'Insertion Sort: Inserting {key} at position {j + 1}')
                plt.xlabel('Index')
                plt.ylabel('Value')
                plt.pause(0.01)

        arr[j + 1] = key

    if visualize:
        plt.clf()
        plt.bar(x, arr, color='green')
        plt.title('Final Sorted Array (Insertion Sort)')
        plt.xlabel('Index')
        plt.ylabel('Value')
        plt.draw()
        plt.show()

    return arr


def bucket_sort(arr, visualize=True):
    """
    Bucket Sort Algorithm with visualization
    :param arr: List of integers to be sorted
    :param visualize: Whether to show visualization
    :return: Sorted list of integers
    """
    if not arr:
        return arr

    if visualize:
        plt.close('all')
        fig = plt.figure(figsize=(10, 6))

    max_val = max(arr)
    bucket_count = len(arr)
    buckets = [[] for _ in range(bucket_count)]

    for num in arr:
        index = min(int(num * bucket_count / (max_val + 1)), bucket_count - 1)
        buckets[index].append(num)

        if visualize:
            plt.clf()
            bucket_sizes = [len(b) for b in buckets]
            bars = plt.bar(range(len(buckets)), bucket_sizes, color='blue')

            for i, (bar, bucket) in enumerate(zip(bars, buckets)):
                plt.text(i, bar.get_height() + 0.1, str(bucket),
                         ha='center', va='bottom', fontsize=8, rotation=45)

            plt.title(f'Bucket Sort: Adding {num} to bucket {index}')
            plt.xlabel('Bucket Index')
            plt.ylabel('Number of Elements')
            plt.pause(0.5)

    sorted_arr = []
    for i, bucket in enumerate(buckets):
        if bucket:
            sorted_bucket = insertion_sort(bucket, visualize=False)
            sorted_arr.extend(sorted_bucket)

            if visualize:
                plt.clf()
                bucket_sizes = [len(b) for b in buckets]
                bars = plt.bar(range(len(buckets)), bucket_sizes, color='blue')
                bars[i].set_color('green')

                for j, (bar, b) in enumerate(zip(bars, buckets)):
                    content = str(b) if j != i else str(sorted_bucket)
                    plt.text(j, bar.get_height() + 0.1, content,
                             ha='center', va='bottom', fontsize=8, rotation=45)

                plt.title(f'Bucket {i} Sorted: {sorted_bucket}')
                plt.xlabel('Bucket Index')
                plt.ylabel('Number of Elements')
                plt.pause(0.5)

    # Show final sorted array
    if visualize:
        plt.clf()
        x = np.arange(0, len(sorted_arr), 1)
        bars = plt.bar(x, sorted_arr, color='green')

        # Add value labels
        for bar, val in zip(bars, sorted_arr):
            height = bar.get_height()
            plt.text(bar.get_x() + bar.get_width() / 2, height + 0.1,
                     str(val), ha='center', va='bottom', fontsize=8)

        plt.title('Final Sorted Array (Bucket Sort with Insertion Sort)')
        plt.xlabel('Index')
        plt.ylabel('Value')
        plt.draw()
        plt.show()
        plt.close('all')

if __name__ == '__main__':
    print("Orignal Array:")
    lst = np.random.randint(0, 100, 20)
    print("Choose algorithm to visualize:\n"
          "1. Bubble Sort (not linear sort)\n"
          "2. Counting Sort (linear sort)\n"
          "3. Radix Sort (linear sort)\n"
          "4. Insertion Sort (not liner sort)\n"
          "5. Bucket sort (linear)\n")
    choice = int(input("Choose algorithm: "))
    match choice:
        case 1:
            print("Visualizing Bubble Sort...")
            test_visualization_bubble(lst.tolist())
        case 2:
            print("Visualizing Counting Sort...")
            counting_sort_radix(lst.tolist())
        case 3:
            print("Visualizing Radix Sort...")
            radix_sort(lst.tolist())
        case 4:
            print("Visualizing Insertion Sort...")
            insertion_sort(lst.tolist())
        case 5:
            print("Visualizing Bucket Sort...")
            bucket_sort(lst.tolist())
        case _:
            print("Invalid choice.")
