local a = {"Hello", "world"}

local function log(...)
    print(...)
end

local i = 1
while i <= #a do
    log(a[i])
    i = i + 1
end
