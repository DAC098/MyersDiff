#  Returns a minimal list of differences between 2 lists e and f
#  requring O(min(len(e),len(f))) space and O(min(len(e),len(f)) * D)
#  worst-case execution time where D is the number of differences.
def diff(left, right, left_index=0, right_index=0):
  print(left, right, left_index, right_index)

  #  Documented at http://blog.robertelder.org/diff-algorithm/
  left_len, right_len, total_len, Z = len(left), len(right), len(left) + len(right), 2 * min(len(left), len(right)) + 2

  print("Z", Z)
  
  if left_len > 0 and right_len > 0:
    w, g, p = left_len - right_len, [0] * Z, [0] * Z

    print("h_range:", (total_len//2 + (total_len % 2 != 0)) + 1)

    for h in range(0, (total_len//2 + (total_len % 2 != 0)) + 1):
      for r in range(0, 2):
        c, d, o, m = (g, p, 1, 1) if r == 0 else (p, g, 0, -1)

        k_start = -(h - 2 * max(0, h - right_len))
        k_end = h - 2 * max(0, h - left_len) + 1

        print("o:", o, "m:", m, "k: ", k_start, "..", k_end)

        for k in range(k_start, k_end, 2):
          a = c[(k + 1) % Z] if (k == -h or k != h and c[(k - 1) % Z] < c[(k + 1) % Z]) else c[(k - 1) % Z] + 1
          b = a - k
          s, t = a, b

          print("base a:", a, "b:", b, "s:", s, "t:", t)

          loop_count = 0
          
          while a < left_len and b < right_len:
            left_check = (1 - o) * left_len + m * a + (o - 1)
            right_check = (1 - o) * right_len + m * b + (o - 1)
            
            print("-- left_check (1 - {o}) * {left_len} + {m} * {a} + ({o} - 1) =".format(
                o=o, left_len=left_len, m=m, a=a
            ), left_check, "right_checK:", right_check)
            
            if left[left_check] != right[right_check]:
                break
            
            a += 1
            b += 1
            loop_count += 1

          print("updated a:", a, "b:", b, "count:", loop_count)

          c[k % Z], z = a, -(k - w)

          if total_len % 2 == o and z >= -(h - o) and z <= h - o and c[k % Z] + d[z % Z] >= left_len:
            D, x, y, u, v = (2 * h - 1, s, t, a, b) if o == 1 else (2 * h, left_len - a, right_len - b, left_len - s, right_len - t)

            if D > 1 or (x != u and y != v):
              print("recursive first branch. x:", x, "y:", y, "u:", u, "v:", v)

              return diff(left[0:x], right[0:y], left_index, right_index) + diff(left[u:left_len], right[v:right_len], left_index + u,right_index + v)
            elif right_len > left_len:
              print("recursive second branch")

              return diff([], right[left_len:right_len], left_index + left_len, right_index + left_len)
            elif right_len < left_len:
              print("recursive thrid branch")

              return diff(left[right_len:left_len], [], left_index + right_len, right_index + right_len)
            else:
              print("return empty")

              return []

  elif left_len > 0: #  Modify the return statements below if you want a different edit script format
    return [{"operation": "delete", "position_old": left_index + n} for n in range(0, left_len)]
  else:
    return [{"operation": "insert", "position_old": left_index, "position_new":right_index+n} for n in range(0, right_len)]
    
left_str = "abgdef"
right_str = "gh"

for op in diff(left_str, right_str):
    print(op)
    
check = True
a = 2 if check else 3 + 1
print("a", a)

foo, bar = 2, 3
print("foo", foo)
print("bar", bar)

test_list = [1, 2, 3, 4, 5, 6]
subset = test_list[0:2]

print("subset", subset)

for v in range(-1, 2):
    print("range", v)