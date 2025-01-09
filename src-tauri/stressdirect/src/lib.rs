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

pub fn stressdirect(jsondata:&str)->String {
    use serde_json::{Result, Value};
    let v: Value = serde_json::from_str(jsondata).unwrap();
    println!("Hello, stressdirect!");
    let str=format!(r#"#encoding: utf-8
# -*- coding: mbcs -*-
from abaqus import *
from abaqusConstants import *
session.Viewport(name='Viewport: 1', origin=(0.0, 0.0), width=184.541656494141, 
    height=126.414352416992)
session.viewports['Viewport: 1'].makeCurrent()
session.viewports['Viewport: 1'].maximize()
from caeModules import *
from driverUtils import executeOnCaeStartup
executeOnCaeStartup()
#: Executing "onCaeStartup()" in the site directory ...
length={}
width={}
height={}
distance={}

#front_view_left=length/4
#front_view_right=length/4*3

front_view_left=length/2-distance/2
front_view_right=length/2+distance/2
front_view_top=height/4*3

top_view_top=width/4*3

session.viewports['Viewport: 1'].partDisplay.geometryOptions.setValues(
    referenceRepresentation=ON)
cliCommand("""session.journalOptions.setValues(replayGeometry=COORDINATE,recoverGeometry= COORDINATE)""")
s = mdb.models['Model-1'].ConstrainedSketch(name='__profile__', 
    sheetSize=200.0)
g, v, d, c = s.geometry, s.vertices, s.dimensions, s.constraints
s.setPrimaryObject(option=STANDALONE)
s.rectangle(point1=(0.0, 0.0), point2=(length, width))
p = mdb.models['Model-1'].Part(name='Part-1', dimensionality=THREE_D, 
    type=DEFORMABLE_BODY)
p = mdb.models['Model-1'].parts['Part-1']
p.BaseSolidExtrude(sketch=s, depth=height)
s.unsetPrimaryObject()
p = mdb.models['Model-1'].parts['Part-1']
session.viewports['Viewport: 1'].setValues(displayedObject=p)
del mdb.models['Model-1'].sketches['__profile__']
p = mdb.models['Model-1'].parts['Part-1']
f, e, d1 = p.faces, p.edges, p.datums
t = p.MakeSketchTransform(sketchPlane=f.findAt(coordinates=(length/2, 0.0, 
    height/2)), sketchUpEdge=e.findAt(coordinates=(length, 0.0, height/2)), 
    sketchPlaneSide=SIDE1, origin=(0.0, 0.0, 0.0))
s1 = mdb.models['Model-1'].ConstrainedSketch(name='__profile__', 
    sheetSize=401.99, gridSpacing=10.04, transform=t)
g, v, d, c = s1.geometry, s1.vertices, s1.dimensions, s1.constraints
s1.setPrimaryObject(option=SUPERIMPOSE)
p = mdb.models['Model-1'].parts['Part-1']
p.projectReferencesOntoSketch(sketch=s1, filter=COPLANAR_EDGES)

s1.Line(point1=(front_view_left, 0), point2=(front_view_left, height))
s1.Line(point1=(front_view_right, 0), point2=(front_view_right, height))
s1.Line(point1=(0, front_view_top), point2=(length, front_view_top))
p = mdb.models['Model-1'].parts['Part-1']
f = p.faces
pickedFaces = f.findAt(((length/2, 0.0, height/2), ))
e1, d2 = p.edges, p.datums
p.PartitionFaceBySketch(sketchUpEdge=e1.findAt(coordinates=(length, 0.0, height/2)), 
    faces=pickedFaces, sketch=s1)
s1.unsetPrimaryObject()
del mdb.models['Model-1'].sketches['__profile__']


#正视图左侧纵向切割
p = mdb.models['Model-1'].parts['Part-1']
c = p.cells
# print(c.__dict__)
pickedCells = c
e, v1, d1 = p.edges, p.vertices, p.datums
p.PartitionCellByPlanePointNormal(point=v1.findAt(coordinates=(front_view_left, 0.0, 
    height)), normal=e.findAt(coordinates=(length/2, 0.0, height)), cells=pickedCells)


#正视图右侧纵向切割
p = mdb.models['Model-1'].parts['Part-1']
c = p.cells
pickedCells = c
e1, v2, d2 = p.edges, p.vertices, p.datums
p.PartitionCellByPlanePointNormal(point=v2.findAt(coordinates=(front_view_right, 0.0, 
    height)), normal=e1.findAt(coordinates=(length/2, 0.0, height)), 
    cells=pickedCells)
#正视图上侧横向切割
p = mdb.models['Model-1'].parts['Part-1']
c = p.cells
pickedCells=c
e, v1, d1 = p.edges, p.vertices, p.datums
p.PartitionCellByPlanePointNormal(point=v1.findAt(coordinates=(0.0, 0.0, 
    front_view_top)), normal=e.findAt(coordinates=(0.0, 0.0, height/2)), cells=pickedCells)


p = mdb.models['Model-1'].parts['Part-1']
f1, e1, d2 = p.faces, p.edges, p.datums
t = p.MakeSketchTransform(sketchPlane=f1.findAt(coordinates=(length/2, 
    width/2, height)), sketchUpEdge=e1.findAt(coordinates=(length, width/2, height)), 
    sketchPlaneSide=SIDE1, origin=(0.0, 0.0, 0.0))
s = mdb.models['Model-1'].ConstrainedSketch(name='__profile__', 
    sheetSize=448.99, gridSpacing=11.22, transform=t)
g, v, d, c = s.geometry, s.vertices, s.dimensions, s.constraints
s.setPrimaryObject(option=SUPERIMPOSE)
p = mdb.models['Model-1'].parts['Part-1']
p.projectReferencesOntoSketch(sketch=s, filter=COPLANAR_EDGES)


s.Line(point1=(0,top_view_top), point2=(length, top_view_top))

p = mdb.models['Model-1'].parts['Part-1']
f = p.faces
pickedFaces = f.findAt(((front_view_left/2,width/2, height), ), (((front_view_left+front_view_right)/2,width/2,height), ), (((front_view_right+length)/2, width/2, height), ))
e, d1 = p.edges, p.datums
p.PartitionFaceBySketch(sketchUpEdge=e.findAt(coordinates=(length, width/2, height)), 
    faces=pickedFaces, sketch=s)
s.unsetPrimaryObject()
del mdb.models['Model-1'].sketches['__profile__']

p = mdb.models['Model-1'].parts['Part-1']
c = p.cells
pickedCells = c
e1, v2, d2 = p.edges, p.vertices, p.datums
p.PartitionCellByPlanePointNormal(point=v2.findAt(coordinates=(0.0, top_view_top, 
    height)), normal=e1.findAt(coordinates=(0.0, width/2, height)), 
    cells=pickedCells)
# session.viewports['Viewport: 1'].partDisplay.setValues(renderStyle=WIREFRAME)



p = mdb.models['Model-1'].parts['Part-1']
f = p.faces
faces = f.findAt(((front_view_left, top_view_top/2, (height+front_view_top)/2), ), ((front_view_right, top_view_top/2, (height+front_view_top)/2), ))
p.Set(faces=faces, name='Set-1')
# #: The set 'Set-1' has been created (2 faces).

# (front_view_left+front_view_right)/2
# top_view_top/2
# (top_view_top+width)/2
cells = c.findAt((((front_view_left+front_view_right)/2, top_view_top/2, 0.0), ), (((front_view_left+front_view_right)/2, top_view_top/2,height), ), ((
    (front_view_left+front_view_right)/2, (top_view_top+width)/2, 0.0), ), (((front_view_left+front_view_right)/2, (top_view_top+width)/2, height), ))
leaf = dgm.LeafFromGeometry(cellSeq=cells)
session.viewports['Viewport: 1'].partDisplay.displayGroup.replace(leaf=leaf)

p = mdb.models['Model-1'].parts['Part-1']
s = p.faces
side1Faces = s.findAt(((front_view_left, top_view_top/2, (front_view_top+height)/2), ))
side2Faces = s.findAt(((front_view_right, top_view_top/2, (front_view_top+height)/2), ))
p.Surface(side1Faces=side1Faces, side2Faces=side2Faces, name='Surf-2')
#: The surface 'Surf-2' has been created (2 faces).

leaf = dgm.Leaf(leafType=DEFAULT_MODEL)
session.viewports['Viewport: 1'].partDisplay.displayGroup.replace(leaf=leaf)


p = mdb.models['Model-1'].parts['Part-1']
c = p.cells
cells = c.findAt((((front_view_left+front_view_right)/2, top_view_top/2, 0.0), ), (((front_view_left+front_view_right)/2, top_view_top/2,height), ), ((
    (front_view_left+front_view_right)/2, (top_view_top+width)/2, 0.0), ), (((front_view_left+front_view_right)/2, (top_view_top+width)/2, height), ))
leaf = dgm.LeafFromGeometry(cellSeq=cells)
session.viewports['Viewport: 1'].partDisplay.displayGroup.remove(leaf=leaf)

p = mdb.models['Model-1'].parts['Part-1']
s = p.faces
side1Faces = s.findAt(((front_view_right, top_view_top/2, (front_view_top+height)/2), ))
side2Faces = s.findAt(((front_view_left, top_view_top/2, (front_view_top+height)/2), ))
p.Surface(side1Faces=side1Faces, side2Faces=side2Faces, name='Surf-3')

leaf = dgm.Leaf(leafType=DEFAULT_MODEL)
session.viewports['Viewport: 1'].partDisplay.displayGroup.replace(leaf=leaf)

a=mdb.models['Model-1'].parts['Part-1']
a.SurfaceByBoolean(name='Surf-1', surfaces=(a.surfaces['Surf-2'], 
    a.surfaces['Surf-3'], ))

p = mdb.models['Model-1'].parts['Part-1']
f = p.faces
set1=()
for i in f:
    if i.pointOn[0][0]==0 or i.pointOn[0][0]==length:
        set1=set1+(i.pointOn[0],)
# print(set1)
pickedRegions = f.findAt(*[(coord,) for coord in set1])
p.Set(faces=pickedRegions, name='Set-x')

set2=()
for i in f:
    if i.pointOn[0][1]==0 or i.pointOn[0][1]==width:
        set2=set2+(i.pointOn[0],)
pickedRegions = f.findAt(*[(coord,) for coord in set2])
p.Set(faces=pickedRegions, name='Set-y')


set3=()
for i in f:
    if i.pointOn[0][2]==0 or i.pointOn[0][2]==height:
        set3=set3+(i.pointOn[0],)
pickedRegions = f.findAt(*[(coord,) for coord in set3])
p.Set(faces=pickedRegions, name='Set-z')


c=p.cells
p.Set(cells=c, name='Set-ALL')

set4=()
for i in c:
    if i.pointOn[0][2]>front_view_top:
        set4=set4+(i.pointOn[0],)
pickedRegions = c.findAt(*[(coord,) for coord in set4])
p.Set(cells=pickedRegions, name='R')

set5=()
for i in c:
    if i.pointOn[0][2]<front_view_top:
        set5=set5+(i.pointOn[0],)
pickedRegions = c.findAt(*[(coord,) for coord in set5])
p.Set(cells=pickedRegions, name='C')

#设置材料属性
mdb.models['Model-1'].Material(name='Material-1')
mdb.models['Model-1'].materials['Material-1'].Elastic(table=((30000000000.0, 
    0.3), ))
#mdb.models['Model-1'].materials['Material-1'].Density(table=((1000.0, ), ))
mdb.models['Model-1'].HomogeneousSolidSection(name='Section-1', 
    material='Material-1', thickness=None)
p = mdb.models['Model-1'].parts['Part-1']
region = p.sets['Set-ALL']
p = mdb.models['Model-1'].parts['Part-1']
p.SectionAssignment(region=region, sectionName='Section-1', offset=0.0, 
    offsetType=MIDDLE_SURFACE, offsetField='', 
    thicknessAssignment=FROM_SECTION)

a = mdb.models['Model-1'].rootAssembly
a.DatumCsysByDefault(CARTESIAN)
p = mdb.models['Model-1'].parts['Part-1']
a.Instance(name='Part-1-1', part=p, dependent=OFF)
session.viewports['Viewport: 1'].assemblyDisplay.setValues(interactions=ON, 
    constraints=ON, connectors=ON, engineeringFeatures=ON)
a = mdb.models['Model-1'].rootAssembly
pickedRegions = a.instances['Part-1-1'].sets['Set-1']
mdb.models['Model-1'].rootAssembly.engineeringFeatures.assignSeam(
    regions=pickedRegions)

session.viewports['Viewport: 1'].assemblyDisplay.setValues(mesh=ON, 
    interactions=OFF, constraints=OFF, connectors=OFF, engineeringFeatures=OFF)
session.viewports['Viewport: 1'].assemblyDisplay.meshOptions.setValues(
    meshTechnique=ON)
a = mdb.models['Model-1'].rootAssembly
partInstances =(a.instances['Part-1-1'], )
a.seedPartInstance(regions=partInstances, size=10.0, deviationFactor=0.1, 
    minSizeFactor=0.1)
a = mdb.models['Model-1'].rootAssembly
partInstances =(a.instances['Part-1-1'], )
a.generateMesh(regions=partInstances)

session.viewports['Viewport: 1'].assemblyDisplay.setValues(mesh=OFF)
session.viewports['Viewport: 1'].assemblyDisplay.meshOptions.setValues(
    meshTechnique=OFF)



#设置时间步
a = mdb.models['Model-1'].rootAssembly
session.viewports['Viewport: 1'].setValues(displayedObject=a)
session.viewports['Viewport: 1'].assemblyDisplay.setValues(
    adaptiveMeshConstraints=ON)
mdb.models['Model-1'].GeostaticStep(name='Step-1', previous='Initial')
session.viewports['Viewport: 1'].assemblyDisplay.setValues(step='Step-1')
mdb.models['Model-1'].StaticStep(name='Step-2', previous='Step-1')
session.viewports['Viewport: 1'].assemblyDisplay.setValues(step='Step-2')
a = mdb.models['Model-1'].rootAssembly
session.viewports['Viewport: 1'].setValues(displayedObject=a)
session.viewports['Viewport: 1'].assemblyDisplay.setValues(
    adaptiveMeshConstraints=ON)
session.viewports['Viewport: 1'].assemblyDisplay.setValues(step='Step-1')
mdb.models['Model-1'].FieldOutputRequest(name='F-Output-1', 
    createStepName='Step-1', variables=('S', 'MISES',   
     'U', 'UT', 'UR', 'V', 'VT', 'VR', 'RBANG', 
   ))


#设置边界条件
a1 = mdb.models['Model-1'].rootAssembly
region = a1.instances['Part-1-1'].surfaces['Surf-3']
mdb.models['Model-1'].Pressure(name='Load-1', createStepName='Step-2', 
    region=region, distributionType=UNIFORM, field='', magnitude=75000000.0, 
    amplitude=UNSET)

session.viewports['Viewport: 1'].assemblyDisplay.setValues(step='Initial')
a1 = mdb.models['Model-1'].rootAssembly
region = a1.instances['Part-1-1'].sets['Set-x']
mdb.models['Model-1'].DisplacementBC(name='BC-1', createStepName='Initial', 
    region=region, u1=SET, u2=UNSET, u3=UNSET, ur1=UNSET, ur2=UNSET, ur3=UNSET, 
    amplitude=UNSET, distributionType=UNIFORM, fieldName='', localCsys=None)
a1 = mdb.models['Model-1'].rootAssembly
region = a1.instances['Part-1-1'].sets['Set-y']
mdb.models['Model-1'].DisplacementBC(name='BC-2', createStepName='Initial', 
    region=region, u1=UNSET, u2=SET, u3=UNSET, ur1=UNSET, ur2=UNSET, ur3=UNSET, 
    amplitude=UNSET, distributionType=UNIFORM, fieldName='', localCsys=None)
a1 = mdb.models['Model-1'].rootAssembly
region = a1.instances['Part-1-1'].sets['Set-z']
mdb.models['Model-1'].DisplacementBC(name='BC-3', createStepName='Initial', 
    region=region, u1=UNSET, u2=UNSET, u3=SET, ur1=UNSET, ur2=UNSET, ur3=UNSET, 
    amplitude=UNSET, distributionType=UNIFORM, fieldName='', localCsys=None)

a1 = mdb.models['Model-1'].rootAssembly
region = a1.instances['Part-1-1'].sets['R']
mdb.models['Model-1'].Stress(name='Predefined Field-1', region=region, 
    distributionType=UNIFORM, sigma11=72000000.0, sigma22=87000000.0, 
    sigma33=97000000.0, sigma12=0.0, sigma13=0.0, sigma23=0.0)

a1 = mdb.models['Model-1'].rootAssembly
region = a1.instances['Part-1-1'].sets['C']
mdb.models['Model-1'].Stress(name='Predefined Field-2', region=region, 
    distributionType=UNIFORM, sigma11=80000000.0, sigma22=87000000.0, 
    sigma33=97000000.0, sigma12=0.0, sigma13=0.0, sigma23=0.0)

mdb.Job(name='JOB', model='Model-1', description='', type=ANALYSIS, 
    atTime=None, waitMinutes=0, waitHours=0, queue=None, memory=90, 
    memoryUnits=PERCENTAGE, getMemoryFromAnalysis=True, 
    explicitPrecision=SINGLE, nodalOutputPrecision=SINGLE, echoPrint=OFF, 
    modelPrint=OFF, contactPrint=OFF, historyPrint=OFF, userSubroutine='', 
    scratch='', resultsFormat=ODB, numThreadsPerMpiProcess=1, 
    multiprocessingMode=DEFAULT, numCpus=4, numDomains=4, numGPUs=0)
mdb.jobs['JOB'].submit(consistencyChecking=OFF)
"#,v["length"],v["width"],v["height"],v["distance"]); 
    str.to_string()

}
