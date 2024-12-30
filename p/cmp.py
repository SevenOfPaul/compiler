import time
from datetime import datetime
def item(n):
    denominator = 2 * n + 1
    sign = (-1) ** n  # 这行代码计算符号，-1 的 n 次幂
    return sign / denominator

def leibniz(k):
    quarter = 0
    for i in range(k):
        quarter += item(i)
    return 4 * quarter

# 打印当前时间
print(datetime.fromtimestamp(time.time()).strftime('%Y-%m-%d %H:%M:%S'))

# 计算并打印 leibniz(4000) 的结果
print(leibniz(4000))

# 再次打印当前时间
print(datetime.fromtimestamp(time.time()).strftime('%Y-%m-%d %H:%M:%S'))