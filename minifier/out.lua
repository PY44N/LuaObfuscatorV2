local a={1,0,2}local b={len=a[0],data={hi="hi"}}b.world="world"b.data.hey="hey"b.len=a[1]b.data.hi="hello"for c=a[1],b.len do print(math.abs(-a[2]))print(b.data.hi)print(b.world)print(b.data.hey)end