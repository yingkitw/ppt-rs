use crate::generator::charts::Chart;

/// Generate chart style XML content
pub fn generate_chart_style_xml(_chart: &Chart) -> String {
    format!(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<cs:chartStyle xmlns:cs="http://schemas.microsoft.com/office/drawing/2012/chartStyle" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" meth="combo" id="10">
  <cs:axisTitle>
    <cs:lnRef idx="0"/>
    <cs:fillRef idx="0"/>
    <cs:effectRef idx="0"/>
    <cs:fontRef idx="minor">
      <a:schemeClr val="tx1">
        <a:lumMod val="65000"/>
        <a:lumOff val="35000"/>
      </a:schemeClr>
    </cs:fontRef>
    <cs:defRPr sz="900" kern="1200"/>
  </cs:axisTitle>
  <cs:categoryAxis>
    <cs:lnRef idx="0"/>
    <cs:fillRef idx="0"/>
    <cs:effectRef idx="0"/>
    <cs:fontRef idx="minor">
      <a:schemeClr val="tx1">
        <a:lumMod val="65000"/>
        <a:lumOff val="35000"/>
      </a:schemeClr>
    </cs:fontRef>
    <cs:defRPr sz="900" kern="1200"/>
  </cs:categoryAxis>
  <cs:chartArea>
    <cs:lnRef idx="0"/>
    <cs:fillRef idx="0"/>
    <cs:effectRef idx="0"/>
    <cs:fontRef idx="minor">
      <a:schemeClr val="bg1"/>
    </cs:fontRef>
    <cs:spPr>
      <a:solidFill>
        <a:schemeClr val="bg1"/>
      </a:solidFill>
      <a:ln>
        <a:noFill/>
      </a:ln>
      <a:effectLst/>
    </cs:spPr>
  </cs:chartArea>
  <cs:dataLabel>
    <cs:lnRef idx="0"/>
    <cs:fillRef idx="0"/>
    <cs:effectRef idx="0"/>
    <cs:fontRef idx="minor">
      <a:schemeClr val="lt1"/>
    </cs:fontRef>
    <cs:defRPr sz="900" kern="1200"/>
  </cs:dataLabel>
  <cs:dataLabelCallout>
    <cs:lnRef idx="0"/>
    <cs:fillRef idx="0"/>
    <cs:effectRef idx="0"/>
    <cs:fontRef idx="minor">
      <a:schemeClr val="tx1"/>
    </cs:fontRef>
    <cs:spPr>
      <a:solidFill>
        <a:schemeClr val="bg1"/>
        <a:lumMod val="45000"/>
      </a:solidFill>
      <a:ln w="9525" cap="flat" cmpd="sng" algn="ctr">
        <a:solidFill>
          <a:schemeClr val="tx1">
            <a:lumMod val="50000"/>
            <a:lumOff val="50000"/>
          </a:schemeClr>
        </a:solidFill>
        <a:round/>
      </a:ln>
      <a:effectLst/>
    </cs:spPr>
    <cs:defRPr sz="900" kern="1200"/>
  </cs:dataLabelCallout>
  <cs:dataPoint>
    <cs:lnRef idx="0"/>
    <cs:fillRef idx="0"/>
    <cs:effectRef idx="0"/>
    <cs:fontRef idx="minor">
      <a:schemeClr val="tx1"/>
    </cs:fontRef>
    <cs:spPr>
      <a:solidFill>
        <a:schemeClr val="accent1"/>
      </a:solidFill>
      <a:ln>
        <a:solidFill>
          <a:schemeClr val="bg1"/>
        </a:solidFill>
      </a:ln>
      <a:effectLst/>
    </cs:spPr>
  </cs:dataPoint>
  <cs:dataPoint3D>
    <cs:lnRef idx="0"/>
    <cs:fillRef idx="0"/>
    <cs:effectRef idx="0"/>
    <cs:fontRef idx="minor">
      <a:schemeClr val="tx1"/>
    </cs:fontRef>
    <cs:spPr>
      <a:solidFill>
        <a:schemeClr val="accent1"/>
      </a:solidFill>
      <a:ln w="9525" cap="flat" cmpd="sng" algn="ctr">
        <a:solidFill>
          <a:schemeClr val="bg1"/>
        </a:solidFill>
        <a:round/>
      </a:ln>
      <a:effectLst/>
    </cs:spPr>
  </cs:dataPoint3D>
  <cs:dataPointLine>
    <cs:lnRef idx="0"/>
    <cs:fillRef idx="0"/>
    <cs:effectRef idx="0"/>
    <cs:fontRef idx="minor">
      <a:schemeClr val="tx1"/>
    </cs:fontRef>
    <cs:spPr>
      <a:ln w="19050" cap="flat" cmpd="sng" algn="ctr">
        <a:solidFill>
          <a:schemeClr val="accent1"/>
        </a:solidFill>
        <a:round/>
      </a:ln>
      <a:effectLst/>
    </cs:spPr>
  </cs:dataPointLine>
  <cs:dataPointMarker>
    <cs:lnRef idx="0"/>
    <cs:fillRef idx="0"/>
    <cs:effectRef idx="0"/>
    <cs:fontRef idx="minor">
      <a:schemeClr val="tx1"/>
    </cs:fontRef>
    <cs:spPr>
      <a:solidFill>
        <a:schemeClr val="accent1"/>
      </a:solidFill>
      <a:ln w="9525" cap="flat" cmpd="sng" algn="ctr">
        <a:solidFill>
          <a:schemeClr val="bg1"/>
        </a:solidFill>
        <a:round/>
      </a:ln>
      <a:effectLst/>
    </cs:spPr>
  </cs:dataPointMarker>
  <cs:dataTable>
    <cs:lnRef idx="0"/>
    <cs:fillRef idx="0"/>
    <cs:effectRef idx="0"/>
    <cs:fontRef idx="minor">
      <a:schemeClr val="tx1">
        <a:lumMod val="65000"/>
        <a:lumOff val="35000"/>
      </a:schemeClr>
    </cs:fontRef>
    <cs:defRPr sz="900" kern="1200"/>
  </cs:dataTable>
  <cs:downBar>
    <cs:lnRef idx="0"/>
    <cs:fillRef idx="0"/>
    <cs:effectRef idx="0"/>
    <cs:fontRef idx="minor">
      <a:schemeClr val="tx1"/>
    </cs:fontRef>
    <cs:spPr>
      <a:solidFill>
        <a:schemeClr val="lt1"/>
      </a:solidFill>
      <a:ln w="9525" cap="flat" cmpd="sng" algn="ctr">
        <a:solidFill>
          <a:schemeClr val="tx1">
            <a:lumMod val="65000"/>
            <a:lumOff val="35000"/>
          </a:schemeClr>
        </a:solidFill>
        <a:round/>
      </a:ln>
      <a:effectLst/>
    </cs:spPr>
  </cs:downBar>
  <cs:dropLine>
    <cs:lnRef idx="0"/>
    <cs:fillRef idx="0"/>
    <cs:effectRef idx="0"/>
    <cs:fontRef idx="minor">
      <a:schemeClr val="tx1"/>
    </cs:fontRef>
    <cs:spPr>
      <a:ln w="9525" cap="flat" cmpd="sng" algn="ctr">
        <a:solidFill>
          <a:schemeClr val="tx1">
            <a:lumMod val="35000"/>
            <a:lumOff val="65000"/>
          </a:schemeClr>
        </a:solidFill>
        <a:round/>
      </a:ln>
      <a:effectLst/>
    </cs:spPr>
  </cs:dropLine>
  <cs:errorBar>
    <cs:lnRef idx="0"/>
    <cs:fillRef idx="0"/>
    <cs:effectRef idx="0"/>
    <cs:fontRef idx="minor">
      <a:schemeClr val="tx1"/>
    </cs:fontRef>
    <cs:spPr>
      <a:ln w="9525" cap="flat" cmpd="sng" algn="ctr">
        <a:solidFill>
          <a:schemeClr val="tx1">
            <a:lumMod val="35000"/>
            <a:lumOff val="65000"/>
          </a:schemeClr>
        </a:solidFill>
        <a:round/>
      </a:ln>
      <a:effectLst/>
    </cs:spPr>
  </cs:errorBar>
  <cs:floor>
    <cs:lnRef idx="0"/>
    <cs:fillRef idx="0"/>
    <cs:effectRef idx="0"/>
    <cs:fontRef idx="minor">
      <a:schemeClr val="tx1"/>
    </cs:fontRef>
    <cs:spPr>
      <a:solidFill>
        <a:schemeClr val="bg2"/>
      </a:solidFill>
      <a:ln>
        <a:solidFill>
          <a:schemeClr val="tx1">
            <a:lumMod val="25000"/>
            <a:lumOff val="75000"/>
          </a:schemeClr>
        </a:solidFill>
      </a:ln>
      <a:effectLst/>
    </cs:spPr>
  </cs:floor>
  <cs:gridlineMajor>
    <cs:lnRef idx="0"/>
    <cs:fillRef idx="0"/>
    <cs:effectRef idx="0"/>
    <cs:fontRef idx="minor">
      <a:schemeClr val="tx1"/>
    </cs:fontRef>
    <cs:spPr>
      <a:ln w="9525" cap="flat" cmpd="sng" algn="ctr">
        <a:solidFill>
          <a:schemeClr val="tx1">
            <a:lumMod val="5000"/>
            <a:lumOff val="95000"/>
          </a:schemeClr>
        </a:solidFill>
        <a:round/>
      </a:ln>
      <a:effectLst/>
    </cs:spPr>
  </cs:gridlineMajor>
  <cs:gridlineMinor>
    <cs:lnRef idx="0"/>
    <cs:fillRef idx="0"/>
    <cs:effectRef idx="0"/>
    <cs:fontRef idx="minor">
      <a:schemeClr val="tx1"/>
    </cs:fontRef>
    <cs:spPr>
      <a:ln w="9525" cap="flat" cmpd="sng" algn="ctr">
        <a:solidFill>
          <a:schemeClr val="tx1">
            <a:lumMod val="5000"/>
            <a:lumOff val="95000"/>
          </a:schemeClr>
        </a:solidFill>
        <a:round/>
      </a:ln>
      <a:effectLst/>
    </cs:spPr>
  </cs:gridlineMinor>
  <cs:hiLoLine>
    <cs:lnRef idx="0"/>
    <cs:fillRef idx="0"/>
    <cs:effectRef idx="0"/>
    <cs:fontRef idx="minor">
      <a:schemeClr val="tx1"/>
    </cs:fontRef>
    <cs:spPr>
      <a:ln w="9525" cap="flat" cmpd="sng" algn="ctr">
        <a:solidFill>
          <a:schemeClr val="tx1">
            <a:lumMod val="50000"/>
            <a:lumOff val="50000"/>
          </a:schemeClr>
        </a:solidFill>
        <a:round/>
      </a:ln>
      <a:effectLst/>
    </cs:spPr>
  </cs:hiLoLine>
  <cs:leaderLine>
    <cs:lnRef idx="0"/>
    <cs:fillRef idx="0"/>
    <cs:effectRef idx="0"/>
    <cs:fontRef idx="minor">
      <a:schemeClr val="tx1"/>
    </cs:fontRef>
    <cs:spPr>
      <a:ln w="9525" cap="flat" cmpd="sng" algn="ctr">
        <a:solidFill>
          <a:schemeClr val="tx1">
            <a:lumMod val="35000"/>
            <a:lumOff val="65000"/>
          </a:schemeClr>
        </a:solidFill>
        <a:round/>
      </a:ln>
      <a:effectLst/>
    </cs:spPr>
  </cs:leaderLine>
  <cs:legend>
    <cs:lnRef idx="0"/>
    <cs:fillRef idx="0"/>
    <cs:effectRef idx="0"/>
    <cs:fontRef idx="minor">
      <a:schemeClr val="tx1">
        <a:lumMod val="65000"/>
        <a:lumOff val="35000"/>
      </a:schemeClr>
    </cs:fontRef>
    <cs:defRPr sz="900" kern="1200"/>
  </cs:legend>
  <cs:plotArea mods="allowNoFillOverride allowNoLineOverride">
    <cs:lnRef idx="0"/>
    <cs:fillRef idx="0"/>
    <cs:effectRef idx="0"/>
    <cs:fontRef idx="minor">
      <a:schemeClr val="tx1"/>
    </cs:fontRef>
  </cs:plotArea>
  <cs:plotArea3D mods="allowNoFillOverride allowNoLineOverride">
    <cs:lnRef idx="0"/>
    <cs:fillRef idx="0"/>
    <cs:effectRef idx="0"/>
    <cs:fontRef idx="minor">
      <a:schemeClr val="tx1"/>
    </cs:fontRef>
  </cs:plotArea3D>
  <cs:seriesAxis>
    <cs:lnRef idx="0"/>
    <cs:fillRef idx="0"/>
    <cs:effectRef idx="0"/>
    <cs:fontRef idx="minor">
      <a:schemeClr val="tx1">
        <a:lumMod val="65000"/>
        <a:lumOff val="35000"/>
      </a:schemeClr>
    </cs:fontRef>
    <cs:defRPr sz="900" kern="1200"/>
  </cs:seriesAxis>
  <cs:seriesLine>
    <cs:lnRef idx="0"/>
    <cs:fillRef idx="0"/>
    <cs:effectRef idx="0"/>
    <cs:fontRef idx="minor">
      <a:schemeClr val="tx1"/>
    </cs:fontRef>
    <cs:spPr>
      <a:ln w="9525" cap="flat" cmpd="sng" algn="ctr">
        <a:solidFill>
          <a:schemeClr val="tx1">
            <a:lumMod val="35000"/>
            <a:lumOff val="65000"/>
          </a:schemeClr>
        </a:solidFill>
        <a:round/>
      </a:ln>
      <a:effectLst/>
    </cs:spPr>
  </cs:seriesLine>
  <cs:title>
    <cs:lnRef idx="0"/>
    <cs:fillRef idx="0"/>
    <cs:effectRef idx="0"/>
    <cs:fontRef idx="minor">
      <a:schemeClr val="tx1">
        <a:lumMod val="75000"/>
        <a:lumOff val="25000"/>
      </a:schemeClr>
    </cs:fontRef>
    <cs:defRPr sz="1400" b="1" kern="1200" baseline="0"/>
  </cs:title>
  <cs:trendline>
    <cs:lnRef idx="0">
      <cs:styleClr val="auto"/>
    </cs:lnRef>
    <cs:fillRef idx="0"/>
    <cs:effectRef idx="0"/>
    <cs:fontRef idx="minor">
      <a:schemeClr val="tx1"/>
    </cs:fontRef>
    <cs:spPr>
      <a:ln w="19050" cap="rnd">
        <a:solidFill>
          <a:schemeClr val="phClr"/>
        </a:solidFill>
        <a:prstDash val="sysDot"/>
      </a:ln>
      <a:effectLst/>
    </cs:spPr>
  </cs:trendline>
  <cs:trendlineLabel>
    <cs:lnRef idx="0"/>
    <cs:fillRef idx="0"/>
    <cs:effectRef idx="0"/>
    <cs:fontRef idx="minor">
      <a:schemeClr val="tx1">
        <a:lumMod val="65000"/>
        <a:lumOff val="35000"/>
      </a:schemeClr>
    </cs:fontRef>
    <cs:defRPr sz="900" kern="1200"/>
  </cs:trendlineLabel>
  <cs:upBar>
    <cs:lnRef idx="0"/>
    <cs:fillRef idx="0"/>
    <cs:effectRef idx="0"/>
    <cs:fontRef idx="minor">
      <a:schemeClr val="tx1"/>
    </cs:fontRef>
    <cs:spPr>
      <a:solidFill>
        <a:schemeClr val="lt1"/>
      </a:solidFill>
      <a:ln w="9525" cap="flat" cmpd="sng" algn="ctr">
        <a:solidFill>
          <a:schemeClr val="tx1">
            <a:lumMod val="65000"/>
            <a:lumOff val="35000"/>
          </a:schemeClr>
        </a:solidFill>
        <a:round/>
      </a:ln>
      <a:effectLst/>
    </cs:spPr>
  </cs:upBar>
  <cs:valueAxis>
    <cs:lnRef idx="0"/>
    <cs:fillRef idx="0"/>
    <cs:effectRef idx="0"/>
    <cs:fontRef idx="minor">
      <a:schemeClr val="tx1">
        <a:lumMod val="65000"/>
        <a:lumOff val="35000"/>
      </a:schemeClr>
    </cs:fontRef>
    <cs:defRPr sz="900" kern="1200"/>
  </cs:valueAxis>
  <cs:wall>
    <cs:lnRef idx="0"/>
    <cs:fillRef idx="0"/>
    <cs:effectRef idx="0"/>
    <cs:fontRef idx="minor">
      <a:schemeClr val="tx1"/>
    </cs:fontRef>
    <cs:spPr>
      <a:noFill/>
      <a:ln>
        <a:noFill/>
      </a:ln>
    </cs:spPr>
  </cs:wall>
</cs:chartStyle>"#)
}

/// Generate chart colors XML content
pub fn generate_chart_colors_xml(_chart: &Chart) -> String {
    format!(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<cs:colorStyle xmlns:cs="http://schemas.microsoft.com/office/drawing/2012/chartStyle" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" meth="cycle" id="10">
  <a:schemeClr val="accent1"/>
  <a:schemeClr val="accent2"/>
  <a:schemeClr val="accent3"/>
  <a:schemeClr val="accent4"/>
  <a:schemeClr val="accent5"/>
  <a:schemeClr val="accent6"/>
  <cs:variation/>
  <cs:variation>
    <a:lumMod val="60000"/>
  </cs:variation>
  <cs:variation>
    <a:lumMod val="80000"/>
    <a:lumOff val="20000"/>
  </cs:variation>
  <cs:variation>
    <a:lumMod val="80000"/>
  </cs:variation>
  <cs:variation>
    <a:lumMod val="60000"/>
    <a:lumOff val="40000"/>
  </cs:variation>
  <cs:variation>
    <a:lumMod val="50000"/>
  </cs:variation>
  <cs:variation>
    <a:lumMod val="70000"/>
    <a:lumOff val="30000"/>
  </cs:variation>
  <cs:variation>
    <a:lumMod val="70000"/>
  </cs:variation>
  <cs:variation>
    <a:lumMod val="50000"/>
    <a:lumOff val="50000"/>
  </cs:variation>
</cs:colorStyle>"#)
}