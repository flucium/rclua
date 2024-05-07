local count = 0

function hanoi(n, a, b, c)
    if n > 0 then
        hanoi(n-1, a, c, b)
        
        count = count + 1
        
        hanoi(n-1, c, b, a)
    end
end


-- Move 3 disks from 'a' to 'b' using 'c' as a temporary peg
-- Output: 7
hanoi(3, 'a', 'b', 'c')
print(count)

-- Reset count
count = 0

-- Move 4 disks from 'a' to 'b' using 'c' as a temporary peg
-- Output: 15
hanoi(4, 'a', 'b', 'c')
print(count)

-- Reset count
count = 0

-- Move 5 disks from 'a' to 'b' using 'c' as a temporary peg
-- Output: 31
hanoi(5, 'a', 'b', 'c')
print(count)

