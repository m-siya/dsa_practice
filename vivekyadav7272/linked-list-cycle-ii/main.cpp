#include <bits/stdc++.h>
using namespace std;

struct ListNode
{
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(NULL) {}
};

class Solution
{
public:
    ListNode *detectCycle(ListNode *head)
    {
        if (!head)
            return nullptr;

        ListNode *slow = head;
        ListNode *fast = head;
        bool first_time = true;
        while (first_time || slow != fast)
        {
            first_time = false;
            if (slow->next)
                slow = slow->next;
            else
                return nullptr;

            if (fast->next && fast->next->next)
                fast = fast->next->next;
            else
                return nullptr;
        }

        cout << "Met at " << slow->val << endl;
        // Now both are at (n - m%n)
        // Now that we have not early exit, it means there definitely is a cycle, so
        // no null checks required.
        slow = head;

        while (slow != fast)
        {
            slow = slow->next;
            fast = fast->next;
        }

        cout << "Met again at " << slow->val << endl;

        return slow;
    }
};

ListNode *make_ll(vector<int> &v, int k)
{
    ListNode *head;
    head = new ListNode(v[0]);
    ListNode *traversal_node = head;
    int curr_node = 0;
    ListNode *cycle_head = nullptr;
    for (int i = 1; i < v.size(); i++)
    {
        if (curr_node == k)
        {
            cycle_head = traversal_node;
        }
        traversal_node->next = new ListNode(v[i]);
        traversal_node = traversal_node->next;
        curr_node++;
    }

    traversal_node->next = cycle_head;

    return head;
}

void print_ll(ListNode *head, int sz)
{
    cout << "Printing..." << endl;
    for (int i = 0; i < sz; i++)
    {
        cout << head->val << " -> ";
        head = head->next;
    }
    cout << endl;
}

int main()
{
    vector<int> vec1 = {3, 2, 0, -4};
    ListNode *l1 = make_ll(vec1, 1);
    print_ll(l1, 5);
    ListNode *cycle_head = Solution().detectCycle(l1);
    if (cycle_head->val != 2)
    {
        cout << "Wrong ans!" << endl;
    }
    else
    {
        cout << "Correct ans!" << endl;
    }
}