import sys
from sys import argv
import json
from omg import *
from omg.mapedit import *
from omg.txdef import *
import os

gl_seg_offset = 15
args = argv[1:]

global inwad
inwad = None

def getPointFromGLSeg (vertexId, glv, v):
    shifted = 1 << gl_seg_offset
    isGL = (vertexId & shifted) == shifted
    if isGL:
        vId = vertexId - shifted
        return glv[vId]
    else:
        return v[vertexId]

inwad = WAD()
inwad.from_file(args[0])

glmap = inwad.glmaps["GL_E1M1"]
map = inwad.maps["E1M1"]

editor = MapEditor(map)
editor.load_gl(glmap)

sectors = []

for ss in editor.gl_ssect:
    sect = {}
    sga = []
    segs = []
    for i in range(ss.seg_a, ss.seg_a + ss.numsegs):
        sga.append(editor.gl_segs[i])
    for sg in sga:
        v1 = getPointFromGLSeg(sg.vx_a, editor.gl_vert, editor.vertexes)
        v2 = getPointFromGLSeg(sg.vx_b, editor.gl_vert, editor.vertexes)
        segs.append({
            'p1': [v1.x, v1.y],
            'p2': [v2.x, v2.y],
            'sct': -1,
            'sid': -1
        })
    sd = editor.sidedefs[sga[0].side]
    sct = editor.sectors[sd.sector]
    sect['segs'] = segs
    sect['floor'] = sct.z_floor
    sect['ceil'] = sct.z_ceil
    sectors.append(sect)

# Now we need to see where they back onto each other to merge them with portals
for si1 in range(0, len(sectors)):
    s1 = sectors[si1]
    for sgi1 in range(0, len(s1['segs'])):
        sg1 = s1['segs'][sgi1]
        for si2 in range(0, len(sectors)):
            s2 = sectors[si2]
            for sgi2 in range(0, len(s2['segs'])):
                sg2 = s2['segs'][sgi2]
                if sg1['p1'] == sg2['p2'] and sg1['p2'] == sg2['p1']:
                    sectors[si1]['segs'][sgi1]['sct'] = si2
                    sectors[si1]['segs'][sgi1]['sid'] = sgi2
                    sectors[si2]['segs'][sgi2]['sct'] = si1
                    sectors[si2]['segs'][sgi2]['sid'] = sgi1

for s in sectors:
    print("Sector {")
    print("sides: vec![")
    for sg in s['segs']:
        print("Side {")
        print("p1: Vector2f::new(%d, %d)," % (float(sg['p1'][0]) / 100., float(sg['p1'][1]) / 100.))
        print("p2: Vector2f::new(%d, %d)," % (float(sg['p2'][0]) / 100., float(sg['p2'][1]) / 100.))
        print("neighbour_sect: %d," % sg['sct'])
        print("neighbour_side: %d" % sg['sid'])
        print("},")
    print("],")
    print("ceil_height: %d," % s['ceil'])
    print("floor_height: %d" % s['floor'])
    print("},")