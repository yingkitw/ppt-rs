//! Theme, master, and layout XML generation
use crate::util::format_lang_attributes;
/// Create slide layout XML for different layout types
pub fn create_slide_layout_xml_by_type(layout_type: &str, layout_num: usize) -> String {
    match layout_type {
        "title" => create_title_slide_layout(layout_num),
        "obj" => create_title_and_content_layout(layout_num),
        "secHead" => create_section_header_layout(layout_num),
        "twoObj" => create_two_content_layout(layout_num),
        "twoTxTwoObj" => create_comparison_layout(layout_num),
        "titleOnly" => create_title_only_layout(layout_num),
        "blank" => create_blank_layout(layout_num),
        "objTx" => create_content_with_caption_layout(layout_num),
        "picTx" => create_picture_with_caption_layout(layout_num),
        "vertTx" => create_title_and_vertical_text_layout(layout_num),
        "vertTitleAndTx" => create_vertical_title_and_text_layout(layout_num),
        _ => create_blank_layout(layout_num),
    }
}

/// Create title slide layout (layout 1)
pub fn create_title_slide_layout(_layout_num: usize) -> String {
    format!(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldLayout xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" type="title" preserve="1">
<p:cSld name="Title Slide">
<p:spTree>
<p:nvGrpSpPr>
<p:cNvPr id="1" name=""/>
<p:cNvGrpSpPr/>
<p:nvPr/>
</p:nvGrpSpPr>
<p:grpSpPr>
<a:xfrm>
<a:off x="0" y="0"/>
<a:ext cx="0" cy="0"/>
<a:chOff x="0" y="0"/>
<a:chExt cx="0" cy="0"/>
</a:xfrm>
</p:grpSpPr>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="2" name="Title 1"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="ctrTitle"/><p:custDataLst><p:tags r:id="rId2"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr><a:xfrm><a:off x="1198800" y="914400"/><a:ext cx="9799200" cy="2570400"/></a:xfrm></p:spPr>
<p:txBody>
<a:bodyPr lIns="90000" tIns="46800" rIns="90000" bIns="46800" anchor="b" anchorCtr="0"><a:normAutofit/></a:bodyPr>
<a:lstStyle><a:lvl1pPr algn="ctr"><a:defRPr sz="6000"/></a:lvl1pPr></a:lstStyle>
<a:p><a:r><a:rPr lang="zh-CN" altLang="en-US" dirty="0"/><a:t>Click to edit Master title style</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" dirty="0"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="3" name="Subtitle 2"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="subTitle" idx="1"/><p:custDataLst><p:tags r:id="rId3"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr><a:xfrm><a:off x="1198800" y="3560400"/><a:ext cx="9799200" cy="1472400"/></a:xfrm></p:spPr>
<p:txBody>
<a:bodyPr lIns="90000" tIns="46800" rIns="90000" bIns="46800"><a:normAutofit/></a:bodyPr>
<a:lstStyle>
<a:lvl1pPr marL="0" indent="0" algn="ctr"><a:lnSpc><a:spcPct val="110000"/></a:lnSpc><a:buNone/><a:defRPr sz="2400" spc="200"/></a:lvl1pPr>
<a:lvl2pPr marL="457200" indent="0" algn="ctr"><a:buNone/><a:defRPr sz="2000"/></a:lvl2pPr>
<a:lvl3pPr marL="914400" indent="0" algn="ctr"><a:buNone/><a:defRPr sz="1800"/></a:lvl3pPr>
<a:lvl4pPr marL="1371600" indent="0" algn="ctr"><a:buNone/><a:defRPr sz="1600"/></a:lvl4pPr>
<a:lvl5pPr marL="1828800" indent="0" algn="ctr"><a:buNone/><a:defRPr sz="1600"/></a:lvl5pPr>
<a:lvl6pPr marL="2286000" indent="0" algn="ctr"><a:buNone/><a:defRPr sz="1600"/></a:lvl6pPr>
<a:lvl7pPr marL="2743200" indent="0" algn="ctr"><a:buNone/><a:defRPr sz="1600"/></a:lvl7pPr>
<a:lvl8pPr marL="3200400" indent="0" algn="ctr"><a:buNone/><a:defRPr sz="1600"/></a:lvl8pPr>
<a:lvl9pPr marL="3657600" indent="0" algn="ctr"><a:buNone/><a:defRPr sz="1600"/></a:lvl9pPr>
</a:lstStyle>
<a:p><a:r><a:rPr lang="zh-CN" altLang="en-US" dirty="0"/><a:t>Click to edit Master subtitle style</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" dirty="0"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="16" name="Date Placeholder 15"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="dt" sz="half" idx="10"/><p:custDataLst><p:tags r:id="rId4"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr/>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>
<a:p><a:fld id="{{760FBDFE-C587-4B4C-A407-44438C67B59E}}" type="datetimeFigureOut"><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:fld><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="17" name="Footer Placeholder 16"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="ftr" sz="quarter" idx="11"/><p:custDataLst><p:tags r:id="rId5"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr/>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>
<a:p><a:endParaRPr lang="zh-CN" altLang="en-US" dirty="0"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="18" name="Slide Number Placeholder 17"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="sldNum" sz="quarter" idx="12"/><p:custDataLst><p:tags r:id="rId6"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr/>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>
<a:p><a:fld id="{{49AE70B2-8BF9-45C0-BB95-33D1B9D3A854}}" type="slidenum"><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:fld><a:endParaRPr lang="zh-CN" altLang="en-US" dirty="0"/></a:p>
</p:txBody>
</p:sp>
</p:spTree>
</p:cSld>
<p:clrMapOvr>
<a:masterClrMapping/>
</p:clrMapOvr>
</p:sldLayout>"#)
}

/// Create title and content layout (layout 2)
pub fn create_title_and_content_layout(_layout_num: usize) -> String {
    format!(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldLayout xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" type="obj" preserve="1">
<p:cSld name="Title and Content">
<p:spTree>
<p:nvGrpSpPr>
<p:cNvPr id="1" name=""/>
<p:cNvGrpSpPr/>
<p:nvPr/>
</p:nvGrpSpPr>
<p:grpSpPr>
<a:xfrm>
<a:off x="0" y="0"/>
<a:ext cx="0" cy="0"/>
<a:chOff x="0" y="0"/>
<a:chExt cx="0" cy="0"/>
</a:xfrm>
</p:grpSpPr>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="2" name="Title 1"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="title"/><p:custDataLst><p:tags r:id="rId2"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr><a:xfrm><a:off x="608400" y="608400"/><a:ext cx="10969200" cy="705600"/></a:xfrm></p:spPr>
<p:txBody>
<a:bodyPr vert="horz" lIns="90000" tIns="46800" rIns="90000" bIns="46800" rtlCol="0" anchor="ctr" anchorCtr="0"><a:normAutofit/></a:bodyPr>
<a:lstStyle/>
<a:p><a:pPr lvl="0"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>Click to edit Master title style</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="3" name="Content Placeholder 2"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph idx="1"/><p:custDataLst><p:tags r:id="rId3"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr><a:xfrm><a:off x="608400" y="1490400"/><a:ext cx="10969200" cy="4759200"/></a:xfrm></p:spPr>
<p:txBody>
<a:bodyPr vert="horz" lIns="90000" tIns="46800" rIns="90000" bIns="46800" rtlCol="0"><a:normAutofit/></a:bodyPr>
<a:lstStyle/>
<a:p><a:pPr lvl="0"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>Click to edit Master text style</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:p>
<a:p><a:pPr lvl="1"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>Second level</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:p>
<a:p><a:pPr lvl="2"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>Third level</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:p>
<a:p><a:pPr lvl="3"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>Fourth level</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:p>
<a:p><a:pPr lvl="4"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>Fifth level</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="4" name="Date Placeholder 3"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="dt" sz="half" idx="10"/><p:custDataLst><p:tags r:id="rId4"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr/>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>
<a:p><a:fld id="{{760FBDFE-C587-4B4C-A407-44438C67B59E}}" type="datetimeFigureOut"><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:fld><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="5" name="Footer Placeholder 4"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="ftr" sz="quarter" idx="11"/><p:custDataLst><p:tags r:id="rId5"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr/>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>
<a:p><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="6" name="Slide Number Placeholder 5"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="sldNum" sz="quarter" idx="12"/><p:custDataLst><p:tags r:id="rId6"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr/>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>
<a:p><a:fld id="{{49AE70B2-8BF9-45C0-BB95-33D1B9D3A854}}" type="slidenum"><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:fld><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
</p:spTree>
</p:cSld>
<p:clrMapOvr>
<a:masterClrMapping/>
</p:clrMapOvr>
</p:sldLayout>"#)
}

/// Create blank layout (layout 7)
pub fn create_blank_layout(_layout_num: usize) -> String {
    format!(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldLayout xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" type="blank" preserve="1">
<p:cSld name="Blank">
<p:spTree>
<p:nvGrpSpPr>
<p:cNvPr id="1" name=""/>
<p:cNvGrpSpPr/>
<p:nvPr/>
</p:nvGrpSpPr>
<p:grpSpPr>
<a:xfrm>
<a:off x="0" y="0"/>
<a:ext cx="0" cy="0"/>
<a:chOff x="0" y="0"/>
<a:chExt cx="0" cy="0"/>
</a:xfrm>
</p:grpSpPr>
</p:spTree>
</p:cSld>
<p:clrMapOvr>
<a:masterClrMapping/>
</p:clrMapOvr>
</p:sldLayout>"#)
}

/// Create section header layout (layout 3)
pub fn create_section_header_layout(_layout_num: usize) -> String {
    format!(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldLayout xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" type="secHead" preserve="1">
<p:cSld name="Section Header">
<p:spTree>
<p:nvGrpSpPr>
<p:cNvPr id="1" name=""/>
<p:cNvGrpSpPr/>
<p:nvPr/>
</p:nvGrpSpPr>
<p:grpSpPr>
<a:xfrm>
<a:off x="0" y="0"/>
<a:ext cx="0" cy="0"/>
<a:chOff x="0" y="0"/>
<a:chExt cx="0" cy="0"/>
</a:xfrm>
</p:grpSpPr>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="2" name="标题 1"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="title" hasCustomPrompt="1"/><p:custDataLst><p:tags r:id="rId2"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr><a:xfrm><a:off x="1990800" y="3848400"/><a:ext cx="7768800" cy="766800"/></a:xfrm></p:spPr>
<p:txBody>
<a:bodyPr lIns="90000" tIns="46800" rIns="90000" bIns="46800" anchor="b" anchorCtr="0"><a:normAutofit/></a:bodyPr>
<a:lstStyle><a:lvl1pPr><a:defRPr sz="4400"/></a:lvl1pPr></a:lstStyle>
<a:p><a:pPr lvl="0"/><a:r><a:rPr lang="zh-CN" altLang="en-US" dirty="0"/><a:t>Click to edit title</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" dirty="0"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="3" name="Text Placeholder 2"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="body" idx="1" hasCustomPrompt="1"/><p:custDataLst><p:tags r:id="rId3"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr><a:xfrm><a:off x="1990800" y="4615200"/><a:ext cx="7768800" cy="867600"/></a:xfrm></p:spPr>
<p:txBody>
<a:bodyPr lIns="90000" tIns="46800" rIns="90000" bIns="46800"><a:normAutofit/></a:bodyPr>
<a:lstStyle>
<a:lvl1pPr marL="0" indent="0"><a:buNone/><a:defRPr sz="1800"><a:solidFill><a:schemeClr val="tx1"><a:lumMod val="65000"/><a:lumOff val="35000"/></a:schemeClr></a:solidFill></a:defRPr></a:lvl1pPr>
<a:lvl2pPr marL="457200" indent="0"><a:buNone/><a:defRPr sz="1600"><a:solidFill><a:schemeClr val="tx1"><a:tint val="75000"/></a:schemeClr></a:solidFill></a:defRPr></a:lvl2pPr>
<a:lvl3pPr marL="914400" indent="0"><a:buNone/><a:defRPr sz="1600"><a:solidFill><a:schemeClr val="tx1"><a:tint val="75000"/></a:schemeClr></a:solidFill></a:defRPr></a:lvl3pPr>
<a:lvl4pPr marL="1371600" indent="0"><a:buNone/><a:defRPr sz="1600"><a:solidFill><a:schemeClr val="tx1"><a:tint val="75000"/></a:schemeClr></a:solidFill></a:defRPr></a:lvl4pPr>
<a:lvl5pPr marL="1828800" indent="0"><a:buNone/><a:defRPr sz="1600"><a:solidFill><a:schemeClr val="tx1"><a:tint val="75000"/></a:schemeClr></a:solidFill></a:defRPr></a:lvl5pPr>
<a:lvl6pPr marL="2286000" indent="0"><a:buNone/><a:defRPr sz="1600"><a:solidFill><a:schemeClr val="tx1"><a:tint val="75000"/></a:schemeClr></a:solidFill></a:defRPr></a:lvl6pPr>
<a:lvl7pPr marL="2743200" indent="0"><a:buNone/><a:defRPr sz="1600"><a:solidFill><a:schemeClr val="tx1"><a:tint val="75000"/></a:schemeClr></a:solidFill></a:defRPr></a:lvl7pPr>
<a:lvl8pPr marL="3200400" indent="0"><a:buNone/><a:defRPr sz="1600"><a:solidFill><a:schemeClr val="tx1"><a:tint val="75000"/></a:schemeClr></a:solidFill></a:defRPr></a:lvl8pPr>
<a:lvl9pPr marL="3657600" indent="0"><a:buNone/><a:defRPr sz="1600"><a:solidFill><a:schemeClr val="tx1"><a:tint val="75000"/></a:schemeClr></a:solidFill></a:defRPr></a:lvl9pPr>
</a:lstStyle>
<a:p><a:pPr lvl="0"/><a:r><a:rPr lang="zh-CN" altLang="en-US" dirty="0"/><a:t>Click to edit text</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" dirty="0"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="4" name="日期占位符 3"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="dt" sz="half" idx="10"/><p:custDataLst><p:tags r:id="rId4"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr/>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>
<a:p><a:fld id="{{760FBDFE-C587-4B4C-A407-44438C67B59E}}" type="datetimeFigureOut"><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:fld><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="5" name="页脚占位符 4"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="ftr" sz="quarter" idx="11"/><p:custDataLst><p:tags r:id="rId5"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr/>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>
<a:p><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="6" name="灯片编号占位符 5"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="sldNum" sz="quarter" idx="12"/><p:custDataLst><p:tags r:id="rId6"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr/>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>
<a:p><a:fld id="{{49AE70B2-8BF9-45C0-BB95-33D1B9D3A854}}" type="slidenum"><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:fld><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
</p:spTree>
</p:cSld>
<p:clrMapOvr>
<a:masterClrMapping/>
</p:clrMapOvr>
</p:sldLayout>"#)
}

/// Create title only layout (layout 6)
pub fn create_title_only_layout(_layout_num: usize) -> String {
    format!(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldLayout xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" type="titleOnly" preserve="1">
<p:cSld name="Title Only">
<p:spTree>
<p:nvGrpSpPr>
<p:cNvPr id="1" name=""/>
<p:cNvGrpSpPr/>
<p:nvPr/>
</p:nvGrpSpPr>
<p:grpSpPr>
<a:xfrm>
<a:off x="0" y="0"/>
<a:ext cx="0" cy="0"/>
<a:chOff x="0" y="0"/>
<a:chExt cx="0" cy="0"/>
</a:xfrm>
</p:grpSpPr>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="2" name="标题 1"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="title"/><p:custDataLst><p:tags r:id="rId2"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr><a:xfrm><a:off x="608400" y="608400"/><a:ext cx="10969200" cy="705600"/></a:xfrm></p:spPr>
<p:txBody>
<a:bodyPr vert="horz" lIns="90000" tIns="46800" rIns="90000" bIns="46800" rtlCol="0" anchor="ctr" anchorCtr="0"><a:normAutofit/></a:bodyPr>
<a:lstStyle/>
<a:p><a:pPr lvl="0"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>单击此处编辑母版标题样式</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="3" name="日期占位符 2"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="dt" sz="half" idx="10"/><p:custDataLst><p:tags r:id="rId3"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr/>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>
<a:p><a:fld id="{{760FBDFE-C587-4B4C-A407-44438C67B59E}}" type="datetimeFigureOut"><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:fld><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="4" name="页脚占位符 3"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="ftr" sz="quarter" idx="11"/><p:custDataLst><p:tags r:id="rId4"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr/>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>
<a:p><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="5" name="灯片编号占位符 4"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="sldNum" sz="quarter" idx="12"/><p:custDataLst><p:tags r:id="rId5"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr/>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>
<a:p><a:fld id="{{49AE70B2-8BF9-45C0-BB95-33D1B9D3A854}}" type="slidenum"><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:fld><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
</p:spTree>
</p:cSld>
<p:clrMapOvr>
<a:masterClrMapping/>
</p:clrMapOvr>
</p:sldLayout>"#)
}

/// Create slide layout XML (legacy function for backward compatibility)
pub fn create_slide_layout_xml() -> String {
    create_blank_layout(1)
}

/// Create layout relationships XML
pub fn create_layout_rels_xml() -> String {
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
    <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster" Target="../slideMasters/slideMaster1.xml"/>
</Relationships>"#.to_string()
}

/// Create layout relationships XML for a specific layout
pub fn create_layout_rels_xml_for_layout(layout_num: usize) -> String {
    // Calculate starting tag number based on layout number
    // Layout 1 uses tags 1-5, layout 2 uses tags 6-10, etc.
    let start_tag = (layout_num - 1) * 5 + 1;
    
    format!(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
    <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster" Target="../slideMasters/slideMaster1.xml"/>
    <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/tags" Target="../tags/tag{}.xml"/>
    <Relationship Id="rId3" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/tags" Target="../tags/tag{}.xml"/>
    <Relationship Id="rId4" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/tags" Target="../tags/tag{}.xml"/>
    <Relationship Id="rId5" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/tags" Target="../tags/tag{}.xml"/>
    <Relationship Id="rId6" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/tags" Target="../tags/tag{}.xml"/>
</Relationships>"#, 
    start_tag, start_tag + 1, start_tag + 2, start_tag + 3, start_tag + 4)
}

/// Create slide master XML
pub fn create_slide_master_xml() -> String {
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldMaster xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
<p:cSld>
<p:bg>
<p:bgRef idx="1001">
<a:schemeClr val="bg1"/>
</p:bgRef>
</p:bg>
<p:spTree>
<p:nvGrpSpPr>
<p:cNvPr id="1" name=""/>
<p:cNvGrpSpPr/>
<p:nvPr/>
</p:nvGrpSpPr>
<p:grpSpPr>
<a:xfrm>
<a:off x="0" y="0"/>
<a:ext cx="0" cy="0"/>
<a:chOff x="0" y="0"/>
<a:chExt cx="0" cy="0"/>
</a:xfrm>
</p:grpSpPr>
</p:spTree>
</p:cSld>
<p:clrMap bg1="lt1" tx1="dk1" bg2="lt2" tx2="dk2" accent1="accent1" accent2="accent2" accent3="accent3" accent4="accent4" accent5="accent5" accent6="accent6" hlink="hlink" folHlink="folHlink"/>
<p:sldLayoutIdLst>
<p:sldLayoutId id="2147483649" r:id="rId1"/>
<p:sldLayoutId id="2147483650" r:id="rId2"/>
<p:sldLayoutId id="2147483651" r:id="rId3"/>
<p:sldLayoutId id="2147483652" r:id="rId4"/>
<p:sldLayoutId id="2147483653" r:id="rId5"/>
<p:sldLayoutId id="2147483654" r:id="rId6"/>
<p:sldLayoutId id="2147483655" r:id="rId7"/>
<p:sldLayoutId id="2147483656" r:id="rId8"/>
<p:sldLayoutId id="2147483657" r:id="rId9"/>
<p:sldLayoutId id="2147483658" r:id="rId10"/>
<p:sldLayoutId id="2147483659" r:id="rId11"/>
</p:sldLayoutIdLst>
</p:sldMaster>"#.to_string()
}

/// Create master relationships XML
pub fn create_master_rels_xml() -> String {
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout" Target="../slideLayouts/slideLayout1.xml"/>
<Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout" Target="../slideLayouts/slideLayout2.xml"/>
<Relationship Id="rId3" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout" Target="../slideLayouts/slideLayout3.xml"/>
<Relationship Id="rId4" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout" Target="../slideLayouts/slideLayout4.xml"/>
<Relationship Id="rId5" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout" Target="../slideLayouts/slideLayout5.xml"/>
<Relationship Id="rId6" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout" Target="../slideLayouts/slideLayout6.xml"/>
<Relationship Id="rId7" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout" Target="../slideLayouts/slideLayout7.xml"/>
<Relationship Id="rId8" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout" Target="../slideLayouts/slideLayout8.xml"/>
<Relationship Id="rId9" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout" Target="../slideLayouts/slideLayout9.xml"/>
<Relationship Id="rId10" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout" Target="../slideLayouts/slideLayout10.xml"/>
<Relationship Id="rId11" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout" Target="../slideLayouts/slideLayout11.xml"/>
<Relationship Id="rId12" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme" Target="../theme/theme1.xml"/>
</Relationships>"#.to_string()
}

/// Create two content layout (layout 4)
pub fn create_two_content_layout(_layout_num: usize) -> String {
    format!(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldLayout xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" type="twoObj" preserve="1">
<p:cSld name="Two Content">
<p:spTree>
<p:nvGrpSpPr>
<p:cNvPr id="1" name=""/>
<p:cNvGrpSpPr/>
<p:nvPr/>
</p:nvGrpSpPr>
<p:grpSpPr>
<a:xfrm>
<a:off x="0" y="0"/>
<a:ext cx="0" cy="0"/>
<a:chOff x="0" y="0"/>
<a:chExt cx="0" cy="0"/>
</a:xfrm>
</p:grpSpPr>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="2" name="标题 1"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="title"/><p:custDataLst><p:tags r:id="rId2"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr><a:xfrm><a:off x="608400" y="608400"/><a:ext cx="10969200" cy="705600"/></a:xfrm></p:spPr>
<p:txBody>
<a:bodyPr vert="horz" lIns="90000" tIns="46800" rIns="90000" bIns="46800" rtlCol="0" anchor="ctr" anchorCtr="0"><a:normAutofit/></a:bodyPr>
<a:lstStyle/>
<a:p><a:pPr lvl="0"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>单击此处编辑母版标题样式</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="3" name="内容占位符 2"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph idx="1"/><p:custDataLst><p:tags r:id="rId3"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr><a:xfrm><a:off x="608400" y="1490400"/><a:ext cx="5230800" cy="4327200"/></a:xfrm></p:spPr>
<p:txBody>
<a:bodyPr vert="horz" lIns="90000" tIns="46800" rIns="90000" bIns="46800" rtlCol="0"><a:normAutofit/></a:bodyPr>
<a:lstStyle/>
<a:p><a:pPr lvl="0"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>Click to edit Master text style</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:p>
<a:p><a:pPr lvl="1"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>Second level</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:p>
<a:p><a:pPr lvl="2"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>Third level</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:p>
<a:p><a:pPr lvl="3"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>第四级</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:p>
<a:p><a:pPr lvl="4"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>第五级</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="4" name="内容占位符 3"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph idx="2"/><p:custDataLst><p:tags r:id="rId4"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr><a:xfrm><a:off x="6375600" y="1490400"/><a:ext cx="5230800" cy="4327200"/></a:xfrm></p:spPr>
<p:txBody>
<a:bodyPr vert="horz" lIns="90000" tIns="46800" rIns="90000" bIns="46800" rtlCol="0"><a:normAutofit/></a:bodyPr>
<a:lstStyle/>
<a:p><a:pPr lvl="0"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>Click to edit Master text style</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:p>
<a:p><a:pPr lvl="1"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>Second level</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:p>
<a:p><a:pPr lvl="2"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>Third level</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:p>
<a:p><a:pPr lvl="3"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>第四级</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:p>
<a:p><a:pPr lvl="4"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>第五级</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="5" name="日期占位符 4"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="dt" sz="half" idx="10"/><p:custDataLst><p:tags r:id="rId5"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr/>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>
<a:p><a:fld id="{{760FBDFE-C587-4B4C-A407-44438C67B59E}}" type="datetimeFigureOut"><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:fld><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="6" name="页脚占位符 5"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="ftr" sz="quarter" idx="11"/><p:custDataLst><p:tags r:id="rId6"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr/>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>
<a:p><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="7" name="灯片编号占位符 6"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="sldNum" sz="quarter" idx="12"/><p:custDataLst><p:tags r:id="rId7"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr/>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>
<a:p><a:fld id="{{49AE70B2-8BF9-45C0-BB95-33D1B9D3A854}}" type="slidenum"><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:fld><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
</p:spTree>
</p:cSld>
<p:clrMapOvr>
<a:masterClrMapping/>
</p:clrMapOvr>
</p:sldLayout>"#)
}

/// Create comparison layout (layout 5)
pub fn create_comparison_layout(_layout_num: usize) -> String {
    format!(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldLayout xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" type="twoTxTwoObj" preserve="1">
<p:cSld name="Comparison">
<p:spTree>
<p:nvGrpSpPr>
<p:cNvPr id="1" name=""/>
<p:cNvGrpSpPr/>
<p:nvPr/>
</p:nvGrpSpPr>
<p:grpSpPr>
<a:xfrm>
<a:off x="0" y="0"/>
<a:ext cx="0" cy="0"/>
<a:chOff x="0" y="0"/>
<a:chExt cx="0" cy="0"/>
</a:xfrm>
</p:grpSpPr>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="2" name="标题 1"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="title"/><p:custDataLst><p:tags r:id="rId2"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr><a:xfrm><a:off x="608400" y="608400"/><a:ext cx="10969200" cy="705600"/></a:xfrm></p:spPr>
<p:txBody>
<a:bodyPr vert="horz" lIns="90000" tIns="46800" rIns="90000" bIns="46800" rtlCol="0" anchor="ctr" anchorCtr="0"><a:normAutofit/></a:bodyPr>
<a:lstStyle/>
<a:p><a:pPr lvl="0"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>单击此处编辑母版标题样式</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="3" name="Text Placeholder 2"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph idx="1"/><p:custDataLst><p:tags r:id="rId3"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr><a:xfrm><a:off x="608400" y="1490400"/><a:ext cx="5230800" cy="1003320"/></a:xfrm></p:spPr>
<p:txBody>
<a:bodyPr vert="horz" lIns="90000" tIns="46800" rIns="90000" bIns="46800" rtlCol="0"><a:normAutofit/></a:bodyPr>
<a:lstStyle/>
<a:p><a:pPr lvl="0"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>Click to edit Master text style</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:p>
<a:p><a:pPr lvl="1"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>第二级</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:p>
<a:p><a:pPr lvl="2"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>第三级</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:p>
<a:p><a:pPr lvl="3"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>第四级</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:p>
<a:p><a:pPr lvl="4"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>第五级</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="4" name="内容占位符 3"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph idx="2"/><p:custDataLst><p:tags r:id="rId4"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr><a:xfrm><a:off x="608400" y="2493720"/><a:ext cx="5230800" cy="3325200"/></a:xfrm></p:spPr>
<p:txBody>
<a:bodyPr vert="horz" lIns="90000" tIns="46800" rIns="90000" bIns="46800" rtlCol="0"><a:normAutofit/></a:bodyPr>
<a:lstStyle/>
<a:p><a:pPr lvl="0"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>单击此处编辑母版文本样式</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:p>
<a:p><a:pPr lvl="1"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>第二级</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:p>
<a:p><a:pPr lvl="2"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>第三级</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:p>
<a:p><a:pPr lvl="3"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>第四级</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:p>
<a:p><a:pPr lvl="4"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>第五级</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="5" name="Text Placeholder 4"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph idx="3"/><p:custDataLst><p:tags r:id="rId5"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr><a:xfrm><a:off x="6375600" y="1490400"/><a:ext cx="5230800" cy="1003320"/></a:xfrm></p:spPr>
<p:txBody>
<a:bodyPr vert="horz" lIns="90000" tIns="46800" rIns="90000" bIns="46800" rtlCol="0"><a:normAutofit/></a:bodyPr>
<a:lstStyle/>
<a:p><a:pPr lvl="0"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>单击此处编辑母版文本样式</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:p>
<a:p><a:pPr lvl="1"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>第二级</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:p>
<a:p><a:pPr lvl="2"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>第三级</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:p>
<a:p><a:pPr lvl="3"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>第四级</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:p>
<a:p><a:pPr lvl="4"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>第五级</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="6" name="内容占位符 5"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph idx="4"/><p:custDataLst><p:tags r:id="rId6"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr><a:xfrm><a:off x="6375600" y="2493720"/><a:ext cx="5230800" cy="3325200"/></a:xfrm></p:spPr>
<p:txBody>
<a:bodyPr vert="horz" lIns="90000" tIns="46800" rIns="90000" bIns="46800" rtlCol="0"><a:normAutofit/></a:bodyPr>
<a:lstStyle/>
<a:p><a:pPr lvl="0"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>单击此处编辑母版文本样式</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:p>
<a:p><a:pPr lvl="1"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>第二级</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:p>
<a:p><a:pPr lvl="2"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>第三级</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:p>
<a:p><a:pPr lvl="3"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>第四级</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:p>
<a:p><a:pPr lvl="4"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>第五级</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="7" name="日期占位符 6"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="dt" sz="half" idx="10"/><p:custDataLst><p:tags r:id="rId7"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr/>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>
<a:p><a:fld id="{{760FBDFE-C587-4B4C-A407-44438C67B59E}}" type="datetimeFigureOut"><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:fld><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="8" name="页脚占位符 7"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="ftr" sz="quarter" idx="11"/><p:custDataLst><p:tags r:id="rId8"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr/>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>
<a:p><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="9" name="灯片编号占位符 8"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="sldNum" sz="quarter" idx="12"/><p:custDataLst><p:tags r:id="rId9"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr/>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>
<a:p><a:fld id="{{49AE70B2-8BF9-45C0-BB95-33D1B9D3A854}}" type="slidenum"><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:fld><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
</p:spTree>
</p:cSld>
<p:clrMapOvr>
<a:masterClrMapping/>
</p:clrMapOvr>
</p:sldLayout>"#)
}

/// Create content with caption layout (layout 8)
pub fn create_content_with_caption_layout(_layout_num: usize) -> String {
    format!(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldLayout xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" type="objTx" preserve="1">
<p:cSld name="Content with Caption">
<p:spTree>
<p:nvGrpSpPr>
<p:cNvPr id="1" name=""/>
<p:cNvGrpSpPr/>
<p:nvPr/>
</p:nvGrpSpPr>
<p:grpSpPr>
<a:xfrm>
<a:off x="0" y="0"/>
<a:ext cx="0" cy="0"/>
<a:chOff x="0" y="0"/>
<a:chExt cx="0" cy="0"/>
</a:xfrm>
</p:grpSpPr>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="2" name="标题 1"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="title"/><p:custDataLst><p:tags r:id="rId2"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr><a:xfrm><a:off x="608400" y="608400"/><a:ext cx="10969200" cy="705600"/></a:xfrm></p:spPr>
<p:txBody>
<a:bodyPr vert="horz" lIns="90000" tIns="46800" rIns="90000" bIns="46800" rtlCol="0" anchor="ctr" anchorCtr="0"><a:normAutofit/></a:bodyPr>
<a:lstStyle/>
<a:p><a:pPr lvl="0"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>单击此处编辑母版标题样式</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="3" name="内容占位符 2"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph idx="1"/><p:custDataLst><p:tags r:id="rId3"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr><a:xfrm><a:off x="608400" y="1490400"/><a:ext cx="5230800" cy="4327200"/></a:xfrm></p:spPr>
<p:txBody>
<a:bodyPr vert="horz" lIns="90000" tIns="46800" rIns="90000" bIns="46800" rtlCol="0"><a:normAutofit/></a:bodyPr>
<a:lstStyle/>
<a:p><a:pPr lvl="0"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>单击此处编辑母版文本样式</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:p>
<a:p><a:pPr lvl="1"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>第二级</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:p>
<a:p><a:pPr lvl="2"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>第三级</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:p>
<a:p><a:pPr lvl="3"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>第四级</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:p>
<a:p><a:pPr lvl="4"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>第五级</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="4" name="Text Placeholder 3"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="body" idx="2" hasCustomPrompt="1"/><p:custDataLst><p:tags r:id="rId4"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr><a:xfrm><a:off x="6375600" y="1490400"/><a:ext cx="5230800" cy="4327200"/></a:xfrm></p:spPr>
<p:txBody>
<a:bodyPr lIns="90000" tIns="46800" rIns="90000" bIns="46800"><a:normAutofit/></a:bodyPr>
<a:lstStyle>
<a:lvl1pPr marL="0" indent="0"><a:buNone/><a:defRPr sz="1800"><a:solidFill><a:schemeClr val="tx1"><a:lumMod val="65000"/><a:lumOff val="35000"/></a:schemeClr></a:solidFill></a:defRPr></a:lvl1pPr>
<a:lvl2pPr marL="457200" indent="0"><a:buNone/><a:defRPr sz="1600"><a:solidFill><a:schemeClr val="tx1"><a:tint val="75000"/></a:schemeClr></a:solidFill></a:defRPr></a:lvl2pPr>
<a:lvl3pPr marL="914400" indent="0"><a:buNone/><a:defRPr sz="1600"><a:solidFill><a:schemeClr val="tx1"><a:tint val="75000"/></a:schemeClr></a:solidFill></a:defRPr></a:lvl3pPr>
<a:lvl4pPr marL="1371600" indent="0"><a:buNone/><a:defRPr sz="1600"><a:solidFill><a:schemeClr val="tx1"><a:tint val="75000"/></a:schemeClr></a:solidFill></a:defRPr></a:lvl4pPr>
<a:lvl5pPr marL="1828800" indent="0"><a:buNone/><a:defRPr sz="1600"><a:solidFill><a:schemeClr val="tx1"><a:tint val="75000"/></a:schemeClr></a:solidFill></a:defRPr></a:lvl5pPr>
<a:lvl6pPr marL="2286000" indent="0"><a:buNone/><a:defRPr sz="1600"><a:solidFill><a:schemeClr val="tx1"><a:tint val="75000"/></a:schemeClr></a:solidFill></a:defRPr></a:lvl6pPr>
<a:lvl7pPr marL="2743200" indent="0"><a:buNone/><a:defRPr sz="1600"><a:solidFill><a:schemeClr val="tx1"><a:tint val="75000"/></a:schemeClr></a:solidFill></a:defRPr></a:lvl7pPr>
<a:lvl8pPr marL="3200400" indent="0"><a:buNone/><a:defRPr sz="1600"><a:solidFill><a:schemeClr val="tx1"><a:tint val="75000"/></a:schemeClr></a:solidFill></a:defRPr></a:lvl8pPr>
<a:lvl9pPr marL="3657600" indent="0"><a:buNone/><a:defRPr sz="1600"><a:solidFill><a:schemeClr val="tx1"><a:tint val="75000"/></a:schemeClr></a:solidFill></a:defRPr></a:lvl9pPr>
</a:lstStyle>
<a:p><a:pPr lvl="0"/><a:r><a:rPr lang="zh-CN" altLang="en-US" dirty="0"/><a:t>单击此处编辑标题</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" dirty="0"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="5" name="日期占位符 4"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="dt" sz="half" idx="10"/><p:custDataLst><p:tags r:id="rId5"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr/>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>
<a:p><a:fld id="{{760FBDFE-C587-4B4C-A407-44438C67B59E}}" type="datetimeFigureOut"><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:fld><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="6" name="页脚占位符 5"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="ftr" sz="quarter" idx="11"/><p:custDataLst><p:tags r:id="rId6"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr/>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>
<a:p><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="7" name="灯片编号占位符 6"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="sldNum" sz="quarter" idx="12"/><p:custDataLst><p:tags r:id="rId7"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr/>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>
<a:p><a:fld id="{{49AE70B2-8BF9-45C0-BB95-33D1B9D3A854}}" type="slidenum"><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:fld><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
</p:spTree>
</p:cSld>
<p:clrMapOvr>
<a:masterClrMapping/>
</p:clrMapOvr>
</p:sldLayout>"#)
}

/// Create picture with caption layout (layout 9)
pub fn create_picture_with_caption_layout(_layout_num: usize) -> String {
    format!(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldLayout xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" type="picTx" preserve="1">
<p:cSld name="Picture with Caption">
<p:spTree>
<p:nvGrpSpPr>
<p:cNvPr id="1" name=""/>
<p:cNvGrpSpPr/>
<p:nvPr/>
</p:nvGrpSpPr>
<p:grpSpPr>
<a:xfrm>
<a:off x="0" y="0"/>
<a:ext cx="0" cy="0"/>
<a:chOff x="0" y="0"/>
<a:chExt cx="0" cy="0"/>
</a:xfrm>
</p:grpSpPr>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="2" name="标题 1"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="title"/><p:custDataLst><p:tags r:id="rId2"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr><a:xfrm><a:off x="608400" y="608400"/><a:ext cx="10969200" cy="705600"/></a:xfrm></p:spPr>
<p:txBody>
<a:bodyPr vert="horz" lIns="90000" tIns="46800" rIns="90000" bIns="46800" rtlCol="0" anchor="ctr" anchorCtr="0"><a:normAutofit/></a:bodyPr>
<a:lstStyle/>
<a:p><a:pPr lvl="0"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>单击此处编辑母版标题样式</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="3" name="图片占位符 2"/>
<p:cNvSpPr><a:spLocks noGrp="1" noRot="1" noChangeAspect="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="pic" idx="1"/><p:custDataLst><p:tags r:id="rId3"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr><a:xfrm><a:off x="608400" y="1490400"/><a:ext cx="5230800" cy="4327200"/></a:xfrm></p:spPr>
<p:txBody>
<a:bodyPr vert="horz" lIns="90000" tIns="46800" rIns="90000" bIns="46800" rtlCol="0"><a:normAutofit/></a:bodyPr>
<a:lstStyle/>
<a:p><a:pPr lvl="0"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>单击图标添加图片</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="4" name="Text Placeholder 3"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="body" idx="2" hasCustomPrompt="1"/><p:custDataLst><p:tags r:id="rId4"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr><a:xfrm><a:off x="6375600" y="1490400"/><a:ext cx="5230800" cy="4327200"/></a:xfrm></p:spPr>
<p:txBody>
<a:bodyPr lIns="90000" tIns="46800" rIns="90000" bIns="46800"><a:normAutofit/></a:bodyPr>
<a:lstStyle>
<a:lvl1pPr marL="0" indent="0"><a:buNone/><a:defRPr sz="1800"><a:solidFill><a:schemeClr val="tx1"><a:lumMod val="65000"/><a:lumOff val="35000"/></a:schemeClr></a:solidFill></a:defRPr></a:lvl1pPr>
<a:lvl2pPr marL="457200" indent="0"><a:buNone/><a:defRPr sz="1600"><a:solidFill><a:schemeClr val="tx1"><a:tint val="75000"/></a:schemeClr></a:solidFill></a:defRPr></a:lvl2pPr>
<a:lvl3pPr marL="914400" indent="0"><a:buNone/><a:defRPr sz="1600"><a:solidFill><a:schemeClr val="tx1"><a:tint val="75000"/></a:schemeClr></a:solidFill></a:defRPr></a:lvl3pPr>
<a:lvl4pPr marL="1371600" indent="0"><a:buNone/><a:defRPr sz="1600"><a:solidFill><a:schemeClr val="tx1"><a:tint val="75000"/></a:schemeClr></a:solidFill></a:defRPr></a:lvl4pPr>
<a:lvl5pPr marL="1828800" indent="0"><a:buNone/><a:defRPr sz="1600"><a:solidFill><a:schemeClr val="tx1"><a:tint val="75000"/></a:schemeClr></a:solidFill></a:defRPr></a:lvl5pPr>
<a:lvl6pPr marL="2286000" indent="0"><a:buNone/><a:defRPr sz="1600"><a:solidFill><a:schemeClr val="tx1"><a:tint val="75000"/></a:schemeClr></a:solidFill></a:defRPr></a:lvl6pPr>
<a:lvl7pPr marL="2743200" indent="0"><a:buNone/><a:defRPr sz="1600"><a:solidFill><a:schemeClr val="tx1"><a:tint val="75000"/></a:schemeClr></a:solidFill></a:defRPr></a:lvl7pPr>
<a:lvl8pPr marL="3200400" indent="0"><a:buNone/><a:defRPr sz="1600"><a:solidFill><a:schemeClr val="tx1"><a:tint val="75000"/></a:schemeClr></a:solidFill></a:defRPr></a:lvl8pPr>
<a:lvl9pPr marL="3657600" indent="0"><a:buNone/><a:defRPr sz="1600"><a:solidFill><a:schemeClr val="tx1"><a:tint val="75000"/></a:schemeClr></a:solidFill></a:defRPr></a:lvl9pPr>
</a:lstStyle>
<a:p><a:pPr lvl="0"/><a:r><a:rPr lang="zh-CN" altLang="en-US" dirty="0"/><a:t>单击此处编辑标题</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" dirty="0"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="5" name="日期占位符 4"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="dt" sz="half" idx="10"/><p:custDataLst><p:tags r:id="rId5"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr/>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>
<a:p><a:fld id="{{760FBDFE-C587-4B4C-A407-44438C67B59E}}" type="datetimeFigureOut"><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:fld><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="6" name="页脚占位符 5"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="ftr" sz="quarter" idx="11"/><p:custDataLst><p:tags r:id="rId6"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr/>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>
<a:p><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="7" name="灯片编号占位符 6"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="sldNum" sz="quarter" idx="12"/><p:custDataLst><p:tags r:id="rId7"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr/>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>
<a:p><a:fld id="{{49AE70B2-8BF9-45C0-BB95-33D1B9D3A854}}" type="slidenum"><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:fld><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
</p:spTree>
</p:cSld>
<p:clrMapOvr>
<a:masterClrMapping/>
</p:clrMapOvr>
</p:sldLayout>"#)
}

/// Create title and vertical text layout (layout 10)
pub fn create_title_and_vertical_text_layout(_layout_num: usize) -> String {
    format!(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldLayout xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" type="vertTx" preserve="1">
<p:cSld name="垂直排列标题和文本">
<p:spTree>
<p:nvGrpSpPr>
<p:cNvPr id="1" name=""/>
<p:cNvGrpSpPr/>
<p:nvPr/>
</p:nvGrpSpPr>
<p:grpSpPr>
<a:xfrm>
<a:off x="0" y="0"/>
<a:ext cx="0" cy="0"/>
<a:chOff x="0" y="0"/>
<a:chExt cx="0" cy="0"/>
</a:xfrm>
</p:grpSpPr>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="2" name="标题 1"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="title"/><p:custDataLst><p:tags r:id="rId2"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr><a:xfrm><a:off x="608400" y="608400"/><a:ext cx="10969200" cy="705600"/></a:xfrm></p:spPr>
<p:txBody>
<a:bodyPr vert="horz" lIns="90000" tIns="46800" rIns="90000" bIns="46800" rtlCol="0" anchor="ctr" anchorCtr="0"><a:normAutofit/></a:bodyPr>
<a:lstStyle/>
<a:p><a:pPr lvl="0"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>单击此处编辑母版标题样式</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="3" name="Text Placeholder 2"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph idx="1"/><p:custDataLst><p:tags r:id="rId3"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr><a:xfrm><a:off x="608400" y="1490400"/><a:ext cx="10969200" cy="4327200"/></a:xfrm></p:spPr>
<p:txBody>
<a:bodyPr vert="vert" lIns="90000" tIns="46800" rIns="90000" bIns="46800" rtlCol="0"><a:normAutofit/></a:bodyPr>
<a:lstStyle/>
<a:p><a:pPr lvl="0"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>单击此处编辑母版文本样式</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:p>
<a:p><a:pPr lvl="1"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>第二级</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:p>
<a:p><a:pPr lvl="2"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>第三级</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:p>
<a:p><a:pPr lvl="3"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>第四级</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:p>
<a:p><a:pPr lvl="4"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>第五级</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="4" name="日期占位符 3"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="dt" sz="half" idx="10"/><p:custDataLst><p:tags r:id="rId4"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr/>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>
<a:p><a:fld id="{{760FBDFE-C587-4B4C-A407-44438C67B59E}}" type="datetimeFigureOut"><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:fld><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="5" name="页脚占位符 4"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="ftr" sz="quarter" idx="11"/><p:custDataLst><p:tags r:id="rId5"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr/>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>
<a:p><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="6" name="灯片编号占位符 5"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="sldNum" sz="quarter" idx="12"/><p:custDataLst><p:tags r:id="rId6"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr/>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>
<a:p><a:fld id="{{49AE70B2-8BF9-45C0-BB95-33D1B9D3A854}}" type="slidenum"><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:fld><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
</p:spTree>
</p:cSld>
<p:clrMapOvr>
<a:masterClrMapping/>
</p:clrMapOvr>
</p:sldLayout>"#)
}

/// Create vertical title and text layout (layout 11)
pub fn create_vertical_title_and_text_layout(_layout_num: usize) -> String {
    format!(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldLayout xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" type="vertTitleAndTx" preserve="1">
<p:cSld name="垂直排列标题和文本">
<p:spTree>
<p:nvGrpSpPr>
<p:cNvPr id="1" name=""/>
<p:cNvGrpSpPr/>
<p:nvPr/>
</p:nvGrpSpPr>
<p:grpSpPr>
<a:xfrm>
<a:off x="0" y="0"/>
<a:ext cx="0" cy="0"/>
<a:chOff x="0" y="0"/>
<a:chExt cx="0" cy="0"/>
</a:xfrm>
</p:grpSpPr>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="2" name="标题 1"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="vertTitle"/><p:custDataLst><p:tags r:id="rId2"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr><a:xfrm><a:off x="608400" y="608400"/><a:ext cx="10969200" cy="705600"/></a:xfrm></p:spPr>
<p:txBody>
<a:bodyPr vert="vert" lIns="90000" tIns="46800" rIns="90000" bIns="46800" rtlCol="0" anchor="ctr" anchorCtr="0"><a:normAutofit/></a:bodyPr>
<a:lstStyle/>
<a:p><a:pPr lvl="0"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>单击此处编辑母版标题样式</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="3" name="Text Placeholder 2"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph idx="1"/><p:custDataLst><p:tags r:id="rId3"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr><a:xfrm><a:off x="608400" y="1490400"/><a:ext cx="10969200" cy="4327200"/></a:xfrm></p:spPr>
<p:txBody>
<a:bodyPr vert="horz" lIns="90000" tIns="46800" rIns="90000" bIns="46800" rtlCol="0"><a:normAutofit/></a:bodyPr>
<a:lstStyle/>
<a:p><a:pPr lvl="0"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>单击此处编辑母版文本样式</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:p>
<a:p><a:pPr lvl="1"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>第二级</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:p>
<a:p><a:pPr lvl="2"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>第三级</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:p>
<a:p><a:pPr lvl="3"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>第四级</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:p>
<a:p><a:pPr lvl="4"/><a:r><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/><a:t>第五级</a:t></a:r><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="4" name="日期占位符 3"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="dt" sz="half" idx="10"/><p:custDataLst><p:tags r:id="rId4"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr/>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>
<a:p><a:fld id="{{760FBDFE-C587-4B4C-A407-44438C67B59E}}" type="datetimeFigureOut"><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:fld><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="5" name="页脚占位符 4"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="ftr" sz="quarter" idx="11"/><p:custDataLst><p:tags r:id="rId5"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr/>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>
<a:p><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="6" name="灯片编号占位符 5"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="sldNum" sz="quarter" idx="12"/><p:custDataLst><p:tags r:id="rId6"/></p:custDataLst></p:nvPr>
</p:nvSpPr>
<p:spPr/>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>
<a:p><a:fld id="{{49AE70B2-8BF9-45C0-BB95-33D1B9D3A854}}" type="slidenum"><a:rPr lang="zh-CN" altLang="en-US" smtClean="0"/></a:fld><a:endParaRPr lang="zh-CN" altLang="en-US"/></a:p>
</p:txBody>
</p:sp>
</p:spTree>
</p:cSld>
<p:clrMapOvr>
<a:masterClrMapping/>
</p:clrMapOvr>
</p:sldLayout>"#)
}

/// Create theme XML
pub fn create_theme_xml() -> String {
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<a:theme xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" name="Office Theme">
<a:themeElements>
<a:clrScheme name="Office">
<a:dk1><a:sysClr val="windowText" lastClr="000000"/></a:dk1>
<a:lt1><a:sysClr val="window" lastClr="FFFFFF"/></a:lt1>
<a:dk2><a:srgbClr val="1F497D"/></a:dk2>
<a:lt2><a:srgbClr val="EEECE1"/></a:lt2>
<a:accent1><a:srgbClr val="4F81BD"/></a:accent1>
<a:accent2><a:srgbClr val="C0504D"/></a:accent2>
<a:accent3><a:srgbClr val="9BBB59"/></a:accent3>
<a:accent4><a:srgbClr val="8064A2"/></a:accent4>
<a:accent5><a:srgbClr val="4BACC6"/></a:accent5>
<a:accent6><a:srgbClr val="F79646"/></a:accent6>
<a:hlink><a:srgbClr val="0000FF"/></a:hlink>
<a:folHlink><a:srgbClr val="800080"/></a:folHlink>
</a:clrScheme>
<a:fontScheme name="Office">
<a:majorFont>
<a:latin typeface="Calibri"/>
<a:ea typeface=""/>
<a:cs typeface=""/>
</a:majorFont>
<a:minorFont>
<a:latin typeface="Calibri"/>
<a:ea typeface=""/>
<a:cs typeface=""/>
</a:minorFont>
</a:fontScheme>
<a:fmtScheme name="Office">
<a:fillStyleLst>
<a:solidFill><a:schemeClr val="phClr"/></a:solidFill>
<a:gradFill rotWithShape="1"><a:gsLst><a:gs pos="0"><a:schemeClr val="phClr"><a:tint val="50000"/><a:satMod val="300000"/></a:schemeClr></a:gs><a:gs pos="35000"><a:schemeClr val="phClr"><a:tint val="37000"/><a:satMod val="300000"/></a:schemeClr></a:gs><a:gs pos="100000"><a:schemeClr val="phClr"><a:tint val="15000"/><a:satMod val="350000"/></a:schemeClr></a:gs></a:gsLst><a:lin ang="16200000" scaled="1"/></a:gradFill>
<a:gradFill rotWithShape="1"><a:gsLst><a:gs pos="0"><a:schemeClr val="phClr"><a:shade val="51000"/><a:satMod val="130000"/></a:schemeClr></a:gs><a:gs pos="80000"><a:schemeClr val="phClr"><a:shade val="93000"/><a:satMod val="130000"/></a:schemeClr></a:gs><a:gs pos="100000"><a:schemeClr val="phClr"><a:shade val="94000"/><a:satMod val="135000"/></a:schemeClr></a:gs></a:gsLst><a:lin ang="16200000" scaled="0"/></a:gradFill>
</a:fillStyleLst>
<a:lnStyleLst>
<a:ln w="9525" cap="flat" cmpd="sng" algn="ctr"><a:solidFill><a:schemeClr val="phClr"><a:shade val="95000"/><a:satMod val="105000"/></a:schemeClr></a:solidFill><a:prstDash val="solid"/></a:ln>
<a:ln w="25400" cap="flat" cmpd="sng" algn="ctr"><a:solidFill><a:schemeClr val="phClr"/></a:solidFill><a:prstDash val="solid"/></a:ln>
<a:ln w="38100" cap="flat" cmpd="sng" algn="ctr"><a:solidFill><a:schemeClr val="phClr"/></a:solidFill><a:prstDash val="solid"/></a:ln>
</a:lnStyleLst>
<a:effectStyleLst>
<a:effectStyle><a:effectLst/></a:effectStyle>
<a:effectStyle><a:effectLst/></a:effectStyle>
<a:effectStyle><a:effectLst/></a:effectStyle>
</a:effectStyleLst>
<a:bgFillStyleLst>
<a:solidFill><a:schemeClr val="phClr"/></a:solidFill>
<a:gradFill rotWithShape="1"><a:gsLst><a:gs pos="0"><a:schemeClr val="phClr"><a:tint val="40000"/><a:satMod val="350000"/></a:schemeClr></a:gs><a:gs pos="40000"><a:schemeClr val="phClr"><a:tint val="45000"/><a:shade val="99000"/><a:satMod val="350000"/></a:schemeClr></a:gs><a:gs pos="100000"><a:schemeClr val="phClr"><a:shade val="20000"/><a:satMod val="255000"/></a:schemeClr></a:gs></a:gsLst><a:path path="circle"><a:fillToRect l="50000" t="-80000" r="50000" b="180000"/></a:path></a:gradFill>
<a:gradFill rotWithShape="1"><a:gsLst><a:gs pos="0"><a:schemeClr val="phClr"><a:tint val="80000"/><a:satMod val="300000"/></a:schemeClr></a:gs><a:gs pos="100000"><a:schemeClr val="phClr"><a:shade val="30000"/><a:satMod val="200000"/></a:schemeClr></a:gs></a:gsLst><a:path path="circle"><a:fillToRect l="50000" t="50000" r="50000" b="50000"/></a:path></a:gradFill>
</a:bgFillStyleLst>
</a:fmtScheme>
</a:themeElements>
<a:objectDefaults/>
<a:extraClrSchemeLst/>
</a:theme>"#.to_string()
}
