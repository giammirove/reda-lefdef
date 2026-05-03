/*
*
* General rules:
*   - case sensitive: "LAYER" is not "layer"
*
* If it is not defined here it most likely will not be
* recognized:
* http://coriolis.lip6.fr/doc/lefdef/lefdefref/LEFSyntax.html#Extensions
*
    LEF Statement Definitions
    [-] Bus Bit Characters
    [-] Clearance Measure
    [-] Divider Character
    [ ] Extensions
    [-] Layer (Cut)
    [-] Layer (Implant)
    [-] Layer (Masterslice or Overlap)
    [-] Layer (Routing)
    [-] Library
    [-] Macro
    [-] Layer Geometries
    [-] Macro Obstruction Statement
    [-] Macro Pin Statement
    [-] Manufacturing Grid
    [-] Maximum Via Stack
    [-] Nondefault Rule
    [-] Property Definitions
    [-] Site
    [-] Units
    [-] Use Min Spacing
    [-] Version
    [-] Via
    [-] Via Rule
    [-] Via Rule Generate
    [ ] Alias
*
*/
#![allow(unused)]
#![allow(clippy::upper_case_acronyms)]
use crate::copy_opt;
use crate::copy_vec_opt;
use crate::{common_handler::*, lef};
use eyre::OptionExt;
use eyre::{eyre, Result};
use std::ffi::OsString;
use std::fmt;
use std::time::Instant;

#[derive(Debug)]
pub enum ClearanceMeasureValue {
    MAXXY,
    EUCLIDEAN,
}

#[derive(Debug)]
pub struct MaxViaStackRange {
    bottomlayer: String,
    toplayer: String,
}
impl MaxViaStackRange {
    pub fn new(bottomlayer: String, toplayer: String) -> Self {
        Self {
            bottomlayer,
            toplayer,
        }
    }
}

#[derive(Debug)]
pub struct MaxViaStack {
    value: f32,
    range: Option<MaxViaStackRange>,
}
impl MaxViaStack {
    pub fn new(value: f32, range: Option<MaxViaStackRange>) -> Self {
        Self { value, range }
    }
}

#[derive(Debug, Default)]
pub struct Units {
    pub time: Option<i32>,
    pub capacitance: Option<i32>,
    pub resistance: Option<i32>,
    pub power: Option<i32>,
    pub current: Option<i32>,
    pub voltage: Option<i32>,
    pub database: Option<i32>,
    pub frequency: Option<i32>,
}

impl Units {
    pub fn new() -> Self {
        Self {
            time: None,
            capacitance: None,
            resistance: None,
            power: None,
            current: None,
            voltage: None,
            database: None,
            frequency: None,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default)]
pub struct LayerCutSpacingLayer {
    name: String,
    stack: Option<bool>,
}
impl LayerCutSpacingLayer {
    pub fn new(name: String, stack: Option<bool>) -> Self {
        Self { name, stack }
    }
}

#[derive(Debug, Default)]
pub struct LayerCutSpacingAdjacentCut {
    num: i32,
    within: f32,
    exceptsamepgnet: Option<bool>,
}
impl LayerCutSpacingAdjacentCut {
    pub fn new(num: i32, within: f32, exceptsamepgnet: Option<bool>) -> Self {
        Self {
            num,
            within,
            exceptsamepgnet,
        }
    }
}

#[derive(Debug)]
pub enum LayerCutSpacingEnum {
    LAYER(LayerCutSpacingLayer),
    ADJACENTCUT(LayerCutSpacingAdjacentCut),
    PARALLELOVERLAP,
    AREA(f32),
}

#[derive(Debug)]
pub struct LayerCutSpacing {
    spacing: f32,
    centertocenter: Option<bool>,
    samenet: Option<bool>,
    opts: Option<LayerCutSpacingEnum>,
}
impl LayerCutSpacing {
    pub fn new(
        spacing: f32,
        centertocenter: Option<bool>,
        samenet: Option<bool>,
        opts: Option<LayerCutSpacingEnum>,
    ) -> Self {
        Self {
            spacing,
            centertocenter,
            samenet,
            opts,
        }
    }
}
////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct LayerCutSpacingTableWithin {
    within: f32,
    spacing: f32,
}
impl LayerCutSpacingTableWithin {
    pub fn new(within: f32, spacing: f32) -> Self {
        Self { within, spacing }
    }
}

#[derive(Debug)]
pub struct LayerCutSpacingTable {
    spacings: Option<Vec<LayerCutSpacingTableWithin>>,
}
impl LayerCutSpacingTable {
    pub fn new(spacings: Option<Vec<LayerCutSpacingTableWithin>>) -> Self {
        Self { spacings }
    }
}
////////////////////////////////////////////////////////////////////////////////

/////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct LayerArraySpacingArrayCuts {
    cuts: i32,
    spacing: f32,
}
impl LayerArraySpacingArrayCuts {
    pub fn new(cuts: i32, spacing: f32) -> Self {
        Self { cuts, spacing }
    }
}

#[derive(Debug)]
pub struct LayerArraySpacing {
    longarray: bool,
    width: Option<f32>,
    cutspacing: f32,
    arraycuts: Vec<LayerArraySpacingArrayCuts>,
}
impl LayerArraySpacing {
    pub fn new(
        longarray: bool,
        width: Option<f32>,
        cutspacing: f32,
        arraycuts: Vec<LayerArraySpacingArrayCuts>,
    ) -> Self {
        Self {
            longarray,
            width,
            cutspacing,
            arraycuts,
        }
    }
}

/////////////////////////////////////////////////////////////////////////////////

/////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct LayerEnclosureWidth {
    width: f32,
    exceptextracut: Option<f32>,
}

#[derive(Debug)]
pub enum LayerEnclosureEnum {
    WIDTH(LayerEnclosureWidth),
    LENGTH(f32),
}
impl LayerEnclosureWidth {
    pub fn new(width: f32, exceptextracut: Option<f32>) -> Self {
        Self {
            width,
            exceptextracut,
        }
    }
}

#[derive(Debug)]
pub enum LayerEnclosureTypeEnum {
    ABOVE,
    BELOW,
}

#[derive(Debug)]
pub struct LayerEnclosure {
    ltype: Option<LayerEnclosureTypeEnum>,
    overhang1: f32,
    overhang2: f32,
    width_length: Option<LayerEnclosureEnum>,
}
impl LayerEnclosure {
    pub fn new(
        ltype: Option<LayerEnclosureTypeEnum>,
        overhang1: f32,
        overhang2: f32,
        width_length: Option<LayerEnclosureEnum>,
    ) -> Self {
        Self {
            ltype,
            overhang1,
            overhang2,
            width_length,
        }
    }
    pub fn get_overhang(&self) -> (f32, f32) {
        (self.overhang1, self.overhang2)
    }
}

/////////////////////////////////////////////////////////////////////////////////

/////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum Value {
    NUMBER(f32),
    STRING(String),
}

#[derive(Debug)]
pub struct NameValue {
    name: String,
    value: Value,
}
impl NameValue {
    pub fn new(name: String, value: Value) -> Self {
        Self { name, value }
    }
}

#[derive(Debug)]
pub struct Property {
    name_values: Vec<NameValue>,
}
impl Property {
    pub fn new(name_values: Vec<NameValue>) -> Self {
        Self { name_values }
    }
}

/////////////////////////////////////////////////////////////////////////////////

/////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct ACCurrentDensityPeakRMS {
    frequency: Vec<f32>,
    cutarea: Option<Vec<f32>>,
    width: Option<Vec<f32>>,
    tableentries: Vec<f32>,
}
impl ACCurrentDensityPeakRMS {
    pub fn new(
        frequency: Vec<f32>,
        cutarea: Option<Vec<f32>>,
        width: Option<Vec<f32>>,
        tableentries: Vec<f32>,
    ) -> Self {
        Self {
            frequency,
            cutarea,
            width,
            tableentries,
        }
    }
}

#[derive(Debug)]
pub enum ACCurrentDensityEnum {
    PEAK(ACCurrentDensityPeakRMS),
    RMS(ACCurrentDensityPeakRMS),
    AVERAGE,
}

#[derive(Debug)]
pub struct ACCurrentDensity {
    atype: ACCurrentDensityEnum,
}
impl ACCurrentDensity {
    pub fn new(a: ACCurrentDensityEnum) -> Self {
        Self { atype: a }
    }
}

#[derive(Debug)]
pub struct DCCurrentDensityCutTable {
    cutarea: Option<Vec<f32>>,
    width: Option<Vec<f32>>,
    tableentries: Vec<f32>,
}
impl DCCurrentDensityCutTable {
    pub fn new(cutarea: Option<Vec<f32>>, width: Option<Vec<f32>>, tableentries: Vec<f32>) -> Self {
        Self {
            cutarea,
            width,
            tableentries,
        }
    }
}

#[derive(Debug)]
pub enum DCCurrentDensityEnum {
    CUTTABLE(DCCurrentDensityCutTable),
    REAL(f32),
}

#[derive(Debug)]
pub struct DCCurrentDensity {
    atype: DCCurrentDensityEnum,
}
impl DCCurrentDensity {
    pub fn new(a: DCCurrentDensityEnum) -> Self {
        Self { atype: a }
    }
}

/////////////////////////////////////////////////////////////////////////////////

/////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum AntennaModelType {
    OXIDE1,
    OXIDE2,
    OXIDE3,
    OXIDE4,
}

#[derive(Debug)]
pub struct AntennaModel {
    atype: AntennaModelType,
}
impl AntennaModel {
    pub fn new(a: AntennaModelType) -> Self {
        Self { atype: a }
    }
}

#[derive(Debug)]
pub struct AntennaPartialMetalArea {
    area: f32,
    layer_name: Option<String>,
}
impl AntennaPartialMetalArea {
    pub fn new(area: f32, layer_name: Option<String>) -> Self {
        Self { area, layer_name }
    }
}

#[derive(Debug)]
pub struct AntennaPartialMetalSideArea {
    area: f32,
    layer_name: Option<String>,
}
impl AntennaPartialMetalSideArea {
    pub fn new(area: f32, layer_name: Option<String>) -> Self {
        Self { area, layer_name }
    }
}

#[derive(Debug)]
pub struct AntennaPartialCutArea {
    area: f32,
    layer_name: Option<String>,
}
impl AntennaPartialCutArea {
    pub fn new(area: f32, layer_name: Option<String>) -> Self {
        Self { area, layer_name }
    }
}

#[derive(Debug)]
pub struct AntennaDiffArea {
    area: f32,
    layer_name: Option<String>,
}
impl AntennaDiffArea {
    pub fn new(area: f32, layer_name: Option<String>) -> Self {
        Self { area, layer_name }
    }
}

#[derive(Debug)]
pub struct AntennaGateArea {
    area: f32,
    layer_name: Option<String>,
}
impl AntennaGateArea {
    pub fn new(area: f32, layer_name: Option<String>) -> Self {
        Self { area, layer_name }
    }
}

#[derive(Debug)]
pub struct AntennaMaxAreaCar {
    area: f32,
    layer_name: String,
}
impl AntennaMaxAreaCar {
    pub fn new(area: f32, layer_name: String) -> Self {
        Self { area, layer_name }
    }
}

#[derive(Debug)]
pub struct AntennaMaxSideAreaCar {
    area: f32,
    layer_name: String,
}
impl AntennaMaxSideAreaCar {
    pub fn new(area: f32, layer_name: String) -> Self {
        Self { area, layer_name }
    }
}

#[derive(Debug)]
pub struct AntennaMaxCutCar {
    area: f32,
    layer_name: String,
}
impl AntennaMaxCutCar {
    pub fn new(area: f32, layer_name: String) -> Self {
        Self { area, layer_name }
    }
}

/////////////////////////////////////////////////////////////////////////////////

/////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum PieceWiseLinear {
    REAL(f32),
    PWL(Vec<(f32, f32)>),
}

#[derive(Debug)]
pub struct LayerAntennaAreaFactor {
    value: f32,
    diffuseonly: bool,
}
impl LayerAntennaAreaFactor {
    pub fn new(v: f32, d: bool) -> Self {
        Self {
            value: v,
            diffuseonly: d,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default)]
pub struct LayerCutOpts {
    pub mask: Option<i32>,
    pub spacings: Option<Vec<LayerCutSpacing>>,
    pub spacingtable: Option<LayerCutSpacingTable>,
    pub arrayspacing: Option<LayerArraySpacing>,
    pub width: Option<f32>,
    pub diagwidth: Option<f32>,
    pub diagspacing: Option<f32>,
    pub diagminedgelength: Option<f32>,
    pub offset: Option<RangeOrValue<f32>>,
    pub pitch: Option<RangeOrValue<f32>>,
    pub diagpitch: Option<RangeOrValue<f32>>,
    pub enclosures: Option<Vec<LayerEnclosure>>,
    pub preferenclosures: Option<Vec<LayerEnclosure>>,
    pub resistance: Option<PieceWiseLinear>,
    pub capacitance: Option<PieceWiseLinear>,
    pub accurrentdensity: Option<ACCurrentDensity>,
    pub dccurrentdensity: Option<DCCurrentDensity>,

    pub antennamodels: Option<Vec<AntennaModel>>,
    pub antennaarearatios: Option<Vec<f32>>,
    pub antennasidearearatios: Option<Vec<f32>>,
    pub antennacumsidearearatios: Option<Vec<f32>>,
    pub antennadiffsidearearatios: Option<Vec<PieceWiseLinear>>,
    pub antennacumdiffsidearearatios: Option<Vec<PieceWiseLinear>>,
    pub antennadiffarearatios: Option<Vec<PieceWiseLinear>>,
    pub antennacumarearatios: Option<Vec<f32>>,
    pub antennacumdiffarearatios: Option<Vec<PieceWiseLinear>>,
    pub antennaareafactors: Option<Vec<LayerAntennaAreaFactor>>,
    pub antennasideareafactors: Option<Vec<LayerAntennaAreaFactor>>,

    pub antennacumroutingpluscut: Option<bool>,
    pub antennagateplusdiff: Option<f32>,
    pub antennaareaminusdiff: Option<f32>,
    pub antennaareadiffreducepwl: Option<Vec<(f32, f32)>>,
    pub properties: Option<Vec<Property>>,
}

#[derive(Debug)]
pub struct LayerCut {
    name: String,
    opts: LayerCutOpts,
}
impl LayerCut {
    pub fn new(name: String, opts: LayerCutOpts, chk_name: String) -> Self {
        if name != chk_name {
            panic!("{} is different from {}", name, chk_name);
        }
        Self { name, opts }
    }
}

////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct LayerImplantSpacing {
    spacing: f32,
    layer: Option<String>,
}
impl LayerImplantSpacing {
    pub fn new(s: f32, l: Option<String>) -> Self {
        Self {
            spacing: s,
            layer: l,
        }
    }
}

#[derive(Debug, Default)]
pub struct LayerImplantOpts {
    pub mask: Option<i32>,
    pub spacing: Option<LayerImplantSpacing>,
    pub width: Option<f32>,
    pub properties: Option<Vec<Property>>,
}

#[derive(Debug)]
pub struct LayerImplant {
    name: String,
    opts: LayerImplantOpts,
}
impl LayerImplant {
    pub fn new(name: String, opts: LayerImplantOpts, chk_name: String) -> Self {
        if name != chk_name {
            panic!("{} is different from {}", name, chk_name);
        }
        Self { name, opts }
    }
}

////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum LayerMasterSliceEnum {
    MASTERSLICE,
    OVERLAP,
}

#[derive(Debug, Default)]
pub struct LayerMasterSliceOpts {
    pub mask: Option<i32>,
    pub properties: Option<Vec<Property>>,
}

#[derive(Debug)]
pub struct LayerMasterSlice {
    name: String,
    opts: LayerMasterSliceOpts,
}
impl LayerMasterSlice {
    pub fn new(name: String, opts: LayerMasterSliceOpts, chk_name: String) -> Self {
        if name != chk_name {
            panic!("{} is different from {}", name, chk_name);
        }
        Self { name, opts }
    }
}

////////////////////////////////////////////////////////////////////////////////

/////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum LayerRoutingDirection {
    HORIZONTAL,
    VERTICAL,
    DIAG45,
    DIAG135,
}

#[derive(Debug)]
pub struct LayerRoutingSize {
    width: f32,
    length: f32,
}
impl LayerRoutingSize {
    pub fn new(w: f32, l: f32) -> Self {
        Self {
            width: w,
            length: l,
        }
    }
}

#[derive(Debug)]
pub struct LayerRoutingMinSize {
    size1: LayerRoutingSize,
    size2: Vec<LayerRoutingSize>,
}
impl LayerRoutingMinSize {
    pub fn new(size1: LayerRoutingSize, size2: Vec<LayerRoutingSize>) -> Self {
        Self { size1, size2 }
    }
}

#[derive(Debug)]
pub enum LayerRoutingMinimumCutFrom {
    FROMABOVE,
    FROMBELOW,
}

#[derive(Debug)]
pub struct LayerRoutingMinimumCutLength {
    length: f32,
    within: f32,
}
impl LayerRoutingMinimumCutLength {
    pub fn new(length: f32, within: f32) -> Self {
        Self { length, within }
    }
}

#[derive(Debug)]
pub struct LayerRoutingMinimumCut {
    num_cuts: i32,
    width: f32,
    within: Option<f32>,
    from: Option<LayerRoutingMinimumCutFrom>,
    length: Option<LayerRoutingMinimumCutLength>,
}
impl LayerRoutingMinimumCut {
    pub fn new(
        n: i32,
        width: f32,
        within: Option<f32>,
        from: Option<LayerRoutingMinimumCutFrom>,
        length: Option<LayerRoutingMinimumCutLength>,
    ) -> Self {
        Self {
            num_cuts: n,
            width,
            within,
            from,
            length,
        }
    }
}

#[derive(Debug)]
pub enum LayerRoutingMinStepCorner {
    INSIDECORNER,
    OUTSIDECORNER,
    STEP,
}

#[derive(Debug)]
pub struct LayerRoutingMinStep {
    minstep: f32,
    lengthsum: Option<f32>,
    corner: Option<LayerRoutingMinStepCorner>,
    maxedges: Option<i32>,
}
impl LayerRoutingMinStep {
    pub fn new(
        minstep: f32,
        corner: Option<LayerRoutingMinStepCorner>,
        lengthsum: Option<f32>,
        maxedges: Option<i32>,
    ) -> Self {
        Self {
            minstep,
            lengthsum,
            corner,
            maxedges,
        }
    }
}

#[derive(Debug)]
pub struct LayerRoutingMinEnclosedArea {
    area: f32,
    width: Option<f32>,
}
impl LayerRoutingMinEnclosedArea {
    pub fn new(a: f32, w: Option<f32>) -> Self {
        Self { area: a, width: w }
    }
}

#[derive(Debug)]
pub struct LayerRoutingProtrusionWidth {
    width1: f32,
    length: f32,
    width2: f32,
}
impl LayerRoutingProtrusionWidth {
    pub fn new(w1: f32, l: f32, w2: f32) -> Self {
        Self {
            width1: w1,
            length: l,
            width2: w2,
        }
    }
}

#[derive(Debug)]
pub struct WidthLength {
    width: f32,
    length: f32,
}
impl WidthLength {
    pub fn new(w: f32, l: f32) -> Self {
        Self {
            width: w,
            length: l,
        }
    }
}

#[derive(Debug)]
pub struct LayerRoutingSpacingRangeInfluence {
    influence: f32,
    range: Option<Range<f32>>,
}
impl LayerRoutingSpacingRangeInfluence {
    pub fn new(influence: f32, range: Option<Range<f32>>) -> Self {
        Self { influence, range }
    }
}

#[derive(Debug)]
pub enum LayerRoutingSpacingRangeEnum {
    USELENGTHTHRESHOLD,
    INFLUENCE(LayerRoutingSpacingRangeInfluence),
    RANGE(Range<f32>),
}

#[derive(Debug)]
pub struct LayerRoutingSpacingRange {
    range: Range<f32>,
    rtype: Option<LayerRoutingSpacingRangeEnum>,
}
impl LayerRoutingSpacingRange {
    pub fn new(range: Range<f32>, rtype: Option<LayerRoutingSpacingRangeEnum>) -> Self {
        Self { range, rtype }
    }
}

#[derive(Debug)]
pub struct LayerRoutingSpacingLengthThreshold {
    maxlength: f32,
    range: Option<Range<f32>>,
}
impl LayerRoutingSpacingLengthThreshold {
    pub fn new(maxlength: f32, range: Option<Range<f32>>) -> Self {
        Self { maxlength, range }
    }
}

#[derive(Debug)]
pub struct LayerRoutingSpacingEndOfLineParallelEdge {
    width: f32,
    within: f32,
    twoedges: bool,
}
impl LayerRoutingSpacingEndOfLineParallelEdge {
    pub fn new(width: f32, within: f32, twoedges: bool) -> Self {
        Self {
            width,
            within,
            twoedges,
        }
    }
}

#[derive(Debug)]
pub struct LayerRoutingSpacingEndOfLine {
    width: f32,
    within: f32,
    paralleledge: Option<LayerRoutingSpacingEndOfLineParallelEdge>,
}
impl LayerRoutingSpacingEndOfLine {
    pub fn new(
        width: f32,
        within: f32,
        paralleledge: Option<LayerRoutingSpacingEndOfLineParallelEdge>,
    ) -> Self {
        Self {
            width,
            within,
            paralleledge,
        }
    }
}

#[derive(Debug)]
pub struct LayerRoutingSpacingSameNet {
    pgonly: bool,
}
impl LayerRoutingSpacingSameNet {
    pub fn new(pgonly: bool) -> Self {
        Self { pgonly }
    }
}

#[derive(Debug)]
pub struct LayerRoutingSpacingNotch {
    endofnotchwidth: f32,
    spacing: f32,
    length: f32,
}
impl LayerRoutingSpacingNotch {
    pub fn new(endofnotchwidth: f32, spacing: f32, length: f32) -> Self {
        Self {
            endofnotchwidth,
            spacing,
            length,
        }
    }
}

#[derive(Debug)]
pub enum LayerRoutingSpacingEnum {
    RANGE(LayerRoutingSpacingRange),
    LENGTHTHRESHOLD(LayerRoutingSpacingLengthThreshold),
    ENDOFLINE(LayerRoutingSpacingEndOfLine),
    SAMENET(LayerRoutingSpacingSameNet),
    NOTCHLENGTH(f32),
    NOTCH(LayerRoutingSpacingNotch),
}

#[derive(Debug)]
pub struct LayerRoutingSpacing {
    spacing: f32,
    stype: Option<LayerRoutingSpacingEnum>,
}
impl LayerRoutingSpacing {
    pub fn new(spacing: f32, stype: Option<LayerRoutingSpacingEnum>) -> Self {
        Self { spacing, stype }
    }
}

#[derive(Debug)]
pub struct LayerRoutingSpacingTableParallelWidth {
    width: f32,
    spacings: Vec<f32>,
}
impl LayerRoutingSpacingTableParallelWidth {
    pub fn new(width: f32, spacings: Vec<f32>) -> Self {
        Self { width, spacings }
    }
}

#[derive(Debug)]
pub struct LayerRoutingSpacingTableParallelSpacingTableWidth {
    width: f32,
    within: f32,
    spacing: f32,
}
impl LayerRoutingSpacingTableParallelSpacingTableWidth {
    pub fn new(width: f32, within: f32, spacing: f32) -> Self {
        Self {
            width,
            within,
            spacing,
        }
    }
}

#[derive(Debug)]
pub struct LayerRoutingSpacingTableParallelSpacingTable {
    widths: Vec<LayerRoutingSpacingTableParallelSpacingTableWidth>,
}
impl LayerRoutingSpacingTableParallelSpacingTable {
    pub fn new(widths: Vec<LayerRoutingSpacingTableParallelSpacingTableWidth>) -> Self {
        Self { widths }
    }
}

#[derive(Debug)]
pub struct LayerRoutingSpacingTableParallel {
    lengths: Vec<f32>,
    widths: Vec<LayerRoutingSpacingTableParallelWidth>,
    spacingtable: Option<LayerRoutingSpacingTableParallelSpacingTable>,
}
impl LayerRoutingSpacingTableParallel {
    pub fn new(
        lengths: Vec<f32>,
        widths: Vec<LayerRoutingSpacingTableParallelWidth>,
        spacingtable: Option<LayerRoutingSpacingTableParallelSpacingTable>,
    ) -> Self {
        Self {
            lengths,
            widths,
            spacingtable,
        }
    }
}

#[derive(Debug)]
pub struct LayerRoutingSpacingTableTwoWidthsWidth {
    width: f32,
    runlength: Option<f32>,
    spacings: Vec<f32>,
}
impl LayerRoutingSpacingTableTwoWidthsWidth {
    pub fn new(width: f32, runlength: Option<f32>, spacings: Vec<f32>) -> Self {
        Self {
            width,
            runlength,
            spacings,
        }
    }
}

#[derive(Debug)]
pub struct LayerRoutingSpacingTableTwoWidths {
    widths: Vec<LayerRoutingSpacingTableTwoWidthsWidth>,
}
impl LayerRoutingSpacingTableTwoWidths {
    pub fn new(widths: Vec<LayerRoutingSpacingTableTwoWidthsWidth>) -> Self {
        Self { widths }
    }
}

#[derive(Debug)]
pub enum LayerRoutingSpacingTable {
    PARALLEL(LayerRoutingSpacingTableParallel),
    TWOWIDTHS(LayerRoutingSpacingTableTwoWidths),
}

#[derive(Debug, Default)]
pub struct LayerRoutingOpts {
    pub mask: Option<i32>,
    pub direction: Option<LayerRoutingDirection>,
    pub pitch: Option<RangeOrValue<f32>>,
    pub diagpitch: Option<RangeOrValue<f32>>,
    pub width: Option<f32>,
    pub diagwidth: Option<f32>,
    pub diagspacing: Option<f32>,
    pub offset: Option<RangeOrValue<f32>>,
    pub diagminedgelength: Option<f32>,
    pub area: Option<f32>,
    pub minsize: Option<LayerRoutingMinSize>,

    pub spacings: Option<Vec<LayerRoutingSpacing>>,
    pub spacingtable: Option<LayerRoutingSpacingTable>,
    pub arrayspacing: Option<LayerArraySpacing>,

    pub wireextension: Option<f32>,
    pub minimumcut: Option<LayerRoutingMinimumCut>,
    pub maxwidth: Option<f32>,
    pub minwidth: Option<f32>,
    pub minstep: Option<LayerRoutingMinStep>,
    pub minenclosedarea: Option<LayerRoutingMinEnclosedArea>,
    pub protrusionwidth: Option<LayerRoutingProtrusionWidth>,
    pub resistance: Option<PieceWiseLinear>,
    pub capacitance: Option<PieceWiseLinear>,
    pub height: Option<f32>,
    pub thickness: Option<f32>,
    pub shrinkage: Option<f32>,
    pub capmultiplier: Option<f32>,
    pub edgecapacitance: Option<f32>,
    pub minimumdensity: Option<f32>,
    pub maximumdensity: Option<f32>,
    pub densitycheckwindow: Option<WidthLength>,
    pub densitycheckstep: Option<f32>,
    pub fillactivespacing: Option<f32>,

    pub antennamodels: Option<Vec<AntennaModel>>,
    pub antennaarearatios: Option<Vec<f32>>,
    pub antennasidearearatios: Option<Vec<f32>>,
    pub antennacumsidearearatios: Option<Vec<f32>>,
    pub antennadiffsidearearatios: Option<Vec<PieceWiseLinear>>,
    pub antennacumdiffsidearearatios: Option<Vec<PieceWiseLinear>>,
    pub antennadiffarearatios: Option<Vec<PieceWiseLinear>>,
    pub antennacumarearatios: Option<Vec<f32>>,
    pub antennacumdiffarearatios: Option<Vec<PieceWiseLinear>>,
    pub antennaareafactors: Option<Vec<LayerAntennaAreaFactor>>,
    pub antennasideareafactors: Option<Vec<LayerAntennaAreaFactor>>,

    pub antennacumroutingpluscut: Option<bool>,
    pub antennagateplusdiff: Option<f32>,
    pub antennaareaminusdiff: Option<f32>,
    pub antennaareadiffreducepwl: Option<Vec<(f32, f32)>>,

    pub properties: Option<Vec<Property>>,
    pub accurrentdensity: Option<ACCurrentDensity>,
    pub dccurrentdensity: Option<DCCurrentDensity>,
}
impl LayerRoutingOpts {
    pub fn copy(&mut self, opts: LayerRoutingOpts) {
        copy_opt![self, mask, opts];
        copy_opt![self, direction, opts];
        copy_opt![self, pitch, opts];
        copy_opt![self, diagpitch, opts];
        copy_opt![self, width, opts];
        copy_opt![self, diagwidth, opts];
        copy_opt![self, offset, opts];
        copy_opt![self, diagminedgelength, opts];
        copy_opt![self, area, opts];
        copy_opt![self, minsize, opts];

        copy_vec_opt![self, spacings, opts];
        copy_opt![self, spacingtable, opts];
        copy_opt![self, arrayspacing, opts];

        copy_opt![self, wireextension, opts];
        copy_opt![self, minimumcut, opts];
        copy_opt![self, maxwidth, opts];
        copy_opt![self, minwidth, opts];
        copy_opt![self, minstep, opts];
        copy_opt![self, minenclosedarea, opts];
        copy_opt![self, protrusionwidth, opts];
        copy_opt![self, resistance, opts];
        copy_opt![self, capacitance, opts];
        copy_opt![self, height, opts];
        copy_opt![self, thickness, opts];
        copy_opt![self, shrinkage, opts];
        copy_opt![self, capmultiplier, opts];
        copy_opt![self, edgecapacitance, opts];
        copy_opt![self, minimumdensity, opts];
        copy_opt![self, maximumdensity, opts];
        copy_opt![self, densitycheckwindow, opts];
        copy_opt![self, densitycheckstep, opts];
        copy_opt![self, fillactivespacing, opts];

        copy_vec_opt![self, antennamodels, opts];
        copy_vec_opt![self, antennaarearatios, opts];
        copy_vec_opt![self, antennasidearearatios, opts];
        copy_vec_opt![self, antennacumsidearearatios, opts];
        copy_vec_opt![self, antennacumdiffsidearearatios, opts];
        copy_vec_opt![self, antennadiffarearatios, opts];
        copy_vec_opt![self, antennacumarearatios, opts];
        copy_vec_opt![self, antennacumdiffarearatios, opts];
        copy_vec_opt![self, antennaareafactors, opts];
        copy_vec_opt![self, antennasideareafactors, opts];

        copy_opt![self, antennacumroutingpluscut, opts];
        copy_opt![self, antennagateplusdiff, opts];
        copy_opt![self, antennaareaminusdiff, opts];
        copy_vec_opt![self, antennaareadiffreducepwl, opts];

        copy_vec_opt![self, properties, opts];
        copy_opt![self, accurrentdensity, opts];
        copy_opt![self, dccurrentdensity, opts];
    }
}

#[derive(Debug)]
pub struct LayerRouting {
    name: String,
    opts: LayerRoutingOpts,
}
impl LayerRouting {
    pub fn new(name: String, opts: LayerRoutingOpts, chk_name: String) -> Self {
        if name != chk_name {
            panic!("{} is different from {}", name, chk_name);
        }
        Self { name, opts }
    }
}

/////////////////////////////////////////////////////////////////////////////////

/////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct LayerOpts {
    pub mask: Option<i32>,
    pub direction: Option<LayerRoutingDirection>,
    pub pitch: Option<RangeOrValue<f32>>,
    pub diagpitch: Option<RangeOrValue<f32>>,
    pub width: Option<f32>,
    pub offset: Option<RangeOrValue<f32>>,
    pub diagwidth: Option<f32>,
    pub diagspacing: Option<f32>,
    pub diagminedgelength: Option<f32>,
    pub area: Option<f32>,
    pub minsize: Option<LayerRoutingMinSize>,

    pub enclosures: Option<Vec<LayerEnclosure>>,
    pub preferenclosures: Option<Vec<LayerEnclosure>>,

    pub spacings: Option<Vec<LayerCutSpacing>>,
    pub spacingtable: Option<LayerCutSpacingTable>,

    pub wireextension: Option<f32>,
    pub minimumcut: Option<LayerRoutingMinimumCut>,
    pub maxwidth: Option<f32>,
    pub minwidth: Option<f32>,
    pub minstep: Option<LayerRoutingMinStep>,
    pub minenclosedarea: Option<LayerRoutingMinEnclosedArea>,
    pub protrusionwidth: Option<LayerRoutingProtrusionWidth>,
    pub resistance: Option<PieceWiseLinear>,
    pub capacitance: Option<PieceWiseLinear>,
    pub height: Option<f32>,
    pub thickness: Option<f32>,
    pub shrinkage: Option<f32>,
    pub capmultiplier: Option<f32>,
    pub edgecapacitance: Option<f32>,
    pub minimumdensity: Option<f32>,
    pub maximumdensity: Option<f32>,
    pub densitycheckwindow: Option<WidthLength>,
    pub densitycheckstep: Option<f32>,
    pub fillactivespacing: Option<f32>,

    pub antennamodels: Option<Vec<AntennaModel>>,
    pub antennaarearatios: Option<Vec<f32>>,
    pub antennasidearearatios: Option<Vec<f32>>,
    pub antennacumsidearearatios: Option<Vec<f32>>,
    pub antennadiffsidearearatios: Option<Vec<PieceWiseLinear>>,
    pub antennacumdiffsidearearatios: Option<Vec<PieceWiseLinear>>,
    pub antennadiffarearatios: Option<Vec<PieceWiseLinear>>,
    pub antennacumarearatios: Option<Vec<f32>>,
    pub antennacumdiffarearatios: Option<Vec<PieceWiseLinear>>,
    pub antennaareafactors: Option<Vec<LayerAntennaAreaFactor>>,
    pub antennasideareafactors: Option<Vec<LayerAntennaAreaFactor>>,

    pub antennacumroutingpluscut: Option<bool>,
    pub antennagateplusdiff: Option<f32>,
    pub antennaareaminusdiff: Option<f32>,
    pub antennaareadiffreducepwl: Option<Vec<(f32, f32)>>,

    pub properties: Option<Vec<Property>>,
    pub accurrentdensity: Option<ACCurrentDensity>,
    pub dccurrentdensity: Option<DCCurrentDensity>,
}

#[derive(Debug)]
pub enum Layer {
    Cut(LayerCut),
    Implant(LayerImplant),
    MasterSlice(LayerMasterSlice),
    Routing(LayerRouting),
}

/////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}
impl Size {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

#[derive(Debug)]
pub struct ViaViaRuleLayers {
    bot: String,
    cut: String,
    top: String,
}
impl ViaViaRuleLayers {
    pub fn new(bot: String, cut: String, top: String) -> Self {
        Self { bot, cut, top }
    }
}

#[derive(Debug)]
pub struct ViaViaRuleEnclosure {
    xbot: f32,
    ybot: f32,
    xtop: f32,
    ytop: f32,
}
impl ViaViaRuleEnclosure {
    pub fn new(xbot: f32, ybot: f32, xtop: f32, ytop: f32) -> Self {
        Self {
            xbot,
            ybot,
            xtop,
            ytop,
        }
    }
}

#[derive(Debug)]
pub struct ViaViaRuleRowCol {
    rows: i32,
    cols: i32,
}
impl ViaViaRuleRowCol {
    pub fn new(rows: i32, cols: i32) -> Self {
        Self { rows, cols }
    }
}

#[derive(Debug)]
pub struct ViaViaRuleOrigin {
    xoffset: f32,
    yoffset: f32,
}
impl ViaViaRuleOrigin {
    pub fn new(xoffset: f32, yoffset: f32) -> Self {
        Self { xoffset, yoffset }
    }
}

#[derive(Debug)]
pub struct ViaViaRuleOffset {
    xbotoffset: f32,
    ybotoffset: f32,
    xtopoffset: f32,
    ytopoffset: f32,
}
impl ViaViaRuleOffset {
    pub fn new(xbotoffset: f32, ybotoffset: f32, xtopoffset: f32, ytopoffset: f32) -> Self {
        Self {
            xbotoffset,
            ybotoffset,
            xtopoffset,
            ytopoffset,
        }
    }
}

#[derive(Debug, Default)]
pub struct ViaViaRuleOpts {
    pub cutsize: Option<Size>,
    pub layers: Option<ViaViaRuleLayers>,
    pub cutspacing: Option<Size>,
    pub enclosure: Option<ViaViaRuleEnclosure>,
    pub rowcol: Option<ViaViaRuleRowCol>,
    pub origin: Option<ViaViaRuleOrigin>,
    pub offset: Option<ViaViaRuleOffset>,
    pub pattern: Option<String>,
}

#[derive(Debug)]
pub struct ViaViaRule {
    vianame: String,
    opts: ViaViaRuleOpts,
}
impl ViaViaRule {
    pub fn new(vianame: String, opts: ViaViaRuleOpts) -> Self {
        if opts.cutsize.is_none()
            || opts.layers.is_none()
            || opts.cutspacing.is_none()
            || opts.enclosure.is_none()
        {
            panic!("VIARULE of VIA {} is incomplete", vianame)
        }
        Self { vianame, opts }
    }
}

#[derive(Debug)]
pub enum ShapeEnum {
    RECT((f32, f32), (f32, f32)),
    POLYGON(Vec<f32>),
}

#[derive(Debug)]
pub struct Shape {
    mask: Option<i32>,
    shape: ShapeEnum,
}
impl Shape {
    pub fn new(mask: Option<i32>, shape: ShapeEnum) -> Self {
        Self { mask, shape }
    }
}

#[derive(Debug)]
pub struct ViaLayer {
    name: String,
    shapes: Vec<Shape>,
}
impl ViaLayer {
    pub fn new(name: String, shapes: Vec<Shape>) -> Self {
        Self { name, shapes }
    }
}

#[derive(Debug)]
pub struct ViaFixed {
    foreign: Option<Foreign>,
    resistance: Option<f32>,
    layers: Vec<ViaLayer>,
}
impl ViaFixed {
    pub fn new(foreign: Option<Foreign>, resistance: Option<f32>, layers: Vec<ViaLayer>) -> Self {
        Self {
            foreign,
            resistance,
            layers,
        }
    }
}

#[derive(Debug)]
pub enum ViaType {
    VIARULE(ViaViaRule),
    FIXED(ViaFixed),
}

#[derive(Debug)]
pub struct Via {
    name: String,
    default: bool,
    generated: bool,
    topofstackonly: bool,
    vtype: ViaType,
    properties: Vec<Property>,
}
impl Via {
    pub fn new(
        name: String,
        default: bool,
        generated: bool,
        topofstackonly: bool,
        vtype: ViaType,
        properties: Vec<Property>,
        chk_name: String,
    ) -> Self {
        if name != chk_name {
            panic!("{} is different from {}", name, chk_name);
        }
        Self {
            name,
            default,
            generated,
            topofstackonly,
            vtype,
            properties,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum ViaRuleLayerDirection {
    HORIZONTAL,
    VERTICAL,
}
#[derive(Debug, Default)]
pub struct ViaRuleLayerOpts {
    pub direction: Option<ViaRuleLayerDirection>,
    pub enclosure: Option<Pair<f32>>,
    pub widthrange: Option<Range<f32>>,
    pub overhang: Option<f32>,
    pub metaloverhang: Option<f32>,
}

#[derive(Debug)]
pub struct ViaRuleLayer {
    name: String,
    opts: ViaRuleLayerOpts,
}
impl ViaRuleLayer {
    pub fn new(name: String, opts: ViaRuleLayerOpts) -> Self {
        Self { name, opts }
    }
}

#[derive(Debug)]
pub struct ViaRuleGenerateCutLayer {
    name: String,
    rect: Rect,
    spacing: (f32, f32),
    resistance: Option<f32>,
}
impl ViaRuleGenerateCutLayer {
    pub fn new(name: String, rect: Rect, spacing: (f32, f32), resistance: Option<f32>) -> Self {
        Self {
            name,
            rect,
            spacing,
            resistance,
        }
    }
}

#[derive(Debug)]
pub struct ViaRule {
    name: String,
    generate: bool,
    default: bool,
    layer1: ViaRuleLayer,
    layer2: ViaRuleLayer,
    cutlayer: Option<ViaRuleGenerateCutLayer>,
    vias: Vec<String>,
    properties: Vec<Property>,
}
impl ViaRule {
    pub fn new(
        name: String,
        generate: bool,
        default: bool,
        layer1: ViaRuleLayer,
        layer2: ViaRuleLayer,
        cutlayer: Option<ViaRuleGenerateCutLayer>,
        vias: Vec<String>,
        properties: Vec<Property>,
        chk_name: String,
    ) -> Self {
        if name != chk_name {
            panic!("{} is different from {}", name, chk_name);
        }
        Self {
            name,
            generate,
            default,
            layer1,
            layer2,
            cutlayer,
            vias,
            properties,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//  MACRO                                                                     //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct MacroClassCover {
    bump: bool,
}
impl MacroClassCover {
    pub fn new(bump: bool) -> Self {
        Self { bump }
    }
}

#[derive(Debug)]
pub enum MacroClassBlockEnum {
    BLACKBOX,
    SOFT,
}

#[derive(Debug)]
pub struct MacroClassBlock {
    btype: Option<MacroClassBlockEnum>,
}
impl MacroClassBlock {
    pub fn new(btype: Option<MacroClassBlockEnum>) -> Self {
        Self { btype }
    }
}

#[derive(Debug)]
pub enum MacroClassPadEnum {
    INPUT,
    OUTPUT,
    INOUT,
    POWER,
    SPACER,
    AREAIO,
}

#[derive(Debug)]
pub struct MacroClassPad {
    ptype: Option<MacroClassPadEnum>,
}
impl MacroClassPad {
    pub fn new(ptype: Option<MacroClassPadEnum>) -> Self {
        Self { ptype }
    }
}

#[derive(Debug)]
pub enum MacroClassCoreEnum {
    FEEDTHRU,
    TIEHIGH,
    TIELOW,
    SPACER,
    ANTENNACELL,
    WELLTAP,
}

#[derive(Debug)]
pub struct MacroClassCore {
    ctype: Option<MacroClassCoreEnum>,
}
impl MacroClassCore {
    pub fn new(ctype: Option<MacroClassCoreEnum>) -> Self {
        Self { ctype }
    }
}

#[derive(Debug)]
pub enum MacroClassEndCap {
    PRE,
    POST,
    TOPLEFT,
    TOPRIGHT,
    BOTTOMLEFT,
    BOTTOMRIGHT,
}

#[derive(Debug)]
pub enum MacroClass {
    COVER(MacroClassCover),
    RING,
    BLOCK(MacroClassBlock),
    PAD(MacroClassPad),
    CORE(MacroClassCore),
    ENDCAP(MacroClassEndCap),
}

#[derive(Debug)]
pub struct PointOrient {
    pub x: f32,
    pub y: f32,
    pub orient: Option<Orient>,
}
impl PointOrient {
    pub fn new(x: f32, y: f32, orient: Option<Orient>) -> Self {
        Self { x, y, orient }
    }
}
pub type Point = PointOrient;
//impl Point {
//    pub fn new_def_orient(x: f32, y: f32) -> Self {
//        Self { x, y, orient: Some(Orient::N) }
//    }
//}

#[derive(Debug)]
pub struct Rect {
    // left bottom
    pub lb: Point,
    // right top
    pub rt: Point,
}
impl Rect {
    pub fn new(point1: Point, point2: Point) -> Self {
        Self {
            lb: point1,
            rt: point2,
        }
    }
}

#[derive(Debug)]
pub struct Polygon {
    polygon: Vec<Point>,
}
impl Polygon {
    pub fn new(polygon: Vec<Point>) -> Self {
        if polygon.len() < 4 {
            panic!("Polygon needs at least 4 points")
        }
        Self { polygon }
    }
}

#[derive(Debug)]
pub struct Foreign {
    name: String,
    point: Option<PointOrient>,
}
impl Foreign {
    pub fn new(name: String, point: Option<PointOrient>) -> Self {
        Self { name, point }
    }
}

#[derive(Debug)]
pub struct StepPattern {
    xcount: i32,
    ycount: i32,
    xstep: f32,
    ystep: f32,
}
impl StepPattern {
    pub fn new(xcount: i32, ycount: i32, xstep: f32, ystep: f32) -> Self {
        Self {
            xcount,
            ycount,
            xstep,
            ystep,
        }
    }
}

#[derive(Debug)]
pub struct SitePattern {
    xorigin: f32,
    yorigin: f32,
    orient: Orient,
    steppattern: Option<StepPattern>,
}
impl SitePattern {
    pub fn new(
        xorigin: f32,
        yorigin: f32,
        orient: Orient,
        steppattern: Option<StepPattern>,
    ) -> Self {
        Self {
            xorigin,
            yorigin,
            orient,
            steppattern,
        }
    }
}

#[derive(Debug)]
pub enum Symmetry {
    X,
    Y,
    R90,
}

#[derive(Debug)]
pub struct RowPattern {
    sitename: String,
    orient: Orient,
}
impl RowPattern {
    pub fn new(sitename: String, orient: Orient) -> Self {
        Self { sitename, orient }
    }
}

#[derive(Debug)]
pub struct MacroSite {
    name: String,
    sitepattern: Option<SitePattern>,
}
impl MacroSite {
    pub fn new(name: String, sitepattern: Option<SitePattern>) -> Self {
        Self { name, sitepattern }
    }
}

#[derive(Debug)]
pub struct MacroPinDirectionOutput {
    tristate: bool,
}
impl MacroPinDirectionOutput {
    pub fn new(tristate: bool) -> Self {
        Self { tristate }
    }
}

#[derive(Debug)]
pub enum MacroPinDirection {
    INPUT,
    OUTPUT(MacroPinDirectionOutput),
    INOUT,
    FEEDTHRU,
}

#[derive(Debug)]
pub enum MacroPinUse {
    SIGNAL,
    ANALOG,
    POWER,
    GROUND,
    CLOCK,
}

#[derive(Debug)]
pub struct MacroPinNetExpr {
    prop_name: String,
    default_name: String,
}
impl MacroPinNetExpr {
    pub fn new(prop_name: String, default_name: String) -> Self {
        Self {
            prop_name,
            default_name,
        }
    }
}

#[derive(Debug)]
pub enum MacroPinShape {
    ABUTMENT,
    RING,
    FEEDTHRU,
}

#[derive(Debug)]
pub enum MacroPinPortClass {
    NONE,
    CORE,
    BUMP,
}

#[derive(Debug)]
pub enum LayerGeomDesign {
    SPACING(f32),
    DESIGNRULEWIDTH(f32),
}

#[derive(Debug)]
pub struct LayerGeomShapePath {
    mask: Option<i32>,
    points: Vec<Point>,
    steppattern: Option<StepPattern>,
}
impl LayerGeomShapePath {
    pub fn new(mask: Option<i32>, points: Vec<Point>, steppattern: Option<StepPattern>) -> Self {
        Self {
            mask,
            points,
            steppattern,
        }
    }
}

#[derive(Debug)]
pub struct LayerGeomShapeRect {
    mask: Option<i32>,
    rect: Rect,
    steppattern: Option<StepPattern>,
}
impl LayerGeomShapeRect {
    pub fn new(mask: Option<i32>, rect: Rect, steppattern: Option<StepPattern>) -> Self {
        Self {
            mask,
            rect,
            steppattern,
        }
    }
}

#[derive(Debug)]
pub struct LayerGeomShapePolygon {
    mask: Option<i32>,
    polygon: Polygon,
    steppattern: Option<StepPattern>,
}
impl LayerGeomShapePolygon {
    pub fn new(mask: Option<i32>, polygon: Polygon, steppattern: Option<StepPattern>) -> Self {
        Self {
            mask,
            polygon,
            steppattern,
        }
    }
}

#[derive(Debug)]
pub enum LayerGeomShape {
    PATH(LayerGeomShapePath),
    RECT(LayerGeomShapeRect),
    POLYGON(LayerGeomShapePolygon),
}

#[derive(Debug)]
pub struct LayerGeomLayer {
    name: String,
    exceptpgnet: bool,
    spacing_designrulewidth: Option<LayerGeomDesign>,
    width: Option<f32>,
    shapes: Vec<LayerGeomShape>,
}
impl LayerGeomLayer {
    pub fn new(
        name: String,
        exceptpgnet: bool,
        spacing_designrulewidth: Option<LayerGeomDesign>,
        width: Option<f32>,
        shapes: Vec<LayerGeomShape>,
    ) -> Self {
        Self {
            name,
            exceptpgnet,
            spacing_designrulewidth,
            width,
            shapes,
        }
    }
}

#[derive(Debug)]
pub struct LayerGeomVia {
    mask: Option<i32>,
    point: Point,
    name: String,
    steppattern: Option<StepPattern>,
}
impl LayerGeomVia {
    pub fn new(
        mask: Option<i32>,
        point: Point,
        name: String,
        steppattern: Option<StepPattern>,
    ) -> Self {
        Self {
            mask,
            point,
            name,
            steppattern,
        }
    }
}

#[derive(Debug)]
pub enum LayerGeom {
    LAYER(LayerGeomLayer),
    VIA(LayerGeomVia),
}

#[derive(Debug)]
pub struct MacroPinPort {
    pub class: Option<MacroPinPortClass>,
    pub geometries: Vec<LayerGeom>,
}
impl MacroPinPort {
    pub fn new(class: Option<MacroPinPortClass>, geometries: Vec<LayerGeom>) -> Self {
        Self { class, geometries }
    }
    // simple function to abstract pin as a rect (x,y,w,h)
    pub fn get_bbox(&self) -> Result<BBox<f32>> {
        let mut xmin = f32::INFINITY;
        let mut ymin = f32::INFINITY;
        let mut xmax = f32::NEG_INFINITY;
        let mut ymax = f32::NEG_INFINITY;

        let mut found_any = false;

        for port_geom in &self.geometries {
            match port_geom {
                LayerGeom::LAYER(geom) => {
                    for shape in &geom.shapes {
                        match shape {
                            LayerGeomShape::RECT(layer_rect) => {
                                let rect = &layer_rect.rect;
                                let xl = rect.lb.x;
                                let yl = rect.lb.y;
                                let xh = rect.rt.x;
                                let yh = rect.rt.y;
                                xmin = xmin.min(xl);
                                ymin = ymin.min(yl);
                                xmax = xmax.max(xh);
                                ymax = ymax.max(yh);

                                found_any = true;
                            }
                            _ => return Err(eyre!("Port Geometry Shape not supported")),
                        }
                    }
                }
                _ => return Err(eyre!("Port Geometry not supported")),
            }
        }

        if !found_any || xmin == f32::INFINITY {
            return Err(eyre::eyre!("Invalid bbox: no geometry found for pin port",));
        }

        let w = xmax - xmin;
        let h = ymax - ymin;

        if w < 0.0 || h < 0.0 {
            return Err(eyre::eyre!("Invalid bbox computed for pin port"));
        }

        Ok(BBox::new(xmin, ymin, w, h))
    }
}

#[derive(Debug, Default)]
pub struct MacroPinOpts {
    pub taperrule: Option<String>,
    pub direction: Option<MacroPinDirection>,
    // use
    pub puse: Option<MacroPinUse>,
    pub netexpr: Option<MacroPinNetExpr>,
    pub supplysensitivity: Option<String>,
    pub groundsensitivity: Option<String>,
    pub shape: Option<MacroPinShape>,
    pub mustjoin: Option<String>,
    pub antennapartialmetalarea: Option<AntennaPartialMetalArea>,
    pub antennapartialmetalsidearea: Option<AntennaPartialMetalSideArea>,
    pub antennapartialcutarea: Option<AntennaPartialCutArea>,
    pub antennadiffarea: Option<AntennaDiffArea>,
    pub antennamodel: Option<AntennaModel>,
    pub antennagatearea: Option<AntennaGateArea>,
    pub antennamaxareacar: Option<AntennaMaxAreaCar>,
    pub antennamaxsideareacar: Option<AntennaMaxSideAreaCar>,
    pub antennamaxcutcar: Option<AntennaMaxCutCar>,

    pub ports: Option<Vec<MacroPinPort>>,
    pub properties: Option<Vec<Property>>,
}

#[derive(Debug)]
pub struct MacroPin {
    pub name: String,
    pub opts: MacroPinOpts,
}
impl MacroPin {
    pub fn new(name: String, opts: MacroPinOpts, chk_name: String) -> Self {
        if name != chk_name {
            panic!("{} is different from {}", name, chk_name);
        }
        Self { name, opts }
    }

    pub fn get_bbox(&self) -> Result<BBox<f32>> {
        let ports = self
            .opts
            .ports
            .as_ref()
            .ok_or_eyre(format!("No ports in Pin {}", self.name))?;

        let mut xmin = f32::INFINITY;
        let mut ymin = f32::INFINITY;
        let mut xmax = f32::NEG_INFINITY;
        let mut ymax = f32::NEG_INFINITY;

        let mut found_any = false;

        for port in ports {
            match port.get_bbox() {
                Ok(rect) => {
                    let xl = rect.xl as f32;
                    let yl = rect.yl as f32;
                    let w = rect.w as f32;
                    let h = rect.h as f32;
                    let xh = xl + w;
                    let yh = yl + h;

                    xmin = xmin.min(xl);
                    ymin = ymin.min(yl);
                    xmax = xmax.max(xh);
                    ymax = ymax.max(yh);

                    found_any = true;
                }
                _ => {}
            }
        }

        if !found_any || xmin == f32::INFINITY {
            return Err(eyre::eyre!(
                "Invalid bbox: no geometry found for pin {}",
                self.name
            ));
        }

        let w = xmax - xmin;
        let h = ymax - ymin;

        if w < 0.0 || h < 0.0 {
            return Err(eyre::eyre!("Invalid bbox computed for pin {}", self.name));
        }

        Ok(BBox::new(xmin, ymin, w, h))
    }
}

#[derive(Debug)]
pub struct MacroObstruction {
    layergeoms: Vec<LayerGeom>,
}
impl MacroObstruction {
    pub fn new(layergeoms: Vec<LayerGeom>) -> Self {
        Self { layergeoms }
    }
}

#[derive(Debug)]
pub struct MacroDensityLayerRect {
    point1: Point,
    point2: Point,
    density: f32,
}
impl MacroDensityLayerRect {
    pub fn new(point1: Point, point2: Point, density: f32) -> Self {
        Self {
            point1,
            point2,
            density,
        }
    }
}

#[derive(Debug)]
pub struct MacroDensityLayer {
    name: String,
    rects: Vec<MacroDensityLayerRect>,
}
impl MacroDensityLayer {
    pub fn new(name: String, rects: Vec<MacroDensityLayerRect>) -> Self {
        Self { name, rects }
    }
}

#[derive(Debug)]
pub struct MacroDensity {
    layers: Vec<MacroDensityLayer>,
}
impl MacroDensity {
    pub fn new(layers: Vec<MacroDensityLayer>) -> Self {
        Self { layers }
    }
}

#[derive(Debug, Default)]
pub struct MacroOpts {
    pub class: Option<MacroClass>,
    pub fixedmask: Option<bool>,
    pub foreigns: Option<Vec<Foreign>>,
    pub origin: Option<Point>,
    pub eeq: Option<String>,
    pub size: Option<Size>,
    // TODO: add check / remove duplicates
    pub symmetries: Option<Vec<Symmetry>>,
    pub sites: Option<Vec<MacroSite>>,
    pub pins: Option<Vec<MacroPin>>,
    pub obstructions: Option<Vec<MacroObstruction>>,
    pub densities: Option<Vec<MacroDensity>>,
    pub properties: Option<Vec<Property>>,
}

#[derive(Debug)]
pub struct Macro {
    pub name: String,
    pub opts: MacroOpts,
}
impl Macro {
    pub fn new(name: String, opts: MacroOpts, chk_name: String) -> Self {
        if name != chk_name {
            panic!("{} is different from {}", name, chk_name);
        }
        Self { name, opts }
    }

    pub fn is_macro(&self) -> bool {
        matches!(self.opts.class, Some(MacroClass::BLOCK(_)))
    }

    pub fn get_pin(&self, pin_name: &str) -> Option<&MacroPin> {
        if let Some(ref pins) = self.opts.pins {
            for pin in pins {
                if pin.name == pin_name {
                    return Some(pin);
                }
            }
        }

        None
    }

    pub fn is_in_site(&self, site_name: Option<String>) -> bool {
        match site_name {
            None => true,
            Some(site_name) => self
                .opts
                .sites
                .as_ref()
                .is_some_and(|sites| sites.iter().any(|s| s.name == site_name)),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//  End MACRO                                                                 //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

/////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum PropertyDefinitionType {
    LAYER,
    LIBRARY,
    MACRO,
    NONDEFAULTRULE,
    PIN,
    VIA,
    VIARULE,
}

#[derive(Debug)]
pub enum PropertyDefinitionPropType {
    INTEGER,
    REAL,
    STRING,
}

#[derive(Debug)]
pub struct PropertyDefinition {
    otype: PropertyDefinitionType,
    name: String,
    value: RangeOrValueEnum,
}

pub fn units_time(u: &mut Units, n: i32) {
    u.time = Some(n);
}

pub fn units_capacitance(u: &mut Units, n: i32) {
    u.capacitance = Some(n);
}

pub fn units_resistance(u: &mut Units, n: i32) {
    u.resistance = Some(n);
}

pub fn units_power(u: &mut Units, n: i32) {
    u.power = Some(n);
}

pub fn units_current(u: &mut Units, n: i32) {
    u.current = Some(n);
}

pub fn units_voltage(u: &mut Units, n: i32) {
    u.voltage = Some(n);
}

pub fn units_database(u: &mut Units, n: i32) {
    u.database = Some(n);
}

pub fn units_frequency(u: &mut Units, n: i32) {
    u.frequency = Some(n);
}

pub fn propertydefinition(
    _ot: PropertyDefinitionType,
    _name: String,
    _v: RangeOrValueEnum,
) -> PropertyDefinition {
    PropertyDefinition {
        otype: _ot,
        name: _name,
        value: _v,
    }
}

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//  NONDEFAULTRULE                                                            //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct MinCut {
    cutlayername: String,
    numcuts: i32,
}
impl MinCut {
    pub fn new(cutlayername: String, numcuts: i32) -> Self {
        Self {
            cutlayername,
            numcuts,
        }
    }
}

#[derive(Debug, Default)]
pub struct NonDefaultRuleLayerOpts {
    pub diagwidth: Option<f32>,
    pub spacing: Option<f32>,
    pub wireextension: Option<f32>,
}

#[derive(Debug)]
pub struct NonDefaultRuleLayer {
    name: String,
    width: f32,
    opts: NonDefaultRuleLayerOpts,
    //opts: LayerRoutingOpts,
    //opts: LayerOpts,
}
impl NonDefaultRuleLayer {
    pub fn new(name: String, width: f32, opts: NonDefaultRuleLayerOpts, chk_name: String) -> Self {
        if name != chk_name {
            panic!("{} is different from {}", name, chk_name);
        }
        Self { name, width, opts }
    }
}

#[derive(Debug, Default)]
pub struct NonDefaultRuleOpts {
    pub hardspacing: Option<bool>,
    pub layers: Option<Vec<NonDefaultRuleLayer>>,
    pub vias: Option<Vec<Via>>,
    pub usevias: Option<Vec<String>>,
    pub useviarules: Option<Vec<String>>,
    pub mincuts: Option<Vec<MinCut>>,
    pub properties: Option<Vec<Property>>,
}

#[derive(Debug)]
pub struct NonDefaultRule {
    name: String,
    opts: NonDefaultRuleOpts,
}
impl NonDefaultRule {
    pub fn new(name: String, opts: NonDefaultRuleOpts, chk_name: String) -> Self {
        if name != chk_name {
            panic!("{} is different from {}", name, chk_name);
        }
        Self { name, opts }
    }
}

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//  End NONDEFAULTRULE                                                            //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//  SPACING
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Spacing {
    samenet: Option<bool>,
    layer1: String,
    layer2: String,
    spacing: f32,
    stack: Option<bool>,
}
impl Spacing {
    pub fn new(
        samenet: Option<bool>,
        layer1: String,
        layer2: String,
        spacing: f32,
        stack: Option<bool>,
    ) -> Self {
        Self {
            samenet,
            layer1,
            layer2,
            spacing,
            stack,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//  End SPACING
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//  Site                                                                      //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum SiteClass {
    PAD,
    CORE,
}

#[derive(Debug, Default)]
pub struct SiteOpts {
    pub class: Option<SiteClass>,
    pub symmetries: Option<Vec<Symmetry>>,
    pub rowpattern: Option<Vec<RowPattern>>,
    pub size: Option<Size>,
}

#[derive(Debug)]
pub struct Site {
    pub name: String,
    pub opts: SiteOpts,
}
impl Site {
    pub fn new(name: String, opts: SiteOpts, chk_name: String) -> Self {
        if name != chk_name {
            panic!("{} is different from {}", name, chk_name);
        }
        Self { name, opts }
    }
    pub fn is_core(&self) -> bool {
        matches!(self.opts.class, Some(SiteClass::CORE))
    }
}

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//  End Site                                                                  //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default)]
pub struct LEF {
    pub version: f32,
    pub namescasesensitive: Option<OnOff>,
    pub nowireextensionatpin: Option<OnOff>,
    pub busbitchars: Option<String>,
    pub dividerchar: Option<String>,
    pub manufacturinggrid: Option<f32>,
    pub fixedmask: Option<bool>,
    pub useminspacingobs: Option<OnOff>,
    pub useminspacingpin: Option<OnOff>,
    pub clearancemeasure: Option<ClearanceMeasureValue>,
    pub maxviastack: Option<MaxViaStack>,

    pub units: Option<Units>,
    pub propertydefinitions: Option<Vec<PropertyDefinition>>,
    pub spacings: Option<Vec<Spacing>>,
    pub layers: Option<Vec<Layer>>,
    pub vias: Option<Vec<Via>>,
    pub viarules: Option<Vec<ViaRule>>,
    pub nondefaultrules: Option<Vec<NonDefaultRule>>,
    pub sites: Option<Vec<Site>>,
    pub macros: Option<Vec<Macro>>,
}

impl LEF {
    pub fn get_site(&self) -> Option<&Site> {
        self.sites
            .as_ref()
            .and_then(|sites| sites.iter().find(|s| s.is_core()).or_else(|| sites.first()))
    }
}

impl fmt::Display for LEF {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "LEF Summary:")?;
        writeln!(f, "  VERSION: {}", self.version)?;

        if let Some(ref n) = self.namescasesensitive {
            writeln!(f, "  NAMECASESENSITIVE: {:?}", n)?;
        }
        if let Some(ref t) = self.nowireextensionatpin {
            writeln!(f, "  NOWIREEXTENSIONATPIN: {:?}", t)?;
        }
        if let Some(ref t) = self.busbitchars {
            writeln!(f, "  BUSBITCHARS: {}", t)?;
        }
        if let Some(ref c) = self.layers {
            writeln!(f, "  LAYERS: {}", c.len())?;
        }
        if let Some(ref p) = self.vias {
            writeln!(f, "  VIAS: {}", p.len())?;
        }
        if let Some(ref n) = self.viarules {
            writeln!(f, "  VIARULES: {}", n.len())?;
        }
        if let Some(ref sn) = self.sites {
            writeln!(f, "  SITES: {}", sn.len())?;
        }
        if let Some(ref sn) = self.macros {
            writeln!(f, "  MACROS: {}", sn.len())?;
        }

        Ok(())
    }
}

pub fn read_lef(input: &OsString) -> Result<LEF> {
    let uppercase = read_file(input)?;

    let time_stamp = Instant::now();
    let result = lef::lefParser::new().parse(&uppercase);

    let elapsed = time_stamp.elapsed();
    let elapsed = elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 / 1_000_000_000.0;

    match result {
        Ok(def_result) => {
            println!("Input `{}` ({}s): OK", input.display(), elapsed);
            Ok(def_result)
        }
        Err(err) => Err(eyre!(
            "Input `{}` ({}s): parse error {:?}",
            input.display(),
            elapsed,
            err
        )),
    }
}
