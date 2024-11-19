import matplotlib.pyplot as plt
assets = [29429, 30000]
data= ["2024-11-19", "2024-11-20"]

growth = [0]

for i in range(1, len(assets)):
    print(i)
    growth.append(assets[i] - assets[i-1])


fig, (ax1, ax2) = plt.subplots(2, 1, figsize=(10,8))

bars1 = ax1.bar(data, assets)
ax1.set_title('Assets Over Time')
ax1.set_xlabel('Date')
ax1.set_ylabel('Asset Value')
for bar in bars1:
    height = bar.get_height()
    ax1.text(bar.get_x() + bar.get_width()/2., height,
             f'${int(height):,}',
             ha='center', va='bottom')

bars2 = ax2.bar(data, growth)
ax2.set_title('Growth Over Time') 
ax2.set_xlabel('Date')
ax2.set_ylabel('Growth')
for bar in bars2:
    height = bar.get_height()
    ax2.text(bar.get_x() + bar.get_width()/2., height,
             f'${int(height):,}',
             ha='center', va='bottom')

plt.tight_layout()
plt.show()