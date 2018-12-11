'serial size boxsize' =: ". }: stdin''

coords =: (,"0)/~ >: i. size
calc =: 5 -~ _3 { 0 0, 10 #.^:_1 (10+[) * serial + ] * 10 + [
fill =: ((boxsize-1)$0),"1~ ]
powers =: calc/"1 coords
sums =: fill&.|: fill (2$boxsize) ([: +/ +/);._3 powers
max =: >./ >./ sums
max_position =: ((,/sums) i. max) { (,/ coords)

smoutput max,max_position,boxsize
