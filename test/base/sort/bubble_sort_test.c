/**
 * 冒泡排序算法
 * 重复地走访过要排序的数列，一次比较两个元素，
 * 如果他们的顺序（如从大到小、首字母从A到Z）错误就把他们交换过来。
 * */
// Created by weiwei on 2019/1/27.
//
#include "sort.h"

#include <stdio.h>
int main(int argc, char **argv)
{
    int arr[] = { 22, 34, 3, 32, 82, 55, 89, 50, 37, 5, 64, 35, 9, 70 };
    int len = (int) sizeof(arr) / sizeof(*arr);
    bubble_sort(arr, len);
    int i;
    for (i = 0; i < len; i++)
        printf("%d ", arr[i]);
    printf("\n");
    return 0;
}