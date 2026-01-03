//! Chart XML generation

use super::types::ChartType;
use super::data::Chart;
use super::escape_xml;

/// Generate chart XML content (for ppt/charts/chartN.xml)
pub fn generate_chart_part_xml(chart: &Chart) -> String {
    match chart.chart_type {
        ChartType::Bar | ChartType::BarHorizontal | ChartType::BarStacked | ChartType::BarStacked100 => {
            generate_bar_chart_xml(chart)
        }
        ChartType::Line | ChartType::LineMarkers | ChartType::LineStacked => {
            generate_line_chart_xml(chart)
        }
        ChartType::Pie => generate_pie_chart_xml(chart),
        ChartType::Doughnut => generate_doughnut_chart_xml(chart),
        ChartType::Area | ChartType::AreaStacked | ChartType::AreaStacked100 => {
            generate_area_chart_xml(chart)
        }
        ChartType::Scatter | ChartType::ScatterLines | ChartType::ScatterSmooth => {
            generate_scatter_chart_xml(chart)
        }
        ChartType::Bubble => generate_bubble_chart_xml(chart),
        ChartType::Radar | ChartType::RadarFilled => generate_radar_chart_xml(chart),
        ChartType::StockHLC | ChartType::StockOHLC => generate_stock_chart_xml(chart),
        ChartType::Combo => generate_combo_chart_xml(chart),
    }
}

/// Generate chart reference XML for slide (p:graphicFrame)
pub fn generate_chart_ref_xml(chart: &Chart, r_id: &str, shape_id: usize) -> String {
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
<c:chart xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" r:id="{}"/>
</a:graphicData>
</a:graphic>
</p:graphicFrame>"#,
        shape_id, shape_id, chart.x, chart.y, chart.width, chart.height, r_id
    )
}

/// Generate the chart part header
fn chart_part_header(chart: &Chart) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
<c:date1904 val="0"/>
<c:lang val="en-US"/>
<c:roundedCorners val="0"/>
<c:chart>
<c:title>
<c:tx>
<c:rich>
<a:bodyPr/>
<a:lstStyle/>
<a:p>
<a:pPr>
<a:defRPr/>
</a:pPr>
<a:r>
<a:rPr lang="en-US" sz="1800" b="0" i="0" u="none" strike="noStrike">
<a:solidFill>
<a:srgbClr val="595959"/>
</a:solidFill>
<a:latin typeface="Calibri"/>
</a:rPr>
<a:t>{}</a:t>
</a:r>
</a:p>
</c:rich>
</c:tx>
<c:layout/>
<c:overlay val="0"/>
</c:title>
<c:autoTitleDeleted val="0"/>
<c:plotArea>
<c:layout/>"#,
        escape_xml(&chart.title)
    )
}

/// Generate the chart part footer
fn chart_part_footer() -> &'static str {
    r#"</c:plotArea>
<c:legend>
<c:legendPos val="r"/>
<c:layout/>
<c:overlay val="0"/>
</c:legend>
<c:plotVisOnly val="1"/>
<c:dispBlanksAs val="gap"/>
<c:showDLblsOverMax val="0"/>
</c:chart>
</c:chartSpace>"#
}

/// Generate series data XML
fn generate_series_data(_chart: &Chart, idx: usize, series_name: &str, values: &[f64]) -> String {
    let mut xml = format!(
        r#"
<c:ser>
<c:idx val="{}"/>
<c:order val="{}"/>
<c:title>
<c:tx>
<c:rich>
<a:bodyPr/>
<a:lstStyle/>
<a:p>
<a:r>
<a:rPr lang="en-US" sz="1000"/>
<a:t>{}</a:t>
</a:r>
</a:p>
</c:rich>
</c:tx>
</c:title>
<c:dLbls>
<c:showVal val="0"/>
</c:dLbls>
<c:val>
<c:numRef>
<c:f>Sheet1!$B${}:$B${}</c:f>
<c:numCache>
<c:formatCode>General</c:formatCode>"#,
        idx, idx, escape_xml(series_name), 2 + idx, 2 + idx + values.len()
    );

    for value in values {
        xml.push_str(&format!(
            r#"
<c:pt idx="0">
<c:v>{}</c:v>
</c:pt>"#,
            value
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

/// Generate category axis XML
fn generate_category_axis(chart: &Chart, ax_pos: &str) -> String {
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
<c:numFmt formatCode="General" sourceLinked="1"/>
<c:tickLblPos val="low"/>
<c:crossAx val="2"/>
<c:crosses val="autoZero"/>
<c:strRef>
<c:f>Sheet1!$A$2:$A${}</c:f>
<c:strCache>
<c:ptCount val="{}"/>"#,
        ax_pos, 1 + chart.category_count(), chart.category_count()
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
<c:numFmt formatCode="General" sourceLinked="1"/>
<c:tickLblPos val="low"/>
<c:crossAx val="1"/>
<c:crosses val="autoZero"/>
</c:valAx>"#,
        ax_pos
    )
}

/// Generate bar chart XML
fn generate_bar_chart_xml(chart: &Chart) -> String {
    let mut xml = chart_part_header(chart);
    
    xml.push_str(r#"<c:barChart>
<c:barDir val="bar"/>
<c:grouping val="clustered"/>"#);

    for (idx, series) in chart.series.iter().enumerate() {
        xml.push_str(&generate_series_data(chart, idx, &series.name, &series.values));
    }

    xml.push_str(&generate_category_axis(chart, "l"));
    xml.push_str(&generate_value_axis("b"));
    xml.push_str("</c:barChart>");
    xml.push_str(chart_part_footer());

    xml
}

/// Generate line chart XML
fn generate_line_chart_xml(chart: &Chart) -> String {
    let mut xml = chart_part_header(chart);
    
    xml.push_str(r#"<c:lineChart>
<c:grouping val="lineMarkers"/>"#);

    for (idx, series) in chart.series.iter().enumerate() {
        xml.push_str(&generate_series_data(chart, idx, &series.name, &series.values));
    }

    xml.push_str(&generate_category_axis(chart, "b"));
    xml.push_str(&generate_value_axis("l"));
    xml.push_str("</c:lineChart>");
    xml.push_str(chart_part_footer());

    xml
}

/// Generate pie chart XML
fn generate_pie_chart_xml(chart: &Chart) -> String {
    let mut xml = chart_part_header(chart);
    
    xml.push_str(r#"<c:pieChart>
<c:varyColors val="1"/>"#);

    // Pie chart uses first series only
    if let Some(series) = chart.series.first() {
        xml.push_str(&format!(
            r#"
<c:ser>
<c:idx val="0"/>
<c:order val="0"/>
<c:title>
<c:tx>
<c:rich>
<a:bodyPr/>
<a:lstStyle/>
<a:p>
<a:r>
<a:rPr lang="en-US" sz="1000"/>
<a:t>{}</a:t>
</a:r>
</a:p>
</c:rich>
</c:tx>
</c:title>
<c:dLbls>
<c:showCatName val="1"/>
<c:showPercent val="1"/>
</c:dLbls>
<c:val>
<c:numRef>
<c:f>Sheet1!$B$2:$B${}</c:f>
<c:numCache>
<c:formatCode>General</c:formatCode>"#,
            escape_xml(&series.name),
            1 + series.values.len()
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
<c:f>Sheet1!$A$2:$A${}</c:f>
<c:strCache>
<c:ptCount val="{}"/>"#,
            1 + chart.category_count(),
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
    xml.push_str(chart_part_footer());

    xml
}

/// Generate doughnut chart XML
fn generate_doughnut_chart_xml(chart: &Chart) -> String {
    let mut xml = chart_part_header(chart);
    
    xml.push_str(r#"<c:doughnutChart>
<c:varyColors val="1"/>
<c:holeSize val="50"/>"#);

    // Doughnut chart uses first series only (like pie)
    if let Some(series) = chart.series.first() {
        xml.push_str(&format!(
            r#"
<c:ser>
<c:idx val="0"/>
<c:order val="0"/>
<c:tx>
<c:strRef>
<c:f>Sheet1!$B$1</c:f>
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
<c:f>Sheet1!$B$2:$B${}</c:f>
<c:numCache>
<c:formatCode>General</c:formatCode>"#,
            escape_xml(&series.name),
            1 + series.values.len()
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
<c:f>Sheet1!$A$2:$A${}</c:f>
<c:strCache>
<c:ptCount val="{}"/>"#,
            1 + chart.category_count(),
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
    xml.push_str(chart_part_footer());

    xml
}

/// Generate area chart XML
fn generate_area_chart_xml(chart: &Chart) -> String {
    let mut xml = chart_part_header(chart);
    
    let grouping = chart.chart_type.grouping().unwrap_or("standard");
    xml.push_str(&format!(r#"<c:areaChart>
<c:grouping val="{}"/>"#, grouping));

    for (idx, series) in chart.series.iter().enumerate() {
        xml.push_str(&generate_series_data(chart, idx, &series.name, &series.values));
    }

    xml.push_str(&generate_category_axis(chart, "b"));
    xml.push_str(&generate_value_axis("l"));
    xml.push_str("</c:areaChart>");
    xml.push_str(chart_part_footer());

    xml
}

/// Generate scatter chart XML
fn generate_scatter_chart_xml(chart: &Chart) -> String {
    let mut xml = chart_part_header(chart);
    
    let scatter_style = chart.chart_type.scatter_style().unwrap_or("lineMarker");
    xml.push_str(&format!(r#"<c:scatterChart>
<c:scatterStyle val="{}"/>"#, scatter_style));

    for (idx, series) in chart.series.iter().enumerate() {
        xml.push_str(&format!(
            r#"
<c:ser>
<c:idx val="{}"/>
<c:order val="{}"/>
<c:tx>
<c:strRef>
<c:f>Sheet1!$B$1</c:f>
<c:strCache>
<c:ptCount val="1"/>
<c:pt idx="0"><c:v>{}</c:v></c:pt>
</c:strCache>
</c:strRef>
</c:tx>
<c:xVal>
<c:numRef>
<c:f>Sheet1!$A$2:$A${}</c:f>
<c:numCache>
<c:formatCode>General</c:formatCode>"#,
            idx, idx, escape_xml(&series.name), 1 + series.values.len()
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
<c:f>Sheet1!$B$2:$B${}</c:f>
<c:numCache>
<c:formatCode>General</c:formatCode>"#,
            1 + series.values.len()
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
    xml.push_str(chart_part_footer());

    xml
}

/// Generate bubble chart XML
fn generate_bubble_chart_xml(chart: &Chart) -> String {
    let mut xml = chart_part_header(chart);
    
    xml.push_str(r#"<c:bubbleChart>
<c:varyColors val="0"/>
<c:bubbleScale val="100"/>"#);

    for (idx, series) in chart.series.iter().enumerate() {
        xml.push_str(&format!(
            r#"
<c:ser>
<c:idx val="{}"/>
<c:order val="{}"/>
<c:tx>
<c:strRef>
<c:f>Sheet1!$B$1</c:f>
<c:strCache>
<c:ptCount val="1"/>
<c:pt idx="0"><c:v>{}</c:v></c:pt>
</c:strCache>
</c:strRef>
</c:tx>
<c:xVal>
<c:numRef>
<c:f>Sheet1!$A$2:$A${}</c:f>
<c:numCache>
<c:formatCode>General</c:formatCode>"#,
            idx, idx, escape_xml(&series.name), 1 + series.values.len()
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
<c:f>Sheet1!$B$2:$B${}</c:f>
<c:numCache>
<c:formatCode>General</c:formatCode>"#,
            1 + series.values.len()
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

        xml.push_str(&format!(
            r#"
</c:numCache>
</c:numRef>
</c:yVal>
<c:bubbleSize>
<c:numRef>
<c:f>Sheet1!$C$2:$C${}</c:f>
<c:numCache>
<c:formatCode>General</c:formatCode>"#,
            1 + series.values.len()
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
    xml.push_str(chart_part_footer());

    xml
}

/// Generate radar chart XML
fn generate_radar_chart_xml(chart: &Chart) -> String {
    let mut xml = chart_part_header(chart);
    
    let radar_style = chart.chart_type.radar_style().unwrap_or("marker");
    xml.push_str(&format!(r#"<c:radarChart>
<c:radarStyle val="{}"/>"#, radar_style));

    for (idx, series) in chart.series.iter().enumerate() {
        xml.push_str(&generate_series_data(chart, idx, &series.name, &series.values));
    }

    xml.push_str(&generate_category_axis(chart, "b"));
    xml.push_str(&generate_value_axis("l"));
    xml.push_str("</c:radarChart>");
    xml.push_str(chart_part_footer());

    xml
}

/// Generate stock chart XML
fn generate_stock_chart_xml(chart: &Chart) -> String {
    let mut xml = chart_part_header(chart);
    
    xml.push_str(r#"<c:stockChart>"#);

    // Stock charts need High, Low, Close (and optionally Open) series
    for (idx, series) in chart.series.iter().enumerate() {
        xml.push_str(&generate_series_data(chart, idx, &series.name, &series.values));
    }

    xml.push_str(&generate_category_axis(chart, "b"));
    xml.push_str(&generate_value_axis("l"));
    xml.push_str("</c:stockChart>");
    xml.push_str(chart_part_footer());

    xml
}

/// Generate combo chart XML (bar + line)
fn generate_combo_chart_xml(chart: &Chart) -> String {
    let mut xml = chart_part_header(chart);
    
    // First half of series as bars
    xml.push_str(r#"<c:barChart>
<c:barDir val="col"/>
<c:grouping val="clustered"/>"#);

    let mid = chart.series.len() / 2;
    for (idx, series) in chart.series.iter().take(mid.max(1)).enumerate() {
        xml.push_str(&generate_series_data(chart, idx, &series.name, &series.values));
    }

    xml.push_str(&generate_category_axis(chart, "b"));
    xml.push_str(&generate_value_axis("l"));
    xml.push_str("</c:barChart>");

    // Second half as lines
    if chart.series.len() > 1 {
        xml.push_str(r#"<c:lineChart>
<c:grouping val="standard"/>"#);

        for (idx, series) in chart.series.iter().skip(mid.max(1)).enumerate() {
            xml.push_str(&generate_series_data(chart, mid + idx, &series.name, &series.values));
        }

        xml.push_str("</c:lineChart>");
    }

    xml.push_str(chart_part_footer());

    xml
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

        let xml = generate_bar_chart_xml(&chart);
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

        let xml = generate_line_chart_xml(&chart);
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

        let xml = generate_pie_chart_xml(&chart);
        assert!(xml.contains("pieChart"));
    }
}
