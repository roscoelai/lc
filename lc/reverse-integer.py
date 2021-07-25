#!/usr/bin/env python3
# reverse-integer.py

class Solution:
    def reverse(self, x: int) -> int:
        aux = abs(x)
        res = 0

        while aux > 0:
            res = res * 10 + (aux % 10)
            aux = int(aux / 10)

        if x < 0:
            return -res
        return res

if __name__ == "__main__":

    import unittest

    class TestSolution(unittest.TestCase):

        def helper(self, x: int, want: int) -> None:
            got = Solution.reverse(None, x)
            self.assertEqual(got, want)

        def test_reverse_example_1(self) -> None:
            self.helper(123, 321)

        def test_reverse_example_2(self) -> None:
            self.helper(-123, -321)

        def test_reverse_example_3(self) -> None:
            self.helper(120, 21)

        def test_reverse_example_4(self) -> None:
            self.helper(0, 0)

    unittest.main()
