local tbl = {len = 1, data = {hi = "hi"}}
tbl.world = "world"
tbl.data.hey = "hey"

tbl.len = 0;
tbl.data.hi = "hello"

for _ = 0, tbl.len do
    print(math.abs(-2))
    print(tbl.data.hi)
    print(tbl.world)
    print(tbl.data.hey)
end
