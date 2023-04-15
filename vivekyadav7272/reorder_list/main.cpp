#include <iostream>
#include <vector>

using ull = unsigned long long;
// Definition for singly-linked list.
struct ListNode {
    int val;
    ListNode* next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode* next) : val(x), next(next) {}
};

ListNode* prep_list(std::initializer_list<int> items) {
    
    auto item_iter = items.begin();
    ListNode* starting_node = new ListNode(*item_iter, nullptr);
    ListNode* traversal = starting_node;
    item_iter++;
    for (; item_iter != items.end(); item_iter++, traversal=traversal->next) {
        traversal->next = new ListNode(*item_iter, nullptr);
    }

    return starting_node;
}

void printLL(ListNode* start) {
    while (start != nullptr) {
        std::cout << start->val << "-> ";
        start = start->next;
    }
    std::cout << std::endl;
}

class Solution {
public:
    void reorderList(ListNode* head) {
        std::vector<ListNode*> nodes{};
        
        for (ListNode* trav = head; trav != nullptr; trav=trav->next) {
            nodes.push_back(trav);
        }
        
        ull front_ptr = 0, back_ptr = nodes.size() - 1;
        while (front_ptr < back_ptr) 
        {
            auto front_node = nodes[front_ptr++];
            auto back_node = nodes[back_ptr--];
            
            auto old_front_next = front_node->next;
            
            front_node->next = back_node;
            back_node->next = old_front_next;
        }
        
        if (front_ptr == back_ptr) {
            nodes[front_ptr]->next = nullptr;
        }
    }
};

int main() {
    auto list = {1,2,3,4,5,6,7,8};
    ListNode* head = prep_list(list);
    printLL(head);

    Solution{}.reorderList(head);
    
    printLL(head);
}