import time

# 定义 printf 函数
def printf(num):
     num

# 定义 Obj 类
class Obj:
    def __init__(self, printf):
        self.printf = printf

# 创建 Obj 类的实例
obj = Obj(printf)

# 记录开始时间，单位为毫秒
start = int(time.time() * 1000)

# 循环 100 次
for a in range(100000):
    obj.a = a
    obj.printf(obj.a)

# 记录结束时间，单位为毫秒
end = int(time.time() * 1000)

# 计算并打印耗时
print(f"py:{end - start}ms")
