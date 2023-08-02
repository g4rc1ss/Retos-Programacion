﻿Día 19: Escáner de balizas 

A medida que su sonda descendía por esta área, lanzó una variedad de balizas y escáneres en el agua. Es difícil navegar en las aguas abiertas de la fosa oceánica, pero si puede construir un mapa de la fosa utilizando los datos de los escáneres, debería poder llegar al fondo de manera segura.

Las balizas y los escáneres flotan inmóviles en el agua; están diseñados para mantener la misma posición durante largos períodos de tiempo. Cada escáner es capaz de detectar todas las balizas en un cubo grande centrado en el escáner; las balizas que están como máximo a 1000 unidades del escáner en cada uno de los tres ejes ( x, y, y z) tienen su posición precisa determinada en relación con el escáner. Sin embargo, los escáneres no pueden detectar otros escáneres. El submarino ha resumido automáticamente las posiciones relativas de las balizas detectadas por cada escáner (su entrada de rompecabezas).

Por ejemplo, si un escáner está en las x,y,zcoordenadas 500,0,-500y hay balizas en -500,1000,-1500y 1501,0,-500, el escáner podría informar que la primera baliza está en -1000,1000,-1000(en relación con el escáner) pero no detectaría la segunda baliza en absoluto.

Desafortunadamente, aunque cada escáner puede informar las posiciones de todas las balizas detectadas en relación con él mismo, los escáneres no conocen su propia posición . Deberá determinar las posiciones de las balizas y los escáneres usted mismo.

Los escáneres y las balizas mapean una única región 3D contigua. Esta región se puede reconstruir encontrando pares de escáneres que tengan regiones de detección superpuestas, de modo que haya al menos 12 balizas que ambos escáneres detecten dentro de la superposición. Al establecer 12 balizas comunes, puede determinar con precisión dónde están los escáneres entre sí, lo que le permite reconstruir el mapa de balizas un escáner a la vez.

Por un momento, considere sólo dos dimensiones. Suponga que tiene los siguientes informes de escáner:
```
--- scanner 0 ---
0,2
4,1
3,3

--- scanner 1 ---
-1,-1
-5,0
-2,1
```

Dibujo xcreciente hacia la derecha, ycreciente hacia arriba, escáneres como Sy balizas como B, el escáner 0detecta esto:

```
...B.
B....
....B
S....
```
El escáner 1detecta esto:

```
...B..
B....S
....B.
```

Para este ejemplo, suponga que los escáneres solo necesitan 3 balizas superpuestas. Luego, las balizas visibles para ambos escáneres se superponen para producir el siguiente mapa completo:

```
...B..
B....S
....B.
S.....
```
Desafortunadamente, hay un segundo problema: los escáneres tampoco conocen su rotación o dirección de orientación . Debido a la alineación magnética, cada escáner se gira un número entero de giros de 90 grados alrededor de todos los x, yy zejes. Es decir, un escáner podría llamar a una dirección positiva x, mientras que otro escáner podría llamar a esa dirección negativa y. O bien, dos escáneres pueden estar de acuerdo en qué dirección es positiva x, pero un escáner puede estar al revés desde la perspectiva del otro escáner. En total, cada escáner podría estar en cualquiera de las 24 orientaciones diferentes: frente positiva o negativa x, yo z, y teniendo en cuenta cualquiera de las cuatro direcciones "arriba" de esa orientación.

Por ejemplo, aquí hay una disposición de balizas vistas desde un escáner en la misma posición pero en diferentes orientaciones:
```
--- scanner 0 ---
-1,-1,1
-2,-2,2
-3,-3,3
-2,-3,1
5,6,-4
8,0,7

--- scanner 0 ---
1,-1,1
2,-2,2
3,-3,3
2,-1,3
-5,4,-6
-8,-7,0

--- scanner 0 ---
-1,-1,-1
-2,-2,-2
-3,-3,-3
-1,-3,-2
4,6,5
-7,0,8

--- scanner 0 ---
1,1,-1
2,2,-2
3,3,-3
1,3,-2
-4,-6,5
7,0,8

--- scanner 0 ---
1,1,1
2,2,2
3,3,3
3,1,2
-6,-4,-5
0,7,-8
```
Al encontrar pares de escáneres que vean al menos 12 de las mismas balizas, puede ensamblar el mapa completo. Por ejemplo, considere el siguiente informe:

```
--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14
```
Debido a que todas las coordenadas son relativas, en este ejemplo, todas las posiciones "absolutas" se expresarán en relación con el escáner 0(usando la orientación del escáner 0y como si el escáner estuviera 0en las coordenadas 0,0,0).

Los escáneres 0y 1tienen cubos de detección superpuestos; las 12 balizas que ambos detectan (en relación con el escáner 0) están en las siguientes coordenadas:
```
-618,-824,-621
-537,-823,-458
-447,-329,318
404,-588,-901
544,-627,-890
528,-643,409
-661,-816,-575
390,-675,-793
423,-701,434
-345,-311,381
459,-707,401
-485,-357,347
```
Estas mismas 12 balizas (en el mismo orden) pero desde la perspectiva del escáner 1son:
```
686,422,578
605,423,415
515,917,-361
-336,658,858
-476,619,847
-460,603,-452
729,430,532
-322,571,750
-355,545,-477
413,935,-424
-391,539,-444
553,889,-390
```
Debido a esto, el escáner 1debe estar en 68,-1246,-43(relativo al escáner 0).

El escáner se 4superpone con el escáner 1; las 12 balizas que ambos detectan (en relación con el escáner 0) son:
```
459,-707,401
-739,-1745,668
-485,-357,347
432,-2009,850
528,-643,409
423,-701,434
-345,-311,381
408,-1815,803
534,-1912,768
-687,-1600,576
-447,-329,318
-635,-1737,486
```
Entonces, el escáner 4está en -20,-1133,1061(relativo al escáner 0).

Siguiendo este proceso, el escáner 2debe estar en 1105,-1205,1229(relativo al escáner 0) y el escáner 3debe estar en -92,-2380,-20(relativo al escáner 0).

La lista completa de balizas (en relación con el escáner 0) es:
```
-892,524,684
-876,649,763
-838,591,734
-789,900,-551
-739,-1745,668
-706,-3180,-659
-697,-3072,-689
-689,845,-530
-687,-1600,576
-661,-816,-575
-654,-3158,-753
-635,-1737,486
-631,-672,1502
-624,-1620,1868
-620,-3212,371
-618,-824,-621
-612,-1695,1788
-601,-1648,-643
-584,868,-557
-537,-823,-458
-532,-1715,1894
-518,-1681,-600
-499,-1607,-770
-485,-357,347
-470,-3283,303
-456,-621,1527
-447,-329,318
-430,-3130,366
-413,-627,1469
-345,-311,381
-36,-1284,1171
-27,-1108,-65
7,-33,-71
12,-2351,-103
26,-1119,1091
346,-2985,342
366,-3059,397
377,-2827,367
390,-675,-793
396,-1931,-563
404,-588,-901
408,-1815,803
423,-701,434
432,-2009,850
443,580,662
455,729,728
456,-540,1869
459,-707,401
465,-695,1988
474,580,667
496,-1584,1900
497,-1838,-617
527,-524,1933
528,-643,409
534,-1912,768
544,-627,-890
553,345,-567
564,392,-477
568,-2007,-577
605,-1665,1952
612,-1593,1893
630,319,-379
686,-3108,-505
776,-3184,-501
846,-3110,-434
1135,-1161,1235
1243,-1093,1063
1660,-552,429
1693,-557,386
1735,-437,1738
1749,-1800,1813
1772,-405,1572
1776,-675,371
1779,-442,1789
1780,-1548,337
1786,-1538,337
1847,-1591,415
1889,-1729,1762
1994,-1805,1792
```
En total, hay 79balizas.

Reúna el mapa completo de balizas. ¿Cuántas balizas hay?