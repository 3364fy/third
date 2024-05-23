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

pub mod post {
    use std::str;
    pub fn write_inp(path: &str) -> String {
        let str=format!("# -*- coding: mbcs -*-
#
# Abaqus/Viewer Release 2018 replay file
# Internal Version: 2017_11_08-01.21.41 127140
# Run by pkuli on Tue May 14 03:05:49 2024
#

# from driverUtils import executeOnCaeGraphicsStartup
# executeOnCaeGraphicsStartup()
#: Executing \"onCaeGraphicsStartup()\" in the site directory ...
#: Warning: Permission was denied for \"abaqus.rpy\"; \"abaqus.rpy.7\" will be used for this session's replay file.
from abaqus import *
from abaqusConstants import *
session.Viewport(name='Viewport: 1', origin=(0.0, 0.0), width=359.216156005859, 
    height=194.903717041016)
session.viewports['Viewport: 1'].makeCurrent()
session.viewports['Viewport: 1'].maximize()
from viewerModules import *
from driverUtils import executeOnCaeStartup
executeOnCaeStartup()
o1 = session.openOdb(name='{}\\test.odb')
session.viewports['Viewport: 1'].setValues(displayedObject=o1)
#: Model: D:/temp/test0/test.odb
#: Number of Assemblies:         1
#: Number of Assembly instances: 0
#: Number of Part instances:     1
#: Number of Meshes:             1
#: Number of Element Sets:       18
#: Number of Node Sets:          17
#: Number of Steps:              4
session.viewports['Viewport: 1'].odbDisplay.commonOptions.setValues(
    visibleEdges=FREE)
session.viewports['Viewport: 1'].odbDisplay.basicOptions.setValues(
    mirrorAboutYzPlane=True)
session.viewports['Viewport: 1'].odbDisplay.commonOptions.setValues(
    visibleEdges=ALL)
session.viewports['Viewport: 1'].odbDisplay.commonOptions.setValues(
    visibleEdges=NONE)
session.viewports['Viewport: 1'].viewportAnnotationOptions.setValues(triad=OFF, 
    title=OFF, annotations=OFF, compass=OFF)
session.viewports['Viewport: 1'].view.setValues(session.views['Front'])
session.viewports['Viewport: 1'].odbDisplay.setPrimaryVariable(
    variableLabel='S', outputPosition=INTEGRATION_POINT, refinement=(COMPONENT, 
    'S11'), )
session.viewports['Viewport: 1'].odbDisplay.display.setValues(
    plotState=CONTOURS_ON_DEF)
session.viewports['Viewport: 1'].viewportAnnotationOptions.setValues(
    legendFont='-*-times new roman-medium-r-normal-*-*-180-*-*-p-*-*-*')
session.viewports['Viewport: 1'].viewportAnnotationOptions.setValues(
    stateFont='-*-times new roman-medium-r-normal-*-*-180-*-*-p-*-*-*')
session.viewports['Viewport: 1'].viewportAnnotationOptions.setValues(
    legendBox=OFF, legendDecimalPlaces=2)
session.viewports['Viewport: 1'].viewportAnnotationOptions.setValues(
    legendFont='-*-times new roman-bold-r-normal-*-*-180-*-*-p-*-*-*', 
    stateFont='-*-times new roman-bold-r-normal-*-*-180-*-*-p-*-*-*')
session.viewports['Viewport: 1'].view.setValues(nearPlane=697.904, 
    farPlane=1099.03, width=386.756, height=206.175, viewOffsetX=-0.280064, 
    viewOffsetY=-8.67222)
session.pngOptions.setValues(imageSize=(4096, 2186))
session.printToFile(fileName='S11', format=PNG, canvasObjects=(
    session.viewports['Viewport: 1'], ))
session.viewports['Viewport: 1'].odbDisplay.setPrimaryVariable(
    variableLabel='S', outputPosition=INTEGRATION_POINT, refinement=(COMPONENT, 
    'S22'), )
session.printToFile(fileName='S22', format=PNG, canvasObjects=(
    session.viewports['Viewport: 1'], ))
session.viewports['Viewport: 1'].odbDisplay.setPrimaryVariable(
    variableLabel='S', outputPosition=INTEGRATION_POINT, refinement=(COMPONENT, 
    'S33'), )
session.printToFile(fileName='S33', format=PNG, canvasObjects=(
    session.viewports['Viewport: 1'], ))
session.viewports['Viewport: 1'].odbDisplay.setPrimaryVariable(
    variableLabel='NT11', outputPosition=NODAL, )
session.printToFile(fileName='T', format=PNG, canvasObjects=(
    session.viewports['Viewport: 1'], ))
session.viewports['Viewport: 1'].odbDisplay.setPrimaryVariable(
    variableLabel='PEEQ', outputPosition=INTEGRATION_POINT, )
session.viewports['Viewport: 1'].odbDisplay.contourOptions.setValues(
    maxAutoCompute=OFF, maxValue=0.01, minValue=0)
session.viewports['Viewport: 1'].odbDisplay.contourOptions.setValues(
    maxValue=0.001)
session.printToFile(fileName='DAMAGE', format=PNG, canvasObjects=(
    session.viewports['Viewport: 1'], ))
",path);
    return str.to_string();
    }
}