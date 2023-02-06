ass Solution:
        def getRow(self, rowIndex: int) -> List[int]:
                    return (math.comb(rowIndex, r) for r in range(rowIndex + 1))
