/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */

class Solution {
private:
        ListNode* mergeTwoLists(ListNode* l1, ListNode* l2)
        {
            ListNode* dummy = new ListNode();
            ListNode* res = dummy;
            while(l1 && l2){
                if(l1->val < l2->val){
                    dummy->next = l1;
                    l1 = l1->next;
                }

                else{
                    dummy->next = l2;
                    l2 = l2->next;
                }
                dummy = dummy->next;
            }

            dummy->next = l1 ? l1 : l2;

            ListNode* head = res->next;
            delete res;
            return head;

        }

public:
    ListNode* mergeKLists(vector<ListNode*>& lists) {
        int n = lists.size();

        if(n == 0){
            return nullptr;
        }

        if(n == 1){
            return lists.at(0);
        }


        ListNode* result = lists.at(0);

        for(int i = 1; i < lists.size(); i++){
            result = mergeTwoLists(result, lists.at(i));
        }
        return result;
    }
};
