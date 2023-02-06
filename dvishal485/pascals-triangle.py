from itertools import pairwise
from typing import List

class Solution:
    def generate(self, numRows: int) -> List[List[int]]:
        dp = [[1]]
        for i in range(1, numRows):
            dp.append([x+y for x, y in pairwise([0]+dp[i-1]+[0])])
        return dp
