<!--
 * @Author: MoeMoeFish moemoefish@qq.com
 * @Date: 2022-10-19 12:53:26
 * @LastEditors: MoeMoeFish moemoefish@qq.com
 * @LastEditTime: 2022-10-19 12:57:26
 * @Description: 
-->
# 解题思路 
根据题目要求直接遍历数组并找出符合要求的值即可，需要一个 dict 记录值与出现的次数。在遍历完成后，在 dict 中找到值最大的。

时间复杂度：两个过程最多需要遍历 2 次， O(n)
空间复杂度：需要记录每个值的出现次数。O(n)