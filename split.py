

split_token = input("Split token: ").strip()
texts = open(input("File to split: "), "r").read().split(split_token)
ratio = float(input("Split ratio: "))
split_index = int(len(texts)*ratio)
open(input("Save to: "), "w").write(split_token.join(texts[:split_index]))
print("Completed!")
