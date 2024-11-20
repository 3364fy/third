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

front_view_left=length/4
front_view_right=length/4*3
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

p = mdb.models['Model-1'].parts['Part-1']
s = p.faces
side1Faces = f.findAt(((front_view_left, top_view_top/2, (height+front_view_top)/2), ), ((front_view_right, top_view_top/2, (height+front_view_top)/2), ))
p.Surface(side1Faces=side1Faces, name='Surf-1')
# #: The surface 'Surf-1' has been created (2 faces).

a = mdb.models['Model-1'].rootAssembly
session.viewports['Viewport: 1'].setValues(displayedObject=a)
session.viewports['Viewport: 1'].assemblyDisplay.setValues(
    optimizationTasks=OFF, geometricRestrictions=OFF, stopConditions=OFF)
a = mdb.models['Model-1'].rootAssembly
a.DatumCsysByDefault(CARTESIAN)

p = mdb.models['Model-1'].parts['Part-1']
a.Instance(name='Part-1-1', part=p, dependent=OFF)
p = mdb.models['Model-1'].parts['Part-1']
session.viewports['Viewport: 1'].setValues(displayedObject=p)
a = mdb.models['Model-1'].rootAssembly
session.viewports['Viewport: 1'].setValues(displayedObject=a)
session.viewports['Viewport: 1'].assemblyDisplay.setValues(interactions=ON, 
    constraints=ON, connectors=ON, engineeringFeatures=ON)
session.viewports['Viewport: 1'].assemblyDisplay.setValues(
    renderStyle=WIREFRAME)
session.viewports['Viewport: 1'].assemblyDisplay.setValues(renderStyle=SHADED)
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
a.seedPartInstance(regions=partInstances, size=1.0, deviationFactor=0.1, 
    minSizeFactor=0.1)
a = mdb.models['Model-1'].rootAssembly
partInstances =(a.instances['Part-1-1'], )
a.generateMesh(regions=partInstances)

session.viewports['Viewport: 1'].assemblyDisplay.setValues(mesh=OFF)
session.viewports['Viewport: 1'].assemblyDisplay.meshOptions.setValues(
    meshTechnique=OFF)
mdb.Job(name='GEOMODEL', model='Model-1', description='', type=ANALYSIS, 
    atTime=None, waitMinutes=0, waitHours=0, queue=None, memory=90, 
    memoryUnits=PERCENTAGE, getMemoryFromAnalysis=True, 
    explicitPrecision=SINGLE, nodalOutputPrecision=SINGLE, echoPrint=OFF, 
    modelPrint=OFF, contactPrint=OFF, historyPrint=OFF, userSubroutine='', 
    scratch='', resultsFormat=ODB, parallelizationMethodExplicit=DOMAIN, 
    numDomains=1, activateLoadBalancing=False, numThreadsPerMpiProcess=1, 
    multiprocessingMode=DEFAULT, numCpus=1, numGPUs=0)
mdb.jobs['GEOMODEL'].writeInput(consistencyChecking=OFF)

session.viewports['Viewport: 1'].viewportAnnotationOptions.setValues(triad=OFF, 
    legend=OFF, title=OFF, state=OFF, annotations=OFF, compass=OFF)
session.viewports['Viewport: 1'].view.setValues(nearPlane=390.106, 
    farPlane=580.446, width=305.577, height=144.396, cameraPosition=(4.93089, 
    -279.876, 353.12), cameraUpVector=(0.160838, 0.893677, 0.418895), 
    cameraTarget=(108.021, 45.6847, 6.29384))
session.viewports['Viewport: 1'].view.setValues(nearPlane=382.567, 
    farPlane=587.682, width=299.672, height=141.606, cameraPosition=(-23.7718, 
    -295.065, 327.885), cameraUpVector=(0.0900128, 0.88044, 0.465536), 
    cameraTarget=(108.107, 45.7302, 6.36941))
session.viewports['Viewport: 1'].view.setValues(nearPlane=389.735, 
    farPlane=583.73, width=305.288, height=144.259, cameraPosition=(22.64, 
    -349.529, 277.194), cameraUpVector=(-0.0773949, 0.817551, 0.570631), 
    cameraTarget=(107.953, 45.9104, 6.53714))
session.viewports['Viewport: 1'].view.setValues(nearPlane=361.298, 
    farPlane=608.382, width=283.012, height=133.733, cameraPosition=(-117.638, 
    -336.594, 205.798), cameraUpVector=(0.0606263, 0.738327, 0.671713), 
    cameraTarget=(107.952, 45.9105, 6.5367))
session.viewports['Viewport: 1'].view.setValues(nearPlane=363.508, 
    farPlane=606.172, width=284.743, height=134.551, cameraUpVector=(0.149361, 
    0.699914, 0.698435), cameraTarget=(107.952, 45.9105, 6.5367))
session.viewports['Viewport: 1'].view.setValues(nearPlane=363.525, 
    farPlane=606.155, width=284.756, height=134.557, cameraPosition=(-117.638, 
    -336.594, 205.798), cameraUpVector=(0.145112, 0.701866, 0.697371), 
    cameraTarget=(107.952, 45.9105, 6.5367))
session.viewports['Viewport: 1'].view.setValues(cameraPosition=(-117.638, 
    -336.594, 205.798), cameraUpVector=(0.128046, 0.709592, 0.692881), 
    cameraTarget=(107.952, 45.9105, 6.5367))
session.viewports['Viewport: 1'].view.setValues(nearPlane=405.571, 
    farPlane=572.607, width=317.691, height=150.12, cameraPosition=(116.509, 
    -403.081, 193.64), cameraUpVector=(-0.0534894, 0.668773, 0.74154), 
    cameraTarget=(107.039, 46.1696, 6.58408))
session.viewports['Viewport: 1'].view.setValues(nearPlane=360.51, 
    farPlane=610.343, width=282.394, height=133.441, cameraPosition=(-120.298, 
    -338.525, 200.296), cameraUpVector=(0.163895, 0.683127, 0.711671), 
    cameraTarget=(105.897, 46.4811, 6.61619))
session.pngOptions.setValues(imageSize=(4096, 1931))
session.printOptions.setValues(vpDecorations=OFF)
session.printToFile(fileName='1', format=PNG, canvasObjects=(
    session.viewports['Viewport: 1'], ))
"#,v["length"],v["width"],v["height"]); 
    str.to_string()

}
