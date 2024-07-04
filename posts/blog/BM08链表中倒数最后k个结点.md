---
image_path: "/images/blog_images/leetcode/BM08-fig1.png"
title: BM08 链表中倒数最后k个结点
date: 2024-07-04
description: 每日一题：链表中倒数最后k个结点
project_link: none
tags:
  - learn
---
## 描述

输入一个长度为$n$﻿的链表，设链表中的元素的值为$a_i$﻿，返回该链表中倒数第$k$﻿个节点。

如果该链表长度小于$k$﻿，请返回一个长度为 0 的链表。

数据范围：$0≤n≤105$﻿，$0≤a_i≤109$﻿，$0≤k≤109$﻿

要求：空间复杂度 _$O(n)$_﻿，时间复杂度 _$O(n)$_﻿

进阶：空间复杂度 _$O(1)$_﻿，时间复杂度 _$O(n)$_﻿

例如输入{1,2,3,4,5},2时，对应的链表结构如下图所示：

![BM08-fig1.png](/images/blog_images/leetcode/BM08-fig1.png)

其中蓝色部分为该链表的最后2个结点，所以返回倒数第2个结点（也即结点值为4的结点）即可，系统会打印后面所有的节点来比较。

## 示例1

```Plain
输入：{1,2,3,4,5},2
返回值：{4,5}
说明：返回倒数第2个节点4，系统会打印后面所有的节点来比较。
```

## 示例2

```Plain
输入：{2},8
返回值：{}
```

## 解题思路

### 明确问题与步骤

1. 循环取得链表长度$len$﻿
2. 设置答案节点ans，循环$len-k$﻿次后返回即可
3. 异常场景：链表为空，$k=0$﻿，$len<k$﻿

```Python
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None
#
# 代码中的类名、方法名、参数名已经指定，请勿修改，直接返回方法规定的值即可
#
# 
# @param pHead ListNode类 
# @param k int整型 
# @return ListNode类
#
class Solution:
    def FindKthToTail(self , pHead: ListNode, k: int) -> ListNode:
        if not pHead and k == 0:
            return None
        
        len = 0
        curr = pHead
        while curr:
            len += 1
            curr = curr.next
        
        if len < k:
            return None

        ans = pHead
        for i in list(range(len - k)):
            ans = ans.next
        
        return ans
```