local tbl = {len = 1, data = {hi = "hi"}}
tbl.world = "world"

for _ = 0, tbl.len do
    print(tbl.data.hi)
    print(tbl.world)
end
