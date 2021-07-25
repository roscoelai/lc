#!/usr/bin/env python3
# two-sum.py

from typing import List

class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        d = {}
        for j, num in enumerate(nums):
            i = d.get(target - num, None)
            if i is not None:
                return [i, j]
            d[num] = j
        ValueError("Impossible to have no valid answers!")

if __name__ == "__main__":

    import unittest

    class TestSolution(unittest.TestCase):

        def helper(self, nums: List[int], target: int, want: List[int]):
            got = Solution.twoSum(None, nums, target)
            self.assertEqual(got, want)

        def test_twoSum_example_1(self):
            self.helper([2, 7, 11, 15], 9, [0, 1])

        def test_twoSum_example_2(self):
            self.helper([3, 2, 4], 6, [1, 2])

        def test_twoSum_example_3(self):
            self.helper([3, 3], 6, [0, 1])

    unittest.main()
