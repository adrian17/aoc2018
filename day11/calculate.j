'serial size boxsize' =: ". }: stdin''

coords =: ,"0/~ @: >: @: i.
calc =: 5 -~ _3 { 0 0, 10 #.^:_1 (10+[) * serial + ] * 10 + [
powers =: calc/"1 coords size
sums =: (2$boxsize) ([: +/ +/);._3 powers
max =: >./ >./ sums
max_position =: ((,/sums) i. max) { (,/ coords # sums)

smoutput max,max_position,boxsize
