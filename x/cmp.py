n = 200  # 你想计算的斐波那契数列的项数

fibPrevPrev = 0  # 斐波那契数列的前前项
fibPrev = 1     # 斐波那契数列的前一项
fibCurrent = 0

for i1 in range(2, n):
    for i2 in range(2, n):
        print(i1 + i2)  # Python中print默认会在末尾添加换行符，end=''用来阻止这个行为c