local a = {"Hello", "world"}

local printVal = nil
function log(...)
    printVal = ...
    print(printVal)
end

local i = 1
while i <= #a do
    log(a[i])
    i = i + 1
end