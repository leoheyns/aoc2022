
print(sum({'A X':3,'A Y':4,'A Z':8,'B X':1,'B Y':5,'B Z':9,'C X':2,'C Y':6,'C Z':7}[l]for l in open("input").read().split("\n")))
print(sum({'AX':3,'AY':4,'AZ':8,'BX':1,'BY':5,'BZ':9,'CX':2,'CY':6,'CZ':7}[l[::2]]for l in open("input").readlines()))
print(sum('  BXCXAXAYBYCYCZAZBZ'.index(l[::2])for l in open("input"))//2)