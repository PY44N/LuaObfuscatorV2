local a=function(b,c)local d=string;local e=d.char;local f=d.byte;local g=d.sub;local h=d.reverse;local i=d.find;local j=function(k,l)local m,n=i(k,l)return m-b.a end;local o=function(...)local k=c.a;local p={...}for q=b.a,#p do k=k..p[q]end;return k end;local r=select;local s=table;local t=math;local u=error;local v=pairs;local w=ipairs;local x=s.concat;local y=s.insert;local z=function(...)return{}end;local A=s.unpack or unpack;local B=function(...)return{n=r(e(b.b),...),...}end;local C=function(D,E,F,G,H)for q=b.c,F-E do H[G+q]=D[E+q]end end;local I=function(...)local J={}local K={...}for q=b.a,#K do for L=b.a,#K[q]do y(J,K[q][L])end end;return J end;local M=getfenv;local N=t.floor;local O=t.max;local P=pcall;local Q=t.abs;local R=tonumber;local S=function(T,U,V)V=V or b.a;local W=U and T or b.a;U=U or T;local m={}for q=W,U,V do y(m,q)end;return m end;local X=function()local function Y(Z,...)if(Z or b.c)==b.c then return...end;return Y(N(Z/b.d),Z%b.d,...)end;local function _(Z)if Z==b.c then return{b.c}end;return{Y(Z)}end;local function _0(_1)local function _2(Z,_3,...)if not _3 then return Z end;Z,_3=_(Z),_(_3)local _4,_5=#Z,#_3;local _6,_7={},O(_4,_5)for q=b.c,_7-b.a do local _8,_9=Z[_4-q],_3[_5-q]if not(_8 or _9)then break end;_6[_7-q]=_1((_8 or b.c)~=b.c,(_9 or b.c)~=b.c)and b.a or b.c end;return _2(R(x(_6),b.d),...)end;return _2 end;local _a=_0(function(m,_b)return m and _b end)local function _c(Z,_d)return N(Z)*b.d^_d end;local function _e(Z,_d)return N(N(Z)/b.d^_d)end;return _a,_e,_c end;local _f,_g,_h=X()local _i;local _j;local _k;local function _l(D,_m,_n,_o)local _p=b.c;for q=_m,_n,_o do local _q=b.e^Q(q-_m)_p=_p+_q*f(D,q,q)end;return _p end;local function _r(_s,_t,_u,_v,_w,_x,_y,_z)local _A=(-b.a)^_g(_z,b.f)local _B=_h(_f(_z,b.g),b.h)+_g(_y,b.h)local _C=_f(_y,b.i)*b.d^b.j;local _D=b.a;_C=_C+_x*b.d^b.k+_w*b.d^b.l+_v*b.d^b.m+_u*b.d^b.n+_t*b.d^b.o+_s;if _B==b.c then if _C==b.c then return _A*b.c else _D=b.c;_B=b.a end elseif _B==b.p then if _C==b.c then return _A*b.a/b.c else return _A*b.c/b.c end end;return _A*b.d^(_B-b.q)*(_D+_C/b.d^b.r)end;local function _E(D,_m,_n)return _l(D,_m,_n-b.a,b.a)end;local function _F(D,_m)return _r(f(D,_m,_m+b.f))end;local function _G(_H)local _I=_H[b.a]local _J=f(_H[b.d],_I,_I)_H[b.a]=_I+b.a;return _J end;local function _K(_H,_L)local _M=_H[b.a]+_L;local k=g(_H[b.d],_H[b.a],_M-b.a)_H[b.a]=_M;return k end;local function _N(_H)local _M=_H[b.a]+b.d;local _O=_E(_H[b.d],_H[b.a],_M)_H[b.a]=_M;return _O end;local function _P(_H)local _M=_H[b.a]+b.h;local _O=_E(_H[b.d],_H[b.a],_M)_H[b.a]=_M;return _O end;local function _Q(_H)local _M=_H[b.a]+b.o;local _O=_E(_H[b.d],_H[b.a],_M)_H[b.a]=_M;return _O end;local function _R(_H)local _S=_F(_H[b.d],_H[b.a])_H[b.a]=_H[b.a]+b.o;return _S end;local function _T(_H)local _L=_Q(_H)local k;if _L~=b.c then k=g(_K(_H,_L),b.a,-b.d)end;return k end;local function _U(_H)local _L=_Q(_H)local _V=z(_L)for q=b.a,_L do local _W=_N(_H)local _X=_f(_g(_W,b.h),b.s)local _Y=_f(_g(_W,b.d),b.t)local _Z=_f(_g(_W,b.a),b.a)==b.a;local __=_f(_W,b.a)==b.a;local _00={}_00[b.o]=_X;_00[b.u]=_G(_H)if _Y==b.a then _00[b.f]=_N(_H)_00[b.h]=_N(_H)_00[b.v]=_Z and _00[b.f]>b.w;_00[b.i]=__ and _00[b.h]>b.w elseif _Y==b.d then _00[b.f]=_P(_H)_00[b.n]=_Z elseif _Y==b.t then _00[b.f]=_P(_H)-b.x end;_V[q]=_00 end;return _V end;local function _01(_H,D)local _L=_Q(_H)local _V=z(_L)for q=b.a,_L do _V[q]=_k(_H,D)end;return _V end;local function _02(_H)local _L=_Q(_H)local _V=z(_L)for q=b.a,_L do local _03=_G(_H)local _04;if _03==b.t then _04=_G(_H)~=b.c elseif _03==b.c then _04=_R(_H)elseif _03==b.d then _04=_T(_H)end;_V[q]=_04 end;return _V end;function _k(_05,_06)local D=_T(_05)or _06;local _07={}_07[b.y]=D;_07[b.z]=_G(_05)_07[b.d]=_G(_05)_07[b.ab]=_02(_05)_07[b.bb]=_01(_05,D)_07[b.cb]=_U(_05)for n,_08 in w(_07[b.cb])do if _08[b.n]then _08[b.a]=_07[b.ab][_08[b.f]+b.a]else if _08[b.v]then _08[b.t]=_07[b.ab][_08[b.f]-b.w]end;if _08[b.i]then _08[b.db]=_07[b.ab][_08[b.h]-b.w]end end end;return _07 end;function _i(D)local _05={b.a,D}return _k(_05,c.a)end;local function _09(_V,_0a)for q,_0b in v(_V)do if _0b[b.a]>=_0a then _V[q]=nil end end end;local function _0c(_V,_0a,_0d)local _0e=_V[_0a]if not _0e then _0e={_0a,_0d}_V[_0a]=_0e end;return _0e end;local function _0f(_0g,_0h)local D=_0g[b.d]local _0i=b.c;u(o(D,c.b,_0i,c.b,_0h),b.c)end;local function _0j(_0k,_0l,_0m)local _0n=_0k[b.t]local _0o=_0k[b.h]local _0p=_0k[b.a]local _0q=-b.a;local _0r={}local _0d=_0k[b.d]local _0s=_0k[b.u]local function _0t(_0u)return _0u[b.v]and _0u[b.t]or _0d[_0u[b.f]]end;local function _0v(_0u)return _0u[b.i]and _0u[b.db]or _0d[_0u[b.h]]end;while true do local _0u=_0n[_0s]local _X=_0u[b.o]_0s=_0s+b.a;if _X==b.c then _0d[_0u[b.u]]=_0t(_0u)+_0v(_0u)elseif _X==b.a then local _0w=_0u[b.u]local _0x=_0u[b.f]local _L;if _0x==b.c then _L=_0q-_0w+b.a else _L=_0x-b.a end;_09(_0r,b.c)return A(_0d,_0w,_0w+_L-b.a)elseif _X==b.d then _0d[_0u[b.u]]={}elseif _X==b.t then local _0w=_0u[b.u]local _0x=_0u[b.f]local _0y=_0u[b.h]local _0z;if _0x==b.c then _0z=_0q-_0w else _0z=_0x-b.a end;local _0A=B(_0d[_0w](A(_0d,_0w+b.a,_0w+_0z)))local _0B=_0A.n;if _0y==b.c then _0q=_0w+_0B-b.a else _0B=_0y-b.a end;C(_0A,b.a,_0B,_0w,_0d)elseif _X==b.h then _0d[_0u[b.u]]=#_0d[_0u[b.f]]elseif _X==b.u then local _0w=_0u[b.u]local _0y=_0u[b.h]local _L=_0u[b.f]local _0C=_0d[_0w]local G;if _L==b.c then _L=_0q-_0w end;if _0y==b.c then _0y=_0u[_0s][b.d]_0s=_0s+b.a end;G=(_0y-b.a)*b.eb;C(_0d,_0w+b.a,_0w+_L,G+b.a,_0C)elseif _X==b.cb then _0s=_0s+_0u[b.f]elseif _X==b.f then _0d[_0u[b.u]]=_0u[b.a]elseif _X==b.o then if _0t(_0u)<=_0v(_0u)==(_0u[b.u]~=b.c)then _0s=_0s+_0n[_0s][b.f]end;_0s=_0s+b.a elseif _X==b.v then _0d[_0u[b.u]]=_0l[_0u[b.a]]elseif _X==b.ab then _0d[_0u[b.u]]=_0d[_0u[b.f]][_0v(_0u)]end;_0k[b.u]=_0s end end;function _j(_07,_0l,_0D)_0l=_0l or M(b.c)local function _0E(...)local _0F=B(...)local _0d=z()local _0p={b.c,{}}C(_0F,b.a,_07[b.d],b.c,_0d)if _07[b.d]<_0F.n then local W=_07[b.d]+b.a;local _L=_0F.n-_07[b.d]_0p[b.a]=_L;C(_0F,W,W+_L-b.a,b.a,_0p[b.d])end;local _0k={_0p,_0d,_07[b.cb],_07[b.bb],b.a}local _0G=B(P(_0j,_0k,_0l,_0D))if _0G[b.a]then return A(_0G,b.d,_0G.n)else local _0g={_0k[b.u],_07[b.y]}_0f(_0g,_0G[b.d])return end end;return _0E end;local _0H=e(A(I(S(b.j,b.fb),S(b.gb,b.hb))))local function _0I(_0J)local _p,k=b.c,h(_0J)for q=b.a,#k do _p=_p+j(_0H,g(k,q,q))*b.ib^(q-b.a)end;return _p end;local function _0K(_0L)local _0M,_0N,_0O,_0P,_04={},b.e,c.a,e(_0L[b.a])local _0G={_0P}for q=b.c,b.w do _0M[q]=e(q)end;for q=b.d,#_0L do _04=_0L[q]if _0M[_04]then _0O=_0M[_04]elseif _04==_0N then _0O=_0P..g(_0P,b.a,b.a)else return nil,q end;y(_0G,_0O)_0M[_0N]=_0P..g(_0O,b.a,b.a)_0N=_0N+b.a;_0P=_0O end;return x(_0G)end;local function _0Q(_0R)local _0S={}local q=b.a;while q<=#_0R do local _L=_0I(g(_0R,q,q))q=q+b.a;y(_0S,_0I(g(_0R,q,q+_L-b.a)))q=q+_L end;return _0K(_0S)end;_j(_i(_0Q(c.c)))()end;a({a=1,b=35,c=0,d=2,e=256,f=7,g=127,h=4,i=15,j=48,k=40,l=32,m=24,n=16,o=8,p=2047,q=1023,r=52,s=63,t=3,u=5,v=9,w=255,x=131071,y=11,z=12,ab=10,bb=14,cb=6,db=13,eb=50,fb=57,gb=65,hb=90,ib=36},{a=[[]],b=[[:]],c=[[1B102752761021S23822T23123421E21A23023922P27614277276121627L27522022T2302302331027N27P1023B23323623022S27Y1026O21R27X27P23423622X232238285101E27Y2102751227623E101127728O1228Q27522C28L28P1028O1128M27521W27W27723R27528Q28M23027527K28M24A27W1327624L1029H28Y21G27W28M28Q1728P2991129B1026V27328Q1K298276]]})