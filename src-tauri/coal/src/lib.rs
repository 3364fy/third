pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub mod coal {
    use std::str;


    pub fn write_inp(parameter:&Vec<Vec<f64>>,length:&f64,gaplength:&f64,sigv: &f64,sigh: &f64,sig_h: &f64,tempini: &f64,tempgas: &f64,tempcol:&f64,depthcen:&f64,gaspres:&f64,gastime:&f64) -> String {
        let str1=r#"# -*- coding: mbcs -*-
# Abaqus/CAE Release 2022 replay file
# Internal Version: 2021_09_16-01.57.30 176069
# Run by loneve on Wed Apr 24 10:58:42 2024
#

# from driverUtils import executeOnCaeGraphicsStartup
# executeOnCaeGraphicsStartup()
#: Executing "onCaeGraphicsStartup()" in the site directory ...
import math
from abaqus import *
from abaqusConstants import *
# def isclose(a, b,rel_tol=1e-10):
#     return abs(a-b) <= rel_tol * max(abs(a), abs(b))
def isclose(a, b,rel_tol=1e-10):
    return abs(a-b) <= rel_tol
def circle_from_points(point1, point2, point3):
    x1, y1 = point1
    x2, y2 = point2
    x3, y3 = point3

    D = 2 * ( x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2) )

    Ux = ((x1 * x1 + y1 * y1) * (y2 - y3) + (x2 * x2 + y2 * y2) * (y3 - y1) + (x3 * x3 + y3 * y3) * (y1 - y2)) / D
    Uy = ((x1 * x1 + y1 * y1) * (x3 - x2) + (x2 * x2 + y2 * y2) * (x1 - x3) + (x3 * x3 + y3 * y3) * (x2 - x1)) / D

    radius = math.sqrt((x1 - Ux)**2 + (y1 - Uy)**2)

    return (Ux, Uy), radius"#;
            let str2=format!("\nparameter={:?}",parameter);
            let str3=format!("\nlength={}",length);
            let str4=format!("\ngaplength={}",gaplength);
            let str5=r#"
thickness=[] 
for i in parameter:
    thickness.append(i[1])
    if i[0]==1:
        coallayer=parameter.index(i)+1
        print("coallayer:{}".format(coallayer))
# coallayer=6 
height=sum(thickness) 
heightlist=[]
for i in range(len(thickness)):
    heightlist.append(sum(thickness[:i+1]))
print(heightlist)
heightcap=sum(thickness[:coallayer-1])+thickness[coallayer-1]/2
heightund=sum(thickness[coallayer:])+thickness[coallayer-1]/2

session.Viewport(name='Viewport: 1', origin=(0.0, 0.0), width=187.67707824707, 
    height=111.819450378418)
session.viewports['Viewport: 1'].makeCurrent()
session.viewports['Viewport: 1'].maximize()
from caeModules import *
from driverUtils import executeOnCaeStartup
executeOnCaeStartup()
#: Executing "onCaeStartup()" in the site directory ...
session.viewports['Viewport: 1'].partDisplay.geometryOptions.setValues(
    referenceRepresentation=ON)
cliCommand("""session.journalOptions.setValues(replayGeometry=COORDINATE,recoverGeometry= COORDINATE)""")
s = mdb.models['Model-1'].ConstrainedSketch(name='__profile__', 
    sheetSize=200.0)
g, v, d, c = s.geometry, s.vertices, s.dimensions, s.constraints
s.setPrimaryObject(option=STANDALONE)
s.rectangle(point1=(0.0, 0.0), point2=(length, height))
p = mdb.models['Model-1'].Part(name='Part-1', dimensionality=TWO_D_PLANAR, 
    type=DEFORMABLE_BODY)
p = mdb.models['Model-1'].parts['Part-1']
p.BaseShell(sketch=s)
s.unsetPrimaryObject()
p = mdb.models['Model-1'].parts['Part-1']
session.viewports['Viewport: 1'].setValues(displayedObject=p)
del mdb.models['Model-1'].sketches['__profile__']
p = mdb.models['Model-1'].parts['Part-1']
f, e, d1 = p.faces, p.edges, p.datums
s1 = mdb.models['Model-1'].ConstrainedSketch(name='__profile__', 
    sheetSize=116.61, gridSpacing=2.91)
g, v, d, c = s1.geometry, s1.vertices, s1.dimensions, s1.constraints
s1.setPrimaryObject(option=SUPERIMPOSE)
p = mdb.models['Model-1'].parts['Part-1']
p.projectReferencesOntoSketch(sketch=s1, filter=COPLANAR_EDGES)

for i,thick in enumerate(thickness):
    if i==0:
        print("****************************************88")
        print(0,thick)
        s1.Line(point1=(0, thick), point2=(length, thick))
    else:
        s1.Line(point1=(0, sum(thickness[:i+1])), point2=(length, sum(thickness[:i+1])))
        print(0,sum(thickness[:i+1]))
s1.Line(point1=(gaplength, height), point2=(gaplength, 0))
SecondLineOffset=2.91
s1.Line(point1=(gaplength+SecondLineOffset, height), point2=(gaplength+SecondLineOffset, 0))
ThirdPointOffset=0.7275
s1.Arc3Points(point1=(gaplength, sum(thickness[:coallayer])), point2=(gaplength, sum(thickness[:coallayer-1])), point3=(gaplength+ThirdPointOffset, 
    heightlist[coallayer-2]+(heightlist[coallayer-1]-heightlist[coallayer-2])/2))
p = mdb.models['Model-1'].parts['Part-1']
f = p.faces
pickedFaces = f.findAt(((0, 0, 0.0), ))
e1, d2 = p.edges, p.datums
p.PartitionFaceBySketch(faces=pickedFaces, sketch=s1)
s1.unsetPrimaryObject()
del mdb.models['Model-1'].sketches['__profile__']
p = mdb.models['Model-1'].parts['Part-1']
f1 = p.faces
p.RemoveFaces(faceList=(f1.findAt(coordinates=(gaplength/2, sum(thickness[:coallayer-1])+thickness[coallayer-1]/2, 0.0)),
                        f1.findAt(coordinates=(gaplength+ThirdPointOffset/2, sum(thickness[:coallayer-1])+thickness[coallayer-1]/2, 0.0))
                        ), 
                deleteCells=False)

session.viewports['Viewport: 1'].partDisplay.setValues(mesh=ON)


p = mdb.models['Model-1'].parts['Part-1']
f = p.faces
elemType1 = mesh.ElemType(elemCode=CPE8RT, elemLibrary=STANDARD)
elemType2 = mesh.ElemType(elemCode=CPE6MT, elemLibrary=STANDARD)
set0=()
for i in f:
    set0=set0+(i.pointOn[0],)
faces = f.findAt(*[(coord,) for coord in set0])
pickedRegions =(faces, )
p.setElementType(regions=pickedRegions, elemTypes=(elemType1, elemType2))


p = mdb.models['Model-1'].parts['Part-1']
p.seedPart(size=2.0, deviationFactor=0.1, minSizeFactor=0.1)
p = mdb.models['Model-1'].parts['Part-1']
f = p.faces

layers=len(thickness)

p = mdb.models['Model-1'].parts['Part-1']
f = p.faces
print(len(e))
pickedRegions = f[0:len(f)]



if coallayer-3>=0 and layers>=coallayer+2:
    y1bottom=0
    y1top=sum(thickness[:coallayer-4])
    xleft=0
    xright=length
    y2bottom=sum(thickness[:coallayer+3])
    y2top=height
    set1=()
    print("=====================stage1============================")
    if coallayer-4>0:
        for i in f:
            if xleft<=i.pointOn[0][0]<=xright and y1bottom<=i.pointOn[0][1]<=y1top:
                print(i.pointOn[0])
                print(type(i.pointOn[0]))
                set1=set1+(i.pointOn[0],)
        pickedRegions = f.findAt(*[(coord,) for coord in set1])
        p.generateMesh(regions=pickedRegions)
    if coallayer+3<layers:
        for i in f:
            if xleft<=i.pointOn[0][0]<=xright and y2bottom<=i.pointOn[0][1]<=y2top:
                print(i.pointOn[0])
                print(type(i.pointOn[0]))
                set1=set1+(i.pointOn[0],)
        pickedRegions = f.findAt(*[(coord,) for coord in set1])
        p.generateMesh(regions=pickedRegions)


    p = mdb.models['Model-1'].parts['Part-1']
    e = p.edges
    ybottom=sum(thickness[:coallayer-3])
    ytop=sum(thickness[:coallayer+2])
    xleft=0
    xright=gaplength+SecondLineOffset
    set2=()
    print("=====================stage2============================")
    for i in e:
        if xleft<=i.pointOn[0][0]<=xright and ybottom<=i.pointOn[0][1]<=ytop:
            print(i.pointOn[0])
            print(type(i.pointOn[0]))
            set2=set2+(i.pointOn[0],)
    print(set2)
    pickedEdges = e.findAt(*[(coord,) for coord in set2])
    p.seedEdgeBySize(edges=pickedEdges, size=0.2, deviationFactor=0.1, 
    minSizeFactor=0.1, constraint=FINER)

    p = mdb.models['Model-1'].parts['Part-1']
    e = p.edges
    y1bottom=sum(thickness[:coallayer-4])
    y1top=sum(thickness[:coallayer-3])
    xleft=0
    xright=gaplength+SecondLineOffset
    y2bottom=sum(thickness[:coallayer+2])
    y2top=sum(thickness[:coallayer+3])
    set3=()
    print("=====================stage3============================")
    if coallayer-4>=0:
        for i in e:
            if xleft<=i.pointOn[0][0]<=xright and y1bottom<=i.pointOn[0][1]<=y1top and (i.pointOn[0][0]==0 or i.pointOn[0][0]==gaplength+SecondLineOffset or i.pointOn[0][0]==gaplength):
                print(i.pointOn[0])
                print(type(i.pointOn[0]))
                set3=set3+(i.pointOn[0],)
    set4=()
    if coallayer+3<=layers:
        for i in e:
            if xleft<=i.pointOn[0][0]<=xright and y2bottom<=i.pointOn[0][1]<=y2top and (i.pointOn[0][0]==0 or i.pointOn[0][0]==gaplength+SecondLineOffset or i.pointOn[0][0]==gaplength):
                print(i.pointOn[0])
                print(type(i.pointOn[0]))
                set4=set4+(i.pointOn[0],)
    pickedEdge = e.findAt(*[(coord,) for coord in set3])
    p.seedEdgeByBias(biasMethod=SINGLE, end1Edges=pickedEdge, 
        minSize=0.2, maxSize=1.0, constraint=FINER)
    pickedEdge = e.findAt(*[(coord,) for coord in set4])
    p.seedEdgeByBias(biasMethod=SINGLE, end2Edges=pickedEdge, 
        minSize=0.2, maxSize=1.0, constraint=FINER)  
    

    p = mdb.models['Model-1'].parts['Part-1']
    e = p.edges
    xleft=gaplength+SecondLineOffset
    set4=()
    print("=====================stage4============================")
    coallayer_2=sum(thickness[:coallayer-2])
    coallayer_1=sum(thickness[:coallayer-1])
    coallayer0=sum(thickness[:coallayer])
    coallayer1=sum(thickness[:coallayer+1])
    print(coallayer_2,coallayer_1,coallayer0,coallayer1)
    list=[coallayer_2,coallayer_1,coallayer0,coallayer1]
    # print(e)
    for i in e:
        # print(coallayer_2)
        # print(type(i.pointOn[0][1]))
        # print(round(i.pointOn[0][1], 3)-coallayer_2)
        # # print(1e-20<i.pointOn[0][1]-coallayer_2<1e-13)
        # print(isclose(i.pointOn[0][1], coallayer_2))
        # if math.isclose(i.pointOn[0][1], coallayer_2, rel_tol=1e-9):
        #     print(666666)
        if xleft<=i.pointOn[0][0] and (isclose(i.pointOn[0][1], coallayer_2) or isclose(i.pointOn[0][1], coallayer_1) or isclose(i.pointOn[0][1], coallayer0) or isclose(i.pointOn[0][1], coallayer1)):
            print(i.pointOn[0])
            print(type(i.pointOn[0]))
            set4=set4+(i.pointOn[0],)
    print(set4)
    pickedEdges = e.findAt(*[(coord,) for coord in set4])
    p.seedEdgeByBias(biasMethod=SINGLE, end1Edges=pickedEdges, minSize=0.2, 
    maxSize=1.0, constraint=FINER)

    p = mdb.models['Model-1'].parts['Part-1']
    e = p.edges
    set5=()
    print("=====================stage5============================")
    for i in e:
        if i.pointOn[0][0]==length and (sum(thickness[:coallayer-2]) <=i.pointOn[0][1]<=sum(thickness[:coallayer-1]) or sum(thickness[:coallayer])<i.pointOn[0][1]<sum(thickness[:coallayer+1])):
            print(i.pointOn[0])
            set5=set5+(i.pointOn[0],)
    pickedEdges = e.findAt(*[(coord,) for coord in set5])
    p.seedEdgeBySize(edges=pickedEdges, size=1, deviationFactor=0.1, 
    minSizeFactor=0.1, constraint=FINER)


    p = mdb.models['Model-1'].parts['Part-1']
    e = p.edges
    set6=()
    print("=====================stage6============================")
    for i in e:
        if i.pointOn[0][0]==length and ( 
            sum(thickness[:coallayer-1]) <i.pointOn[0][1]<sum(thickness[:coallayer]) or
            sum(thickness[:coallayer-3]) <i.pointOn[0][1]<sum(thickness[:coallayer-2]) or
            sum(thickness[:coallayer+1]) <i.pointOn[0][1]<sum(thickness[:coallayer+2])
            ):
            print(i.pointOn[0])
            set6=set6+(i.pointOn[0],)
    if coallayer-4>=0:
        for i in e:
            if i.pointOn[0][0]==length and ( 
                sum(thickness[:coallayer-4]) <i.pointOn[0][1]<sum(thickness[:coallayer-3])
                ):
                print(i.pointOn[0])
                set6=set6+(i.pointOn[0],)
    if coallayer+3<=layers:
        for i in e:
            if i.pointOn[0][0]==length and ( 
                sum(thickness[:coallayer+2]) <i.pointOn[0][1]<sum(thickness[:coallayer+3])
                ):
                print(i.pointOn[0])
                set6=set6+(i.pointOn[0],)
    pickedEdges = e.findAt(*[(coord,) for coord in set6])
    p.seedEdgeBySize(edges=pickedEdges, size=1, deviationFactor=0.1, 
    minSizeFactor=0.1, constraint=FINER)


    p = mdb.models['Model-1'].parts['Part-1']
    f = p.faces
    set7=()
    print("=====================stage7============================")
    if coallayer-4>=0:
        ybottom=sum(thickness[:coallayer-4])
    elif coallayer-3>=0:
        ybottom=sum(thickness[:coallayer-3])

    if coallayer+3<=layers:
        ytop=sum(thickness[:coallayer+3])
    elif coallayer+2<=layers:
        ytop=sum(thickness[:coallayer+2])
    xleft=0
    xright=length
    for i in f:
        if xleft<=i.pointOn[0][0]<=xright and ybottom<=i.pointOn[0][1]<=ytop :
            print(i.pointOn[0])
            print(type(i.pointOn[0]))
            set7=set7+(i.pointOn[0],)
    print(set7)
    pickedRegions = f.findAt(*[(coord,) for coord in set7])
    p.generateMesh(regions=pickedRegions)

p = mdb.models['Model-1'].parts['Part-1']
f = p.faces
heightlist_copy=heightlist[:]
heightlist_copy.insert(0,0)
for index,_ in enumerate(thickness):
    set=()
    for i in f:
        if 0<=i.pointOn[0][0]<=length and heightlist_copy[index] <=i.pointOn[0][1]<=heightlist_copy[index+1] :
            # print(i.pointOn[0])
            # print(type(i.pointOn[0]))
            set=set+(i.pointOn[0],)
    faces = f.findAt(*[(coord,) for coord in set])
    p.Set(faces=faces, name='CAP{}'.format(index+1))
mdb.models['Model-1'].parts['Part-1'].sets.changeKey(fromName='CAP{}'.format(coallayer), 
    toName='COAL')
    
p = mdb.models['Model-1'].parts['Part-1']
n = p.nodes
selected_nodes = []
points = [(gaplength, sum(thickness[:coallayer])), (gaplength, sum(thickness[:coallayer-1])), (gaplength+ThirdPointOffset, 
    heightlist[coallayer-2]+(heightlist[coallayer-1]-heightlist[coallayer-2])/2)]
circle_center, circle_radius = circle_from_points(*points)
print("==============================")
print(circle_center, circle_radius)
print(heightlist)
for i in n:
    if gaplength<=i.coordinates[0]<=gaplength+ThirdPointOffset and heightlist[coallayer-2]<=i.coordinates[1]<=heightlist[coallayer-1]:
        # print(i.coordinates)
        distance = math.sqrt((i.coordinates[0] - circle_center[0])**2 + (i.coordinates[1] - circle_center[1])**2)
        # print(distance)
        print(isclose(distance, circle_radius,0.001))
        if isclose(distance, circle_radius,0.001):
            print("**********************************")
            print(distance)
            print(i.label)
            selected_nodes.append(i.label)
    if (0<=i.coordinates[0]<=gaplength) and (isclose(i.coordinates[1],heightlist[coallayer-1],0.001) or isclose(i.coordinates[1],heightlist[coallayer-2],0.001)):
        selected_nodes.append(i.label)
    # print(i)
print(selected_nodes)
print(len(selected_nodes))
node=n[selected_nodes[0]-1:selected_nodes[0]]
del selected_nodes[0]
print(selected_nodes)
for i in selected_nodes:
    node+=n[i-1:i]
print(node,1)
p.Set(nodes=node, name='INNER')
print(heightlist)

p = mdb.models['Model-1'].parts['Part-1']
n = p.nodes
selected_nodes = []
for i in n:
    if i.coordinates[0]==0:
        selected_nodes.append(i.label)
node=n[selected_nodes[0]-1:selected_nodes[0]]
del selected_nodes[0]
for i in selected_nodes:
    node+=n[i-1:i]
p.Set(nodes=node, name='XSYM')


p = mdb.models['Model-1'].parts['Part-1']
n = p.nodes
selected_nodes = []
for i in n:
    if i.coordinates[0]==length:
        selected_nodes.append(i.label)
node=n[selected_nodes[0]-1:selected_nodes[0]]
del selected_nodes[0]
for i in selected_nodes:
    node+=n[i-1:i]
p.Set(nodes=node, name='XFIXED')

p = mdb.models['Model-1'].parts['Part-1']
n = p.nodes
selected_nodes = []
for i in n:
    if i.coordinates[1]==0:
        selected_nodes.append(i.label)
node=n[selected_nodes[0]-1:selected_nodes[0]]
del selected_nodes[0]
for i in selected_nodes:
    node+=n[i-1:i]
p.Set(nodes=node, name='YFIXED')

p = mdb.models['Model-1'].parts['Part-1']
n = p.nodes
selected_nodes = []
for i in n:
    if i.coordinates[1]==0:
        selected_nodes.append(i.label)
node=n[selected_nodes[0]-1:selected_nodes[0]]
del selected_nodes[0]
for i in selected_nodes:
    node+=n[i-1:i]
p.Set(nodes=node, name='YFIXED')

p = mdb.models['Model-1'].parts['Part-1']
n = p.nodes
print(len(n))
nodes = n[0:len(n)]
p.Set(nodes=nodes, name='NALL')
#: The set 'NALL' has been created (20996 nodes).
p1 = mdb.models['Model-1'].parts['Part-1']
session.viewports['Viewport: 1'].setValues(displayedObject=p1)
p = mdb.models['Model-1'].parts['Part-1']
e = p.elements
print(len(e))
elements = e[0:len(e)]
p.Set(elements=elements, name='EALL')


p = mdb.models['Model-1'].parts['Part-1']
s = p.edges
set8=()
for i in s:
    print(i.pointOn[0])
    distance = math.sqrt((i.pointOn[0][0] - circle_center[0])**2 + (i.pointOn[0][1] - circle_center[1])**2)
    if isclose(distance, circle_radius,0.001): 
        set8=set8+(i.pointOn[0],)       
    
side1Edges = s.findAt(*[(coord,) for coord in set8])
p.Surface(side1Edges=side1Edges, name='SIDE_SURF')


p = mdb.models['Model-1'].parts['Part-1']
s = p.edges
set9=()
for i in s:
    if 0<=i.pointOn[0][0]<=gaplength and \
    (isclose(i.pointOn[0][1],heightlist[coallayer-1],0.001) or isclose(i.pointOn[0][1],heightlist[coallayer-2],0.001)):
        set9=set9+(i.pointOn[0],)
    
side1Edges = s.findAt(*[(coord,) for coord in set9])
p.Surface(side1Edges=side1Edges, name='TOP_BOT_SURF')


p = mdb.models['Model-1'].parts['Part-1']
s = p.edges
set10=()
for i in s:
    if isclose(i.pointOn[0][1],height,1.0e-3):
        set10=set10+(i.pointOn[0],)
side1Edges = s.findAt(*[(coord,) for coord in set10])
p.Surface(side1Edges=side1Edges, name='TOP_SURF')
a = mdb.models['Model-1'].rootAssembly
a.Instance(name='Part-1-1', part=p, dependent=ON)
mdb.Job(name='GEOMODEL', model='Model-1', description='', type=ANALYSIS, 
    atTime=None, waitMinutes=0, waitHours=0, queue=None, memory=90, 
    memoryUnits=PERCENTAGE, getMemoryFromAnalysis=True, 
    explicitPrecision=SINGLE, nodalOutputPrecision=SINGLE, echoPrint=OFF, 
    modelPrint=OFF, contactPrint=OFF, historyPrint=OFF, userSubroutine='', 
    scratch='', resultsFormat=ODB, parallelizationMethodExplicit=DOMAIN, 
    numDomains=1, activateLoadBalancing=False)
mdb.jobs['GEOMODEL'].writeInput(consistencyChecking=OFF)

mdb.saveAs(pathName='GEOMODEL.cae')

session.viewports['Viewport: 1'].viewportAnnotationOptions.setValues(triad=OFF, 
    legend=OFF, title=OFF, state=OFF, annotations=OFF, compass=OFF)
session.viewports['Viewport: 1'].enableMultipleColors()
session.viewports['Viewport: 1'].setColor(initialColor='#BDBDBD')
cmap = session.viewports['Viewport: 1'].colorMappings['Set']
cmap.updateOverrides(overrides={'EALL':(False, ), 'NALL':(False, )})
session.viewports['Viewport: 1'].setColor(colorMapping=cmap)
session.viewports['Viewport: 1'].disableMultipleColors()
session.viewports['Viewport: 1'].enableMultipleColors()
session.viewports['Viewport: 1'].setColor(initialColor='#BDBDBD')
cmap = session.viewports['Viewport: 1'].colorMappings['Set']
session.viewports['Viewport: 1'].setColor(colorMapping=cmap)
session.viewports['Viewport: 1'].disableMultipleColors()
session.printOptions.setValues(vpDecorations=OFF)
session.printToFile(fileName='1', format=PNG, canvasObjects=(
    session.viewports['Viewport: 1'], ))

with open('test.inp', 'w') as f:
    text1='*Heading\n'
    text2='*INCLUDE, INP=GEOMODEL.INP\n'
    f.write(text1+text2)

with open('test.inp', 'a') as f:
    text='''*PARAMETER
**INITIAL IN-SITU STRESS"#;
    let str6=format!("\n    SIGV_EFF     = {}e4",sigv);
    let str7=format!("\n    SIGh_EFF     = {}e4",sigh);
    let str8=format!("\n    SIGH_EFF     = {}e4",sig_h);
    let str9=format!("\n    DEPTH_CEN     = {}",depthcen);
    let str10=r#"
    THICK_CAP    = {}
    THICK_UND    = {}
    DEPTH_TOP    = DEPTH_CEN - THICK_CAP
    DEPTH_BOT    = DEPTH_CEN + THICK_UND
    SIGV         = DEPTH_CEN*SIGV_EFF
    SIGV_TOP     = DEPTH_TOP*SIGV_EFF
    SIGV_BOT     = DEPTH_BOT*SIGV_EFF
    SIGH         = DEPTH_CEN*SIGH_EFF
    SIGh         = DEPTH_CEN*SIGh_EFF
    SIGH_COE     = SIGH_EFF/SIGV_EFF
    SIGh_COE     = SIGh_EFF/SIGV_EFF
**GASIFICATION PRESSURE"#;
    let str11=format!("\n    GAS_PRES   = DEPTH_CEN*{}",gaspres);
    let str12=r#"
**TEMPERATURE"#;
    let str13=format!("\n    TEMP_INI   = {}",tempini);
    let str14=format!("\n    TEMP_GAS   = {}",tempgas);
    let str15=format!("\n    TEMP_COL   = {}",tempcol);
    let str16=r#"
'''.format(heightcap,heightund)
    f.write(text)
with open('test.inp', 'a') as f:
    for i in range(0,layers):
        if i!=coallayer-1:
            text='*SOLID SECTION, ELSET=CAP{}, MATERIAL=CAP{}\n1.,\n'.format(i+1,i+1)
            f.write(text)
        else:
            text='*SOLID SECTION, ELSET=COAL, MATERIAL=COAL\n1.,\n'
            f.write(text)

with open('test.inp', 'a') as f:
    for index ,i in enumerate(parameter):
        if i[0]==1:
            text1='*MATERIAL,NAME=COAL\n'
        else:
            text1='*MATERIAL,NAME=CAP{}\n'.format(index+1)
        text2='''*ELASTIC
{}e9, {}
*MOHR COULOMB
{}, {}
*MOHR COULOMB HARDENING
{}e6, 0.00 
*CONDUCTIVITY
{}
*DENSITY
{}
*SPECIFIC HEAT
{}
*EXPANSION
{}e-5
'''.format(i[2],i[3],i[4],20.0,i[5],i[6],i[7],i[8],i[9])
        f.write(text1+text2)
with open('test.inp', 'a') as f:
    text1='*BOUNDARY\n XSYM,    1,  1\n XFIXED,  1,  1\n YFIXED,  2,  2 \n'
    text2='''*INITIAL CONDITIONS, TYPE=STRESS, GEOSTATIC
    EALL, -<SIGV_TOP>, {}, -<SIGV_BOT>, 0, <SIGH_COE>, <SIGh_COE> 
*INITIAL CONDITIONS, TYPE=TEMPERATURE
    NALL , <TEMP_INI>
*AMPLITUDE, NAME=TOP_PRESSURE
    0.0, 1.0, 1.0, 1.0
*AMPLITUDE, NAME=PRESS_AMP1
    0.0, <SIGV>, 86400, <GAS_PRES>
*AMPLITUDE, NAME=PRESS_AMP2
    0.0, <SIGH>, 86400, <GAS_PRES> 
*AMPLITUDE, NAME=TEMP_AMP1
    0.0, <TEMP_INI>, 86400, <TEMP_INI>, 432000, <TEMP_GAS>
*AMPLITUDE, NAME=TEMP_AMP2
    0.0, <TEMP_GAS>, 864000, <TEMP_COL>  
*********************************************************************
**STEP1 : EQUILIBRIUM CHECK
*********************************************************************
*STEP, NAME=EQUILIBRIUM CHECK, NLGEOM=YES,INC=2000
*COUPLED TEMPERATURE-DISPLACEMENT, DELTMX=3E+08
    1.0, 1.0, 1.0, 1.0
*BOUNDARY
    INNER, 11, 11, <TEMP_INI>
*DSLOAD
    TOP_SURF,     P, <SIGV_TOP>
    TOP_BOT_SURF, P, <SIGV>
    SIDE_SURF,    P, <SIGH>
*DLOAD
    EALL, BY, -<SIGV_EFF> 
*CONTROLS, PARAMETER=TIME INCREMENTATION
    50, 60, 70, 80, 90, 100, 110, 120, 130, 10, 150 
*OUTPUT, FIELD, VARIABLE=PRESELECT
*ELEMENT OUTPUT, ELSET=EALL
    PE
    PEEQ
*OUTPUT, HISTORY, VARIABLE=PRESELECT
*END STEP
*********************************************************************
**STEP2 : GASIFICATION - INCREASING TEMPERATURE
*********************************************************************
*STEP, NAME=GASIFICATION - INCREASING TEMPERATURE, NLGEOM=YES,INC=2000
*COUPLED TEMPERATURE-DISPLACEMENT, DELTMX=3E+08
    3600, 432000, 1, 43200
*BOUNDARY, AMPLITUDE=TEMP_AMP1
    INNER, 11, 11, 1.0
*DSLOAD, AMPLITUDE=PRESS_AMP1
    TOP_BOT_SURF, P, 1.0
*DSLOAD, AMPLITUDE=PRESS_AMP2
    SIDE_SURF,    P, 1.0
*DSLOAD
    TOP_SURF,     P, <SIGV_TOP> 
*DLOAD
    EALL, BY, -<SIGV_EFF> 
*CONTROLS, PARAMETER=TIME INCREMENTATION
    50, 60, 70, 80, 90, 100, 110, 120, 130, 10, 150 
*OUTPUT, FIELD, VARIABLE=PRESELECT
*ELEMENT OUTPUT, ELSET=EALL
    PE
    PEEQ
*OUTPUT, HISTORY, VARIABLE=PRESELECT
*END STEP
*********************************************************************
**STEP2 : GASIFICATION - MAINTAINING TEMPERATURE
*********************************************************************
*STEP, NAME=GASIFICATION - MAINTAINING TEMPERATURE, NLGEOM=YES,INC=2000
*COUPLED TEMPERATURE-DISPLACEMENT, DELTMX=3E+08"#;
    let str17=format!("\n    3600, {}, 1, 86400",gastime);
    let str18=r#"
*BOUNDARY
    INNER, 11, 11, <TEMP_GAS>
*DSLOAD
    TOP_SURF,     P, <SIGV_TOP> 
*DSLOAD
    TOP_BOT_SURF, P, <GAS_PRES>
*DSLOAD
    SIDE_SURF,    P, <GAS_PRES>
*DLOAD
    EALL, BY, -<SIGV_EFF> 
*CONTROLS, PARAMETER=TIME INCREMENTATION
    50, 60, 70, 80, 90, 100, 110, 120, 130, 10, 150 
*OUTPUT, FIELD, VARIABLE=PRESELECT
*ELEMENT OUTPUT, ELSET=EALL
    PE
    PEEQ
*OUTPUT, HISTORY, VARIABLE=PRESELECT
*END STEP
*********************************************************************
**STEP3 : COOLING
*********************************************************************
*STEP, NAME=COOLING, NLGEOM=YES,INC=2000
*COUPLED TEMPERATURE-DISPLACEMENT, DELTMX=3E+08
    3600, 5184000, 1, 86400
*BOUNDARY, AMPLITUDE=TEMP_AMP2
    INNER, 11, 11, 1.0
*DSLOAD
    TOP_SURF,     P, <SIGV_TOP> 
*DSLOAD
    TOP_BOT_SURF, P, <GAS_PRES>
    SIDE_SURF,    P, <GAS_PRES>
*DLOAD
    EALL, BY, -<SIGV_EFF> 
*CONTROLS, PARAMETER=TIME INCREMENTATION
    50, 60, 70, 80, 90, 100, 110, 120, 130, 10, 150 
*OUTPUT, FIELD, VARIABLE=PRESELECT
*ELEMENT OUTPUT, ELSET=EALL
    PE
    PEEQ
*OUTPUT, HISTORY, VARIABLE=PRESELECT
*END STEP
'''.format(height)
    f.write(text1+text2)
"#;
        return str1.to_owned()+&str2+&str3+&str4+&str5+&str6+&str7+&str8+&str9+&str10+&str11+&str12+&str13+&str14+&str15+&str16+&str17+&str18;
    }
}
