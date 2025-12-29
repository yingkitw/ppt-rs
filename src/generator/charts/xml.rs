//! Chart XML generation

use super::types::ChartType;
use super::data::Chart;
use super::escape_xml;
use super::excel::{get_excel_writer, get_excel_writer_with_name, worksheet_name_for_chart};

/// Generate chart XML for a slide
pub fn generate_chart_xml(chart: &Chart, shape_id: usize) -> String {
    generate_chart_xml_with_number(chart, shape_id, 1)
}

/// Generate chart XML for a slide with specific chart number for worksheet naming
pub fn generate_chart_xml_with_number(chart: &Chart, shape_id: usize, chart_number: usize) -> String {
    match chart.chart_type {
        ChartType::Bar | ChartType::BarHorizontal | ChartType::BarStacked | ChartType::BarStacked100 => {
            generate_bar_chart_xml_with_number(chart, shape_id, chart_number)
        }
        ChartType::Line | ChartType::LineMarkers | ChartType::LineStacked => {
            generate_line_chart_xml_with_number(chart, shape_id, chart_number)
        }
        ChartType::Pie => generate_pie_chart_xml_with_number(chart, shape_id, chart_number),
        ChartType::Doughnut => generate_doughnut_chart_xml(chart, shape_id),
        ChartType::Area | ChartType::AreaStacked | ChartType::AreaStacked100 => {
            generate_area_chart_xml_with_number(chart, shape_id, chart_number)
        }
        ChartType::Scatter | ChartType::ScatterLines | ChartType::ScatterSmooth => {
            generate_scatter_chart_xml_with_number(chart, shape_id, chart_number)
        }
        ChartType::Bubble => generate_bubble_chart_xml_with_number(chart, shape_id, chart_number),
        ChartType::Radar | ChartType::RadarFilled => generate_radar_chart_xml_with_number(chart, shape_id, chart_number),
        ChartType::StockHLC | ChartType::StockOHLC => generate_stock_chart_xml_with_number(chart, shape_id, chart_number),
        ChartType::Combo => generate_combo_chart_xml_with_number(chart, shape_id, chart_number),
    }
}

/// Generate chart frame reference for slide XML (only the frame with relationship reference)
/// This follows the python-pptx pattern where the slide contains only a reference to the chart data
pub fn generate_chart_frame_xml(chart: &Chart, shape_id: usize, relationship_id: &str) -> String {
    format!(
        r#"<p:graphicFrame>
<p:nvGraphicFramePr>
<p:cNvPr id="{}" name="Chart {}"/>
<p:cNvGraphicFramePr/>
<p:nvPr/>
</p:nvGraphicFramePr>
<p:xfrm>
<a:off x="{}" y="{}"/>
<a:ext cx="{}" cy="{}"/>
</p:xfrm>
<a:graphic>
<a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/chart">
<c:chart xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart" r:id="{}"/>
</a:graphicData>
</a:graphic>
</p:graphicFrame>"#,
        shape_id, shape_id, chart.x, chart.y, chart.width, chart.height, relationship_id
    )
}

/// Generate chart data XML for separate chart file (chart content without frame)
/// This follows the python-pptx pattern where chart data is stored in separate files
pub fn generate_chart_data_xml(chart: &Chart) -> String {
    match chart.chart_type {
        ChartType::Bar | ChartType::BarHorizontal | ChartType::BarStacked | ChartType::BarStacked100 => {
            generate_bar_chart_data_xml(chart)
        }
        ChartType::Line | ChartType::LineMarkers | ChartType::LineStacked => {
            generate_line_chart_data_xml(chart)
        }
        ChartType::Pie => generate_pie_chart_data_xml(chart),
        ChartType::Doughnut => generate_doughnut_chart_data_xml(chart),
        ChartType::Area | ChartType::AreaStacked | ChartType::AreaStacked100 => {
            generate_area_chart_data_xml(chart)
        }
        ChartType::Scatter | ChartType::ScatterLines | ChartType::ScatterSmooth => {
            generate_scatter_chart_data_xml(chart)
        }
        ChartType::Bubble => generate_bubble_chart_data_xml(chart),
        ChartType::Radar | ChartType::RadarFilled => generate_radar_chart_data_xml(chart),
        ChartType::StockHLC | ChartType::StockOHLC => generate_stock_chart_data_xml(chart),
        ChartType::Combo => generate_combo_chart_data_xml(chart),
    }
}

/// Generate the common chart frame header
fn chart_frame_header(chart: &Chart, shape_id: usize) -> String {
    format!(
        r#"<p:graphicFrame>
<p:nvGraphicFramePr>
<p:cNvPr id="{}" name="Chart {}"/>
<p:cNvGraphicFramePr/>
<p:nvPr/>
</p:nvGraphicFramePr>
<p:xfrm>
<a:off x="{}" y="{}"/>
<a:ext cx="{}" cy="{}"/>
</p:xfrm>
<a:graphic>
<a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/chart">
<c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
<c:nvChartSpPr>
<c:cNvPr id="1" name="Chart"/>
<c:cNvChartSpPr/>
<c:nvPr/>
</c:nvChartSpPr>
<c:chartSpace>
<c:chart>
<c:title>
<c:tx>
<c:rich>
<a:bodyPr/>
<a:lstStyle/>
<a:p>
<a:r>
<a:rPr lang="en-US" sz="1800"/>
<a:t>{}</a:t>
</a:r>
</a:p>
</c:rich>
</c:tx>
</c:title>
<c:plotArea>
<c:layout/>"#,
        shape_id, shape_id, chart.x, chart.y, chart.width, chart.height, escape_xml(&chart.title)
    )
}

/// Generate the common chart frame footer with external data reference
fn chart_frame_footer(relationship_id: Option<&str>) -> String {
    let mut xml = String::from(r#"</c:plotArea>
<c:legend>
<c:legendPos val="r"/>
<c:overlay val="0"/>
</c:legend>
<c:plotVisOnly val="1"/>
"#);
    
    // Add external data reference if provided
    if let Some(rid) = relationship_id {
        xml.push_str(&format!(
            r#"<c:externalData r:id="{}" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
<c:autoUpdate val="0"/>
</c:externalData>
"#,
            rid
        ));
    }
    
    xml.push_str(r#"</c:chart>
</c:chartSpace>
</a:graphicData>
</a:graphic>
</p:graphicFrame>"#);
    
    xml
}

/// Generate series data XML using Excel references
fn generate_series_data(chart: &Chart, idx: usize, series_name: &str, values: &[f64]) -> String {
    generate_series_data_with_number(chart, idx, series_name, values, 1)
}

/// Generate series data XML using Excel references with specific chart number
fn generate_series_data_with_number(chart: &Chart, idx: usize, series_name: &str, values: &[f64], chart_number: usize) -> String {
    let worksheet_name = worksheet_name_for_chart(chart_number);
    let excel_writer = get_excel_writer_with_name(&chart.chart_type, worksheet_name);
    let values_ref = excel_writer.values_ref(idx);
    let name_ref = excel_writer.series_name_ref(idx);
    
    let mut xml = format!(
        r#"
<c:ser>
<c:idx val="{}"/>
<c:order val="{}"/>
<c:title>
<c:tx>
<c:strRef>
<c:f>{}</c:f>
<c:strCache>
<c:ptCount val="1"/>
<c:pt idx="0">
<c:v>{}</c:v>
</c:pt>
</c:strCache>
</c:strRef>
</c:tx>
</c:title>
<c:dLbls>
<c:showVal val="0"/>
</c:dLbls>
<c:val>
<c:numRef>
<c:f>{}</c:f>
<c:numCache>
<c:formatCode>0</c:formatCode>
<c:ptCount val="{}"/>"#,
        idx, idx, name_ref, escape_xml(series_name), values_ref, values.len()
    );

    for (i, value) in values.iter().enumerate() {
        xml.push_str(&format!(
            r#"
<c:pt idx="{}">
<c:v>{}</c:v>
</c:pt>"#,
            i, value
        ));
    }

    xml.push_str(
        r#"
</c:numCache>
</c:numRef>
</c:val>
</c:ser>"#
    );

    xml
}

/// Generate category axis XML using Excel references
fn generate_category_axis(chart: &Chart, ax_pos: &str) -> String {
    generate_category_axis_with_number(chart, ax_pos, 1)
}

/// Generate category axis XML using Excel references with specific chart number
fn generate_category_axis_with_number(chart: &Chart, ax_pos: &str, chart_number: usize) -> String {
    let worksheet_name = worksheet_name_for_chart(chart_number);
    let excel_writer = get_excel_writer_with_name(&chart.chart_type, worksheet_name);
    let categories_ref = excel_writer.categories_ref();
    
    let mut xml = format!(
        r#"
<c:catAx>
<c:axId val="1"/>
<c:scaling>
<c:orientation val="minMax"/>
</c:scaling>
<c:delete val="0"/>
<c:axPos val="{}"/>
<c:majorGridlines/>
<c:numFmt formatCode="0" sourceLinked="1"/>
<c:tickLblPos val="low"/>
<c:crossAx val="2"/>
<c:crosses val="autoZero"/>
<c:strRef>
<c:f>{}</c:f>
<c:strCache>
<c:ptCount val="{}"/>"#,
        ax_pos, categories_ref, chart.category_count()
    );

    for (idx, cat) in chart.categories.iter().enumerate() {
        xml.push_str(&format!(
            r#"
<c:pt idx="{}">
<c:v>{}</c:v>
</c:pt>"#,
            idx, escape_xml(cat)
        ));
    }

    xml.push_str(
        r#"
</c:strCache>
</c:strRef>
</c:catAx>"#
    );

    xml
}

/// Generate value axis XML
fn generate_value_axis(ax_pos: &str) -> String {
    format!(
        r#"
<c:valAx>
<c:axId val="2"/>
<c:scaling>
<c:orientation val="minMax"/>
</c:scaling>
<c:delete val="0"/>
<c:axPos val="{}"/>
<c:majorGridlines/>
<c:numFmt formatCode="0" sourceLinked="1"/>
<c:tickLblPos val="low"/>
<c:crossAx val="1"/>
<c:crosses val="autoZero"/>
</c:valAx>"#,
        ax_pos
    )
}

/// Generate bar chart XML
fn generate_bar_chart_xml(chart: &Chart, shape_id: usize) -> String {
    generate_bar_chart_xml_with_number(chart, shape_id, 1)
}

fn generate_bar_chart_xml_with_number(chart: &Chart, shape_id: usize, chart_number: usize) -> String {
    let mut xml = chart_frame_header(chart, shape_id);
    
    xml.push_str(r#"<c:barChart>
<c:barDir val="bar"/>
<c:grouping val="clustered"/>"#);

    for (idx, series) in chart.series.iter().enumerate() {
        xml.push_str(&generate_series_data_with_number(chart, idx, &series.name, &series.values, chart_number));
    }

    xml.push_str(&generate_category_axis_with_number(chart, "l", chart_number));
    xml.push_str(&generate_value_axis("b"));
    xml.push_str("</c:barChart>");
    xml.push_str(&chart_frame_footer(Some("rId1")));

    xml
}

/// Generate line chart XML
fn generate_line_chart_xml(chart: &Chart, shape_id: usize) -> String {
    generate_line_chart_xml_with_number(chart, shape_id, 1)
}

fn generate_line_chart_xml_with_number(chart: &Chart, shape_id: usize, chart_number: usize) -> String {
    let mut xml = chart_frame_header(chart, shape_id);
    
    xml.push_str(r#"<c:lineChart>
<c:grouping val="lineMarkers"/>"#);

    for (idx, series) in chart.series.iter().enumerate() {
        xml.push_str(&generate_series_data_with_number(chart, idx, &series.name, &series.values, chart_number));
    }

    xml.push_str(&generate_category_axis_with_number(chart, "b", chart_number));
    xml.push_str(&generate_value_axis("l"));
    xml.push_str("</c:lineChart>");
    xml.push_str(&chart_frame_footer(Some("rId1")));

    xml
}

/// Generate pie chart XML using Excel references
fn generate_pie_chart_xml(chart: &Chart, shape_id: usize) -> String {
    generate_pie_chart_xml_with_number(chart, shape_id, 1)
}

fn generate_pie_chart_xml_with_number(chart: &Chart, shape_id: usize, chart_number: usize) -> String {
    let mut xml = chart_frame_header(chart, shape_id);
    
    xml.push_str(r#"<c:pieChart>
<c:varyColors val="1"/>"#);

    // Pie chart uses first series only
    if let Some(series) = chart.series.first() {
        let worksheet_name = worksheet_name_for_chart(chart_number);
        let excel_writer = get_excel_writer_with_name(&chart.chart_type, worksheet_name);
        let values_ref = excel_writer.values_ref(0);
        let categories_ref = excel_writer.categories_ref();
        
        xml.push_str(&format!(
            r#"
<c:ser>
<c:idx val="0"/>
<c:order val="0"/>
<c:tx>
<c:strRef>
<c:f>{}</c:f>
<c:strCache>
<c:ptCount val="1"/>
<c:pt idx="0"><c:v>{}</c:v></c:pt>
</c:strCache>
</c:strRef>
</c:tx>
<c:dLbls>
<c:showCatName val="1"/>
<c:showPercent val="1"/>
</c:dLbls>
<c:val>
<c:numRef>
<c:f>{}</c:f>
<c:numCache>
<c:formatCode>0</c:formatCode>
<c:ptCount val="{}"/>"#,
            excel_writer.series_name_ref(0),
            escape_xml(&series.name),
            values_ref,
            series.values.len()
        ));

        for (idx, value) in series.values.iter().enumerate() {
            xml.push_str(&format!(
                r#"
<c:pt idx="{}">
<c:v>{}</c:v>
</c:pt>"#,
                idx, value
            ));
        }

        xml.push_str(&format!(
            r#"
</c:numCache>
</c:numRef>
</c:val>
<c:cat>
<c:strRef>
<c:f>{}</c:f>
<c:strCache>
<c:ptCount val="{}"/>"#,
            categories_ref,
            chart.category_count()
        ));

        for (idx, cat) in chart.categories.iter().enumerate() {
            xml.push_str(&format!(
                r#"
<c:pt idx="{}">
<c:v>{}</c:v>
</c:pt>"#,
                idx, escape_xml(cat)
            ));
        }

        xml.push_str(
            r#"
</c:strCache>
</c:strRef>
</c:cat>
</c:ser>"#
        );
    }

    xml.push_str("</c:pieChart>");
    xml.push_str(&chart_frame_footer(Some("rId1")));

    xml
}

/// Generate doughnut chart XML
fn generate_doughnut_chart_xml(chart: &Chart, shape_id: usize) -> String {
    let mut xml = chart_frame_header(chart, shape_id);
    
    xml.push_str(r#"<c:doughnutChart>
<c:varyColors val="1"/>
<c:holeSize val="50"/>"#);

    // Doughnut chart uses first series only (like pie)
    if let Some(series) = chart.series.first() {
        let excel_writer = get_excel_writer(&chart.chart_type);
        let values_ref = excel_writer.values_ref(0);
        let categories_ref = excel_writer.categories_ref();
        
        xml.push_str(&format!(
            r#"
<c:ser>
<c:idx val="0"/>
<c:order val="0"/>
<c:tx>
<c:strRef>
<c:f>{}</c:f>
<c:strCache>
<c:ptCount val="1"/>
<c:pt idx="0"><c:v>{}</c:v></c:pt>
</c:strCache>
</c:strRef>
</c:tx>
<c:dLbls>
<c:showCatName val="1"/>
<c:showPercent val="1"/>
</c:dLbls>
<c:val>
<c:numRef>
<c:f>{}</c:f>
<c:numCache>
<c:formatCode>0</c:formatCode>"#,
            excel_writer.series_name_ref(0),
            escape_xml(&series.name),
            values_ref
        ));

        for (idx, value) in series.values.iter().enumerate() {
            xml.push_str(&format!(
                r#"
<c:pt idx="{}">
<c:v>{}</c:v>
</c:pt>"#,
                idx, value
            ));
        }

        xml.push_str(&format!(
            r#"
</c:numCache>
</c:numRef>
</c:val>
<c:cat>
<c:strRef>
<c:f>{}</c:f>
<c:strCache>
<c:ptCount val="{}"/>"#,
            categories_ref,
            chart.category_count()
        ));

        for (idx, cat) in chart.categories.iter().enumerate() {
            xml.push_str(&format!(
                r#"
<c:pt idx="{}">
<c:v>{}</c:v>
</c:pt>"#,
                idx, escape_xml(cat)
            ));
        }

        xml.push_str(
            r#"
</c:strCache>
</c:strRef>
</c:cat>
</c:ser>"#
        );
    }

    xml.push_str("</c:doughnutChart>");
    xml.push_str(&chart_frame_footer(Some("rId1")));

    xml
}

/// Generate area chart XML
fn generate_area_chart_xml(chart: &Chart, shape_id: usize) -> String {
    generate_area_chart_xml_with_number(chart, shape_id, 1)
}

fn generate_area_chart_xml_with_number(chart: &Chart, shape_id: usize, chart_number: usize) -> String {
    let mut xml = chart_frame_header(chart, shape_id);
    
    let grouping = chart.chart_type.grouping().unwrap_or("standard");
    xml.push_str(&format!(r#"<c:areaChart>
<c:grouping val="{}"/>"#, grouping));

    for (idx, series) in chart.series.iter().enumerate() {
        xml.push_str(&generate_series_data_with_number(chart, idx, &series.name, &series.values, chart_number));
    }

    xml.push_str(&generate_category_axis_with_number(chart, "b", chart_number));
    xml.push_str(&generate_value_axis("l"));
    xml.push_str("</c:areaChart>");
    xml.push_str(&chart_frame_footer(Some("rId1")));

    xml
}

/// Generate scatter chart XML
fn generate_scatter_chart_xml(chart: &Chart, shape_id: usize) -> String {
    generate_scatter_chart_xml_with_number(chart, shape_id, 1)
}

fn generate_scatter_chart_xml_with_number(chart: &Chart, shape_id: usize, chart_number: usize) -> String {
    let mut xml = chart_frame_header(chart, shape_id);
    
    let scatter_style = chart.chart_type.scatter_style().unwrap_or("lineMarker");
    xml.push_str(&format!(r#"<c:scatterChart>
<c:scatterStyle val="{}"/>"#, scatter_style));

    for (idx, series) in chart.series.iter().enumerate() {
        let worksheet_name = worksheet_name_for_chart(chart_number);
        let excel_writer = get_excel_writer_with_name(&chart.chart_type, worksheet_name);
        let values_ref = excel_writer.values_ref(idx);
        let categories_ref = excel_writer.categories_ref();
        
        xml.push_str(&format!(
            r#"
<c:ser>
<c:idx val="{}"/>
<c:order val="{}"/>
<c:tx>
<c:strRef>
<c:f>{}</c:f>
<c:strCache>
<c:ptCount val="1"/>
<c:pt idx="0"><c:v>{}</c:v></c:pt>
</c:strCache>
</c:strRef>
</c:tx>
<c:xVal>
<c:numRef>
<c:f>{}</c:f>
<c:numCache>
<c:formatCode>0</c:formatCode>"#,
            idx, idx, excel_writer.series_name_ref(idx), escape_xml(&series.name), categories_ref
        ));

        // X values (use index as X)
        for (i, _) in series.values.iter().enumerate() {
            xml.push_str(&format!(
                r#"
<c:pt idx="{}">
<c:v>{}</c:v>
</c:pt>"#,
                i, i + 1
            ));
        }

        xml.push_str(&format!(
            r#"
</c:numCache>
</c:numRef>
</c:xVal>
<c:yVal>
<c:numRef>
<c:f>{}</c:f>
<c:numCache>
<c:formatCode>0</c:formatCode>"#,
            values_ref
        ));

        for (i, value) in series.values.iter().enumerate() {
            xml.push_str(&format!(
                r#"
<c:pt idx="{}">
<c:v>{}</c:v>
</c:pt>"#,
                i, value
            ));
        }

        xml.push_str(
            r#"
</c:numCache>
</c:numRef>
</c:yVal>
</c:ser>"#
        );
    }

    xml.push_str(&generate_value_axis("b"));
    xml.push_str(&generate_value_axis("l"));
    xml.push_str("</c:scatterChart>");
    xml.push_str(&chart_frame_footer(Some("rId1")));

    xml
}

/// Generate bubble chart XML
fn generate_bubble_chart_xml(chart: &Chart, shape_id: usize) -> String {
    generate_bubble_chart_xml_with_number(chart, shape_id, 1)
}

fn generate_bubble_chart_xml_with_number(chart: &Chart, shape_id: usize, chart_number: usize) -> String {
    let mut xml = chart_frame_header(chart, shape_id);
    
    xml.push_str(r#"<c:bubbleChart>
<c:varyColors val="0"/>
<c:bubbleScale val="100"/>"#);

    for (idx, series) in chart.series.iter().enumerate() {
        let worksheet_name = worksheet_name_for_chart(chart_number);
        let excel_writer = get_excel_writer_with_name(&chart.chart_type, worksheet_name);
        let values_ref = excel_writer.values_ref(idx);
        let categories_ref = excel_writer.categories_ref();
        
        xml.push_str(&format!(
            r#"
<c:ser>
<c:idx val="{}"/>
<c:order val="{}"/>
<c:tx>
<c:strRef>
<c:f>{}</c:f>
<c:strCache>
<c:ptCount val="1"/>
<c:pt idx="0"><c:v>{}</c:v></c:pt>
</c:strCache>
</c:strRef>
</c:tx>
<c:xVal>
<c:numRef>
<c:f>{}</c:f>
<c:numCache>
<c:formatCode>0</c:formatCode>"#,
            idx, idx, excel_writer.series_name_ref(idx), escape_xml(&series.name), categories_ref
        ));

        for (i, _) in series.values.iter().enumerate() {
            xml.push_str(&format!(
                r#"
<c:pt idx="{}">
<c:v>{}</c:v>
</c:pt>"#,
                i, i + 1
            ));
        }

        xml.push_str(&format!(
            r#"
</c:numCache>
</c:numRef>
</c:xVal>
<c:yVal>
<c:numRef>
<c:f>{}</c:f>
<c:numCache>
<c:formatCode>0</c:formatCode>"#,
            values_ref
        ));

        for (i, value) in series.values.iter().enumerate() {
            xml.push_str(&format!(
                r#"
<c:pt idx="{}">
<c:v>{}</c:v>
</c:pt>"#,
                i, value
            ));
        }

        // Use bubble sizes if available, otherwise use values
        let bubble_sizes_ref = if let Some(ref _bubble_sizes) = series.bubble_sizes {
            // For now, use the same values reference but this should be a separate column
            // TODO: Implement proper bubble size column in Excel writer
            values_ref.clone()
        } else {
            // Use values as bubble sizes (absolute values)
            values_ref.clone()
        };
        
        xml.push_str(&format!(
            r#"
</c:numCache>
</c:numRef>
</c:yVal>
<c:bubbleSize>
<c:numRef>
<c:f>{}</c:f>
<c:numCache>
<c:formatCode>0</c:formatCode>"#,
            bubble_sizes_ref
        ));

        // Bubble sizes (use values as sizes)
        for (i, value) in series.values.iter().enumerate() {
            xml.push_str(&format!(
                r#"
<c:pt idx="{}">
<c:v>{}</c:v>
</c:pt>"#,
                i, value.abs()
            ));
        }

        xml.push_str(
            r#"
</c:numCache>
</c:numRef>
</c:bubbleSize>
</c:ser>"#
        );
    }

    xml.push_str(&generate_value_axis("b"));
    xml.push_str(&generate_value_axis("l"));
    xml.push_str("</c:bubbleChart>");
    xml.push_str(&chart_frame_footer(Some("rId1")));

    xml
}

/// Generate radar chart XML
fn generate_radar_chart_xml(chart: &Chart, shape_id: usize) -> String {
    generate_radar_chart_xml_with_number(chart, shape_id, 1)
}

fn generate_radar_chart_xml_with_number(chart: &Chart, shape_id: usize, chart_number: usize) -> String {
    let mut xml = chart_frame_header(chart, shape_id);
    
    let radar_style = chart.chart_type.radar_style().unwrap_or("marker");
    xml.push_str(&format!(r#"<c:radarChart>
<c:radarStyle val="{}"/>"#, radar_style));

    for (idx, series) in chart.series.iter().enumerate() {
        xml.push_str(&generate_series_data_with_number(chart, idx, &series.name, &series.values, chart_number));
    }

    xml.push_str(&generate_category_axis_with_number(chart, "b", chart_number));
    xml.push_str(&generate_value_axis("l"));
    xml.push_str("</c:radarChart>");
    xml.push_str(&chart_frame_footer(Some("rId1")));

    xml
}

/// Generate stock chart XML
fn generate_stock_chart_xml(chart: &Chart, shape_id: usize) -> String {
    generate_stock_chart_xml_with_number(chart, shape_id, 1)
}

fn generate_stock_chart_xml_with_number(chart: &Chart, shape_id: usize, chart_number: usize) -> String {
    let mut xml = chart_frame_header(chart, shape_id);
    
    xml.push_str(r#"<c:stockChart>"#);

    // Stock charts need High, Low, Close (and optionally Open) series
    for (idx, series) in chart.series.iter().enumerate() {
        xml.push_str(&generate_series_data_with_number(chart, idx, &series.name, &series.values, chart_number));
    }

    xml.push_str(&generate_category_axis_with_number(chart, "b", chart_number));
    xml.push_str(&generate_value_axis("l"));
    xml.push_str("</c:stockChart>");
    xml.push_str(&chart_frame_footer(Some("rId1")));

    xml
}

/// Generate combo chart XML (bar + line)
fn generate_combo_chart_xml(chart: &Chart, shape_id: usize) -> String {
    generate_combo_chart_xml_with_number(chart, shape_id, 1)
}

fn generate_combo_chart_xml_with_number(chart: &Chart, shape_id: usize, chart_number: usize) -> String {
    let mut xml = chart_frame_header(chart, shape_id);
    
    // First half of series as bars
    xml.push_str(r#"<c:barChart>
<c:barDir val="col"/>
<c:grouping val="clustered"/>"#);

    let mid = chart.series.len() / 2;
    for (idx, series) in chart.series.iter().take(mid.max(1)).enumerate() {
        xml.push_str(&generate_series_data_with_number(chart, idx, &series.name, &series.values, chart_number));
    }

    xml.push_str(&generate_category_axis_with_number(chart, "b", chart_number));
    xml.push_str(&generate_value_axis("l"));
    xml.push_str("</c:barChart>");

    // Second half as lines
    if chart.series.len() > 1 {
        xml.push_str(r#"<c:lineChart>
<c:grouping val="standard"/>"#);

        for (idx, series) in chart.series.iter().skip(mid.max(1)).enumerate() {
            xml.push_str(&generate_series_data_with_number(chart, mid + idx, &series.name, &series.values, chart_number));
        }

        xml.push_str("</c:lineChart>");
    }

    xml.push_str(&chart_frame_footer(Some("rId1")));

    xml
}

/// Generate chart data XML header (common for all chart data files)
fn chart_data_header(chart: &Chart) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
<c:date1904 val="0"/>
<c:lang val="en-US"/>
<c:roundedCorners val="0"/>
<c:chart>
<c:autoTitleDeleted val="0"/>
<c:title>
<c:tx>
<c:rich>
<a:bodyPr/>
<a:lstStyle/>
<a:p>
<a:r>
<a:rPr lang="en-US" sz="1800"/>
<a:t>{}</a:t>
</a:r>
</a:p>
</c:rich>
</c:tx>
</c:title>
<c:plotArea>
<c:layout/>"#,
        escape_xml(&chart.title)
    )
}

/// Generate chart data XML footer (common for all chart data files)
fn chart_data_footer(relationship_id: Option<&str>) -> String {
    let mut xml = String::from(r#"</c:plotArea>
<c:legend>
<c:legendPos val="r"/>
<c:overlay val="0"/>
</c:legend>
<c:plotVisOnly val="1"/>
"#);
    
    // Add external data reference if provided
    if let Some(rid) = relationship_id {
        xml.push_str(&format!(
            r#"<c:externalData r:id="{}" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
<c:autoUpdate val="0"/>
</c:externalData>
"#,
            rid
        ));
    }
    
    xml.push_str(r#"</c:chart>
</c:chartSpace>"#);
    
    xml
}

/// Generate bar chart data XML (for external chart file)
fn generate_bar_chart_data_xml(chart: &Chart) -> String {
    let mut xml = chart_data_header(chart);
    
    let bar_dir = if matches!(chart.chart_type, ChartType::BarHorizontal) { "bar" } else { "col" };
    let grouping = chart.chart_type.grouping().unwrap_or("clustered");
    
    xml.push_str(&format!(
        r#"<c:barChart>
<c:barDir val="{}"/>
<c:grouping val="{}"/>"#,
        bar_dir, grouping
    ));

    for (idx, series) in chart.series.iter().enumerate() {
        xml.push_str(&generate_series_data_for_chart(chart, idx, &series.name, &series.values));
    }

    xml.push_str(&generate_category_axis_for_chart(chart, "l"));
    xml.push_str(&generate_value_axis_for_chart("b"));
    xml.push_str("</c:barChart>");
    xml.push_str(&chart_data_footer(Some("rId1")));

    xml
}

/// Generate line chart data XML (for external chart file)
fn generate_line_chart_data_xml(chart: &Chart) -> String {
    let mut xml = chart_data_header(chart);
    
    let grouping = chart.chart_type.grouping().unwrap_or("lineMarkers");
    
    xml.push_str(&format!(
        r#"<c:lineChart>
<c:grouping val="{}"/>"#,
        grouping
    ));

    for (idx, series) in chart.series.iter().enumerate() {
        xml.push_str(&generate_series_data_for_chart(chart, idx, &series.name, &series.values));
    }

    xml.push_str(&generate_category_axis_for_chart(chart, "b"));
    xml.push_str(&generate_value_axis_for_chart("l"));
    xml.push_str("</c:lineChart>");
    xml.push_str(&chart_data_footer(Some("rId1")));

    xml
}

/// Generate pie chart data XML (for external chart file)
fn generate_pie_chart_data_xml(chart: &Chart) -> String {
    let mut xml = chart_data_header(chart);
    
    xml.push_str(r#"<c:pieChart>
<c:varyColors val="1"/>"#);

    // Pie chart uses first series only
    if let Some(series) = chart.series.first() {
        xml.push_str(&generate_series_data_for_chart(chart, 0, &series.name, &series.values));
    }

    xml.push_str("</c:pieChart>");
    xml.push_str(&chart_data_footer(Some("rId1")));

    xml
}

/// Generate doughnut chart data XML (for external chart file)
fn generate_doughnut_chart_data_xml(chart: &Chart) -> String {
    let mut xml = chart_data_header(chart);
    
    xml.push_str(r#"<c:doughnutChart>
<c:varyColors val="1"/>
<c:holeSize val="50"/>"#);

    // Doughnut chart uses first series only (like pie)
    if let Some(series) = chart.series.first() {
        xml.push_str(&generate_series_data_for_chart(chart, 0, &series.name, &series.values));
    }

    xml.push_str("</c:doughnutChart>");
    xml.push_str(&chart_data_footer(Some("rId1")));

    xml
}

/// Generate area chart data XML (for external chart file)
fn generate_area_chart_data_xml(chart: &Chart) -> String {
    let mut xml = chart_data_header(chart);
    
    let grouping = chart.chart_type.grouping().unwrap_or("standard");
    
    xml.push_str(&format!(
        r#"<c:areaChart>
<c:grouping val="{}"/>"#,
        grouping
    ));

    for (idx, series) in chart.series.iter().enumerate() {
        xml.push_str(&generate_series_data_for_chart(chart, idx, &series.name, &series.values));
    }

    xml.push_str(&generate_category_axis_for_chart(chart, "b"));
    xml.push_str(&generate_value_axis_for_chart("l"));
    xml.push_str("</c:areaChart>");
    xml.push_str(&chart_data_footer(Some("rId1")));

    xml
}

/// Generate scatter chart data XML (for external chart file)
fn generate_scatter_chart_data_xml(chart: &Chart) -> String {
    let mut xml = chart_data_header(chart);
    
    let scatter_style = chart.chart_type.scatter_style().unwrap_or("lineMarker");
    
    xml.push_str(&format!(
        r#"<c:scatterChart>
<c:scatterStyle val="{}"/>"#,
        scatter_style
    ));

    for (idx, series) in chart.series.iter().enumerate() {
        xml.push_str(&generate_series_data_for_scatter(chart, idx, &series.name, &series.values));
    }

    xml.push_str(&generate_value_axis_for_chart("b"));
    xml.push_str(&generate_value_axis_for_chart("l"));
    xml.push_str("</c:scatterChart>");
    xml.push_str(&chart_data_footer(Some("rId1")));

    xml
}

/// Generate bubble chart data XML (for external chart file)
fn generate_bubble_chart_data_xml(chart: &Chart) -> String {
    let mut xml = chart_data_header(chart);
    
    xml.push_str(r#"<c:bubbleChart>
<c:varyColors val="0"/>
<c:bubbleScale val="100"/>"#);

    for (idx, series) in chart.series.iter().enumerate() {
        xml.push_str(&generate_series_data_for_bubble(chart, idx, &series.name, &series.values));
    }

    xml.push_str(&generate_value_axis_for_chart("b"));
    xml.push_str(&generate_value_axis_for_chart("l"));
    xml.push_str("</c:bubbleChart>");
    xml.push_str(&chart_data_footer(Some("rId1")));

    xml
}

/// Generate radar chart data XML (for external chart file)
fn generate_radar_chart_data_xml(chart: &Chart) -> String {
    let mut xml = chart_data_header(chart);
    
    let radar_style = chart.chart_type.radar_style().unwrap_or("marker");
    
    xml.push_str(&format!(
        r#"<c:radarChart>
<c:radarStyle val="{}"/>"#,
        radar_style
    ));

    for (idx, series) in chart.series.iter().enumerate() {
        xml.push_str(&generate_series_data_for_chart(chart, idx, &series.name, &series.values));
    }

    xml.push_str(&generate_category_axis_for_chart(chart, "b"));
    xml.push_str(&generate_value_axis_for_chart("l"));
    xml.push_str("</c:radarChart>");
    xml.push_str(&chart_data_footer(Some("rId1")));

    xml
}

/// Generate stock chart data XML (for external chart file)
fn generate_stock_chart_data_xml(chart: &Chart) -> String {
    let mut xml = chart_data_header(chart);
    
    xml.push_str(r#"<c:stockChart>"#);

    // Stock charts need High, Low, Close (and optionally Open) series
    for (idx, series) in chart.series.iter().enumerate() {
        xml.push_str(&generate_series_data_for_chart(chart, idx, &series.name, &series.values));
    }

    xml.push_str(&generate_category_axis_for_chart(chart, "b"));
    xml.push_str(&generate_value_axis_for_chart("l"));
    xml.push_str("</c:stockChart>");
    xml.push_str(&chart_data_footer(Some("rId1")));

    xml
}

/// Generate combo chart data XML (for external chart file)
fn generate_combo_chart_data_xml(chart: &Chart) -> String {
    let mut xml = chart_data_header(chart);
    
    // First half of series as bars
    xml.push_str(r#"<c:barChart>
<c:barDir val="col"/>
<c:grouping val="clustered"/>"#);

    let mid = chart.series.len() / 2;
    for (idx, series) in chart.series.iter().take(mid.max(1)).enumerate() {
        xml.push_str(&generate_series_data_for_chart(chart, idx, &series.name, &series.values));
    }

    xml.push_str(&generate_category_axis_for_chart(chart, "b"));
    xml.push_str(&generate_value_axis_for_chart("l"));
    xml.push_str("</c:barChart>");

    // Second half as lines
    if chart.series.len() > 1 {
        xml.push_str(r#"<c:lineChart>
<c:grouping val="standard"/>"#);

        for (idx, series) in chart.series.iter().skip(mid.max(1)).enumerate() {
            xml.push_str(&generate_series_data_for_chart(chart, mid + idx, &series.name, &series.values));
        }

        xml.push_str("</c:lineChart>");
    }

    xml.push_str(&chart_data_footer(Some("rId1")));

    xml
}

/// Generate series data XML for chart data files
fn generate_series_data_for_chart(chart: &Chart, idx: usize, series_name: &str, values: &[f64]) -> String {
    let excel_writer = get_excel_writer(&chart.chart_type);
    let name_ref = excel_writer.series_name_ref(idx);
    let categories_ref = excel_writer.categories_ref();
    let values_ref = excel_writer.values_ref(idx);
    
    let mut xml = format!(
        r#"
<c:ser>
<c:idx val="{}"/>
<c:order val="{}"/>
<c:tx>
<c:strRef>
<c:f>{}</c:f>
<c:strCache>
<c:ptCount val="1"/>
<c:pt idx="0">
<c:v>{}</c:v>
</c:pt>
</c:strCache>
</c:strRef>
</c:tx>"#,
        idx, idx, name_ref, escape_xml(series_name)
    );

    // Add category data
    xml.push_str(&format!(
        r#"
<c:cat>
<c:strRef>
<c:f>{}</c:f>
<c:strCache>
<c:ptCount val="{}"/>"#,
        categories_ref, chart.category_count()
    ));

    for (cat_idx, cat) in chart.categories.iter().enumerate() {
        xml.push_str(&format!(
            r#"
<c:pt idx="{}">
<c:v>{}</c:v>
</c:pt>"#,
            cat_idx, escape_xml(cat)
        ));
    }

    xml.push_str(&format!(
        r#"
</c:strCache>
</c:strRef>
</c:cat>
<c:val>
<c:numRef>
<c:f>{}</c:f>
<c:numCache>
<c:formatCode>0</c:formatCode>
<c:ptCount val="{}"/>"#,
        values_ref, values.len()
    ));

    for (val_idx, value) in values.iter().enumerate() {
        xml.push_str(&format!(
            r#"
<c:pt idx="{}">
<c:v>{}</c:v>
</c:pt>"#,
            val_idx, value
        ));
    }

    xml.push_str(&format!(
        r#"
</c:numCache>
</c:numRef>
</c:val>
</c:ser>"#
    ));

    xml
}

/// Generate series data XML for scatter charts
fn generate_series_data_for_scatter(_chart: &Chart, idx: usize, series_name: &str, values: &[f64]) -> String {
    let excel_writer = get_excel_writer(&ChartType::Scatter);
    let name_ref = excel_writer.series_name_ref(idx);
    let categories_ref = excel_writer.categories_ref();
    let values_ref = excel_writer.values_ref(idx);
    
    let mut xml = format!(
        r#"
<c:ser>
<c:idx val="{}"/>
<c:order val="{}"/>
<c:tx>
<c:strRef>
<c:f>{}</c:f>
<c:strCache>
<c:ptCount val="1"/>
<c:pt idx="0">
<c:v>{}</c:v>
</c:pt>
</c:strCache>
</c:strRef>
</c:tx>"#,
        idx, idx, name_ref, escape_xml(series_name)
    );

    // X values (use index as X)
    xml.push_str(&format!(
        r#"
<c:xVal>
<c:numRef>
<c:f>{}</c:f>
<c:numCache>
<c:formatCode>0</c:formatCode>
<c:ptCount val="{}"/>"#,
        categories_ref, values.len()
    ));

    for (i, _) in values.iter().enumerate() {
        xml.push_str(&format!(
            r#"
<c:pt idx="{}">
<c:v>{}</c:v>
</c:pt>"#,
            i, i + 1
        ));
    }

    xml.push_str(&format!(
        r#"
</c:numCache>
</c:numRef>
</c:xVal>
<c:yVal>
<c:numRef>
<c:f>{}</c:f>
<c:numCache>
<c:formatCode>0</c:formatCode>
<c:ptCount val="{}"/>"#,
        values_ref, values.len()
    ));

    for (i, value) in values.iter().enumerate() {
        xml.push_str(&format!(
            r#"
<c:pt idx="{}">
<c:v>{}</c:v>
</c:pt>"#,
            i, value
        ));
    }

    xml.push_str(&format!(
        r#"
</c:numCache>
</c:numRef>
</c:yVal>
<c:smooth val="0"/>
</c:ser>"#
    ));

    xml
}

/// Generate series data XML for bubble charts
fn generate_series_data_for_bubble(_chart: &Chart, idx: usize, series_name: &str, values: &[f64]) -> String {
    let excel_writer = get_excel_writer(&ChartType::Bubble);
    let name_ref = excel_writer.series_name_ref(idx);
    let categories_ref = excel_writer.categories_ref();
    let values_ref = excel_writer.values_ref(idx);
    let bubble_sizes_ref = excel_writer.bubble_sizes_ref(idx);
    
    let mut xml = format!(
        r#"
<c:ser>
<c:idx val="{}"/>
<c:order val="{}"/>
<c:tx>
<c:strRef>
<c:f>{}</c:f>
<c:strCache>
<c:ptCount val="1"/>
<c:pt idx="0">
<c:v>{}</c:v>
</c:pt>
</c:strCache>
</c:strRef>
</c:tx>"#,
        idx, idx, name_ref, escape_xml(series_name)
    );

    // X values (use index as X)
    xml.push_str(&format!(
        r#"
<c:xVal>
<c:numRef>
<c:f>{}</c:f>
<c:numCache>
<c:formatCode>0</c:formatCode>
<c:ptCount val="{}"/>"#,
        categories_ref, values.len()
    ));

    for (i, _) in values.iter().enumerate() {
        xml.push_str(&format!(
            r#"
<c:pt idx="{}">
<c:v>{}</c:v>
</c:pt>"#,
            i, i + 1
        ));
    }

    xml.push_str(&format!(
        r#"
</c:numCache>
</c:numRef>
</c:xVal>
<c:yVal>
<c:numRef>
<c:f>{}</c:f>
<c:numCache>
<c:formatCode>0</c:formatCode>
<c:ptCount val="{}"/>"#,
        values_ref, values.len()
    ));

    for (i, value) in values.iter().enumerate() {
        xml.push_str(&format!(
            r#"
<c:pt idx="{}">
<c:v>{}</c:v>
</c:pt>"#,
            i, value
        ));
    }

    xml.push_str(&format!(
        r#"
</c:numCache>
</c:numRef>
</c:yVal>
<c:bubbleSize>
<c:numRef>
<c:f>{}</c:f>
<c:numCache>
<c:formatCode>0</c:formatCode>
<c:ptCount val="{}"/>"#,
        bubble_sizes_ref, values.len()
    ));

    // Bubble sizes (use values as sizes)
    for (i, value) in values.iter().enumerate() {
        xml.push_str(&format!(
            r#"
<c:pt idx="{}">
<c:v>{}</c:v>
</c:pt>"#,
            i, value.abs()
        ));
    }

    xml.push_str(&format!(
        r#"
</c:numCache>
</c:numRef>
</c:bubbleSize>
</c:ser>"#
    ));

    xml
}

/// Generate category axis XML for chart data files
fn generate_category_axis_for_chart(_chart: &Chart, ax_pos: &str) -> String {
    format!(
        r#"
<c:catAx>
<c:axId val="1"/>
<c:scaling>
<c:orientation val="minMax"/>
</c:scaling>
<c:delete val="0"/>
<c:axPos val="{}"/>
<c:majorGridlines/>
<c:numFmt formatCode="0" sourceLinked="0"/>
<c:tickLblPos val="low"/>
<c:crossAx val="2"/>
<c:crosses val="autoZero"/>
</c:catAx>"#,
        ax_pos
    )
}

/// Generate value axis XML for chart data files
fn generate_value_axis_for_chart(ax_pos: &str) -> String {
    format!(
        r#"
<c:valAx>
<c:axId val="2"/>
<c:scaling>
<c:orientation val="minMax"/>
</c:scaling>
<c:delete val="0"/>
<c:axPos val="{}"/>
<c:majorGridlines/>
<c:numFmt formatCode="0" sourceLinked="0"/>
<c:tickLblPos val="low"/>
<c:crossAx val="1"/>
<c:crosses val="autoZero"/>
</c:valAx>"#,
        ax_pos
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::charts::ChartSeries;

    #[test]
    fn test_generate_bar_chart_xml() {
        let chart = Chart::new(
            "Sales",
            ChartType::Bar,
            vec!["Q1".to_string(), "Q2".to_string()],
            0, 0, 5000000, 3750000,
        ).add_series(ChartSeries::new("2024", vec![100.0, 150.0]));

        let xml = generate_bar_chart_xml(&chart, 1);
        assert!(xml.contains("barChart"));
        assert!(xml.contains("Sales"));
    }

    #[test]
    fn test_generate_line_chart_xml() {
        let chart = Chart::new(
            "Trend",
            ChartType::Line,
            vec!["Jan".to_string(), "Feb".to_string()],
            0, 0, 5000000, 3750000,
        ).add_series(ChartSeries::new("Revenue", vec![1000.0, 1200.0]));

        let xml = generate_line_chart_xml(&chart, 1);
        assert!(xml.contains("lineChart"));
    }

    #[test]
    fn test_generate_pie_chart_xml() {
        let chart = Chart::new(
            "Distribution",
            ChartType::Pie,
            vec!["A".to_string(), "B".to_string()],
            0, 0, 5000000, 3750000,
        ).add_series(ChartSeries::new("Data", vec![30.0, 70.0]));

        let xml = generate_pie_chart_xml(&chart, 1);
        assert!(xml.contains("pieChart"));
    }
}
