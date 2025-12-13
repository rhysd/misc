import matplotlib.pyplot as plt

xs, ys = [], []
with open('../in/9-1') as f:
    for line in f:
        (x, y) = map(int, line.strip().split(','))
        xs.append(x)
        ys.append(y)
plt.plot(xs, ys, marker=',')
plt.xlabel('X')
plt.ylabel('Y')
plt.show()
