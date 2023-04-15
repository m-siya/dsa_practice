#include <bits/stdc++.h>
using namespace std;
// Definition for a QuadTree node.
class Node
{
public:
    bool val;
    bool isLeaf;
    Node *topLeft;
    Node *topRight;
    Node *bottomLeft;
    Node *bottomRight;

    Node()
    {
        val = false;
        isLeaf = false;
        topLeft = NULL;
        topRight = NULL;
        bottomLeft = NULL;
        bottomRight = NULL;
    }

    Node(bool _val, bool _isLeaf)
    {
        val = _val;
        isLeaf = _isLeaf;
        topLeft = NULL;
        topRight = NULL;
        bottomLeft = NULL;
        bottomRight = NULL;
    }

    Node(bool _val, bool _isLeaf, Node *_topLeft, Node *_topRight, Node *_bottomLeft, Node *_bottomRight)
    {
        val = _val;
        isLeaf = _isLeaf;
        topLeft = _topLeft;
        topRight = _topRight;
        bottomLeft = _bottomLeft;
        bottomRight = _bottomRight;
    }
};

Node *one = new Node(true, true);
Node *zero = new Node(false, true);

class Solution
{
public:
    Node *construct(vector<vector<int>> &grid)
    {
        int n = grid.size();
        return _construct(grid, 0, 0, n);
    }

    Node *_construct(vector<vector<int>> &grid, int startX, int startY, int gridSize)
    {
        {
            int f = grid[startY][startX];
            bool ass = true;
            for (int i = startX; i < startX + gridSize; i++)
                for (int j = startY; j < startY + gridSize; j++)
                    if (f != grid[j][i])
                    {
                        ass = false;
                        break;
                    }
            if (ass)
                return f == 1 ? one : zero;
        }

        int nextGridSize = gridSize / 2;
        Node *topLeft = _construct(grid, startX, startY, nextGridSize);
        Node *topRight = _construct(grid, startX + nextGridSize, startY, nextGridSize);
        Node *bottomLeft = _construct(grid, startX, startY + nextGridSize, nextGridSize);
        Node *bottomRight = _construct(grid, startX + nextGridSize, startY + nextGridSize, nextGridSize);

        if (topLeft->isLeaf && topRight->isLeaf && bottomLeft->isLeaf && bottomRight->isLeaf &&
            topLeft->val == topRight->val && topRight->val == bottomLeft->val && bottomLeft->val == bottomRight->val)
        {
            return topLeft;
        }
        return new Node(true, false, topLeft, topRight, bottomLeft, bottomRight);
    }
};
