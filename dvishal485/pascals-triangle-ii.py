from itertools import pairwise
from typing import List

class Solution:
    def getRow(self, rowIndex: int) -> List[int]:
        prev = [1]
        for _ in range(rowIndex):
            prev = [x+y for x, y in pairwise([0]+prev+[0])]
        return prev
