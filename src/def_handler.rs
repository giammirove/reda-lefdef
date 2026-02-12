/*
*
* DEF (Design Exchange Format) Handler - Complete Implementation
* Based on: http://coriolis.lip6.fr/doc/lefdef/lefdefref/DEFSyntax.html
*
*/
#![allow(unused)]
#![allow(clippy::upper_case_acronyms)]
use crate::{common_handler::*, def};
use eyre::{eyre, Result};
use std::time::Instant;
use std::{ffi::OsString, fmt};

#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub ext: Option<i32>,
}
impl Point {
    pub fn new(x: i32, y: i32, ext: Option<i32>) -> Self {
        Self { x, y, ext }
    }
}

#[derive(Debug)]
pub struct Rect {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}
impl Rect {
    pub fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Self { x1, y1, x2, y2 }
    }
}

#[derive(Debug)]
pub struct Polygon {
    pub points: Vec<Point>,
}
impl Polygon {
    pub fn new(points: Vec<Point>) -> Self {
        Self { points }
    }
}

#[derive(Debug)]
pub struct DoStepBy {
    pub do_value: i32,
    pub by: Option<i32>,
    pub step_x: i32,
    pub step_y: i32,
}
impl DoStepBy {
    pub fn new(do_value: i32, by: Option<i32>, step_x: i32, step_y: i32) -> Self {
        Self {
            do_value,
            by,
            step_x,
            step_y,
        }
    }
}

#[derive(Debug)]
pub enum Value {
    NUMBER(f32),
    STRING(String),
    INTEGER(i32),
}

#[derive(Debug)]
pub struct Property {
    pub name: String,
    pub value: Value,
}
impl Property {
    pub fn new(name: String, value: Value) -> Self {
        Self { name, value }
    }
}

// HISTORY statement
#[derive(Debug)]
pub struct History {
    pub entries: Vec<String>,
}

// PROPERTYDEFINITIONS section
#[derive(Debug)]
pub enum PropertyDefinitionObjectType {
    COMPONENT,
    NET,
    SPECIALNET,
    GROUP,
    ROW,
    REGION,
    COMPONENTPIN,
    DESIGN,
    NONDEFAULTRULE,
}

#[derive(Debug)]
pub enum PropertyDefinitionType {
    INTEGER,
    REAL,
    STRING,
}

#[derive(Debug)]
pub struct PropertyDefinition {
    pub object_type: PropertyDefinitionObjectType,
    pub name: String,
    pub prop_type: PropertyDefinitionType,
    pub range_or_value: Option<RangeOrValueEnum>,
}

// DIEAREA statement
#[derive(Debug)]
pub struct DieArea {
    pub points: Vec<Point>,
}
impl DieArea {
    pub fn new(points: Vec<Point>) -> Self {
        Self { points }
    }
    pub fn get_rectangle(&self) -> Result<(f32, f32, f32, f32)> {
        if self.points.len() != 2 {
            return Err(eyre!(
                "Cant get rectangle of DIEAREA with {}",
                self.points.len()
            ));
        }

        let first_point = self
            .points
            .first()
            .expect("DIEAREA should have at least 1 points");
        let second_point = self
            .points
            .get(1)
            .expect("DIEAREA should have at least 2 points");
        let offset_x = first_point.x as f32;
        let offset_y = first_point.y as f32;
        let width = second_point.x as f32;
        let height = second_point.y as f32;
        Ok((offset_x, offset_y, width - offset_x, height - offset_y))
    }
}
impl fmt::Display for DieArea {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.points)
    }
}

// ROW statement
#[derive(Debug)]
pub struct Row {
    pub name: String,
    pub site: String,
    pub x: i32,
    pub y: i32,
    pub orient: Orient,
    pub do_step_by: Option<DoStepBy>,
    pub properties: Option<Vec<Property>>,
}
impl Row {
    pub fn new(
        name: String,
        site: String,
        x: i32,
        y: i32,
        orient: Orient,
        do_step_by: Option<DoStepBy>,
        properties: Option<Vec<Property>>,
    ) -> Self {
        Self {
            name,
            site,
            x,
            y,
            orient,
            do_step_by,
            properties,
        }
    }
}

// TRACKS statement
#[derive(Debug)]
pub enum TrackDir {
    X,
    Y,
}

#[derive(Debug)]
pub struct Track {
    pub dir: TrackDir,
    pub start: i32,
    pub num: i32,
    pub step: i32,
    pub layers: Vec<String>,
    pub mask: Option<i32>,
    pub samemask: Option<bool>,
}
impl Track {
    pub fn new(
        dir: TrackDir,
        start: i32,
        num: i32,
        step: i32,
        layers: Vec<String>,
        mask: Option<i32>,
        samemask: Option<bool>,
    ) -> Self {
        Self {
            dir,
            start,
            num,
            step,
            layers,
            mask,
            samemask,
        }
    }
}

// GCELLGRID statement
#[derive(Debug)]
pub struct GCellGrid {
    pub dir: TrackDir,
    pub start: i32,
    pub num: i32,
    pub step: i32,
}
impl GCellGrid {
    pub fn new(dir: TrackDir, start: i32, num: i32, step: i32) -> Self {
        Self {
            dir,
            start,
            num,
            step,
        }
    }
}

// VIA statement
#[derive(Debug)]
pub struct ViaRect {
    pub layer_name: String,
    pub rect: Rect,
    pub mask: Option<i32>,
}
impl ViaRect {
    pub fn new(layer_name: String, rect: Rect, mask: Option<i32>) -> Self {
        Self {
            layer_name,
            rect,
            mask,
        }
    }
}

#[derive(Debug)]
pub struct ViaPolygon {
    pub layer_name: String,
    pub mask: Option<i32>,
    pub polygon: Polygon,
}
impl ViaPolygon {
    pub fn new(layer_name: String, polygon: Polygon, mask: Option<i32>) -> Self {
        Self {
            layer_name,
            polygon,
            mask,
        }
    }
}

#[derive(Debug)]
pub enum ViaLayer {
    Rect(ViaRect),
    Polygon(ViaPolygon),
}

#[derive(Debug)]
pub struct ViaViaRule {
    pub name: String,
    pub cutsize: (i32, i32),
    pub layers: (String, String, String),
    pub cutspacing: (i32, i32),
    pub enclosure: (i32, i32, i32, i32),
    pub rowcol: Option<(i32, i32)>,
    pub origin: Option<(i32, i32)>,
    pub offset: Option<(i32, i32, i32, i32)>,
    pub pattern: Option<String>,
}

#[derive(Debug)]
pub enum ViaType {
    ViaRule(ViaViaRule),
    Geometry(Vec<ViaLayer>),
}

#[derive(Debug)]
pub struct Via {
    pub name: String,
    pub via_type: ViaType,
}

// STYLES statement
#[derive(Debug)]
pub struct Style {
    pub style_num: i32,
    pub points: Vec<(i32, i32)>,
}

// NONDEFAULTRULES statement
#[derive(Debug)]
pub struct NonDefaultRuleLayer {
    pub name: String,
    pub width: i32,
    pub diagwidth: Option<i32>,
    pub spacing: Option<i32>,
    pub wireext: Option<i32>,
}

#[derive(Debug)]
pub struct NonDefaultRule {
    pub name: String,
    pub hardspacing: Option<bool>,
    pub layers: Vec<NonDefaultRuleLayer>,
    pub vias: Option<Vec<String>>,
    pub viarules: Option<Vec<String>>,
    pub mincuts: Option<Vec<(String, i32)>>,
    pub properties: Option<Vec<Property>>,
}

// REGIONS statement
#[derive(Debug)]
pub enum RegionType {
    FENCE,
    GUIDE,
}

#[derive(Debug)]
pub struct Region {
    pub name: String,
    pub region_type: Option<RegionType>,
    pub rects: Vec<Rect>,
    pub properties: Option<Vec<Property>>,
}

// COMPONENTMASKSHIFT statement
#[derive(Debug)]
pub struct ComponentMaskShift {
    pub shifts: Vec<i32>,
}

// COMPONENTS section
#[derive(Debug)]
pub enum ComponentSource {
    NETLIST,
    DIST,
    USER,
    TIMING,
}

#[derive(Debug, PartialEq)]
pub enum ComponentPlacement {
    PLACED,
    FIXED,
    COVER,
    UNPLACED,
}

#[derive(Debug, Default)]
pub struct ComponentOpts {
    pub source: Option<ComponentSource>,
    pub weight: Option<i32>,
    pub eeq: Option<String>,
    pub region: Option<String>,
    pub placement: Option<(ComponentPlacement, Point, Orient)>,
    pub halo: Option<(i32, i32, i32, i32)>,
    pub routehalo: Option<(i32, String, String)>,
    pub maskshift: Option<i32>,
    pub properties: Option<Vec<Property>>,
}

#[derive(Debug)]
pub struct Component {
    pub name: String,
    pub model: String,
    pub opts: ComponentOpts,
}
impl Component {
    pub fn new(name: String, model: String, opts: ComponentOpts) -> Self {
        Self { name, model, opts }
    }

    pub fn is_fixed(&self) -> bool {
        if let Some(pl) = &self.opts.placement {
            return pl.0 == ComponentPlacement::FIXED;
        }
        false
    }

    pub fn is_unplaced(&self) -> bool {
        if let Some(pl) = &self.opts.placement {
            return pl.0 == ComponentPlacement::UNPLACED;
        }
        false
    }
}

// PINS section
#[derive(Debug)]
pub enum PinDirection {
    INPUT,
    OUTPUT,
    INOUT,
    FEEDTHRU,
}

#[derive(Debug)]
pub enum PinUse {
    SIGNAL,
    POWER,
    GROUND,
    CLOCK,
    TIEOFF,
    ANALOG,
    SCAN,
    RESET,
}

#[derive(Debug)]
pub enum PinPlacement {
    PLACED,
    FIXED,
    COVER,
}

#[derive(Debug)]
pub enum PinNetExpr {
    MustJoin(String),
}

#[derive(Debug)]
pub struct PinPort {
    pub layer: Option<String>,
    pub rects: Option<Vec<Rect>>,
    pub polygons: Option<Vec<Polygon>>,
    pub vias: Option<Vec<(Point, String)>>,
    pub mask: Option<i32>,
}

#[derive(Debug)]
pub struct PinOptPlacement {
    pub placement: PinPlacement,
    pub location: Point,
    pub orient: Orient,
}
impl PinOptPlacement {
    pub fn new(placement: PinPlacement, location: Point, orient: Orient) -> Self {
        Self {
            placement,
            location,
            orient,
        }
    }
}

#[derive(Debug)]
pub enum PinOptsEnum {
    Direction(PinDirection),
    Use(PinUse),
    NetExpr(String),
    SupplySensitivity(String),
    GroundSensitivity(String),
    Placement(PinOptPlacement),
    Ports(Vec<PinPort>),
    Properties(Vec<Property>),
}

#[derive(Debug, Default)]
pub struct PinOpts {
    pub direction: Option<PinDirection>,
    pub use_type: Option<PinUse>,
    pub net_expr: Option<String>,
    pub supply_sensitivity: Option<String>,
    pub ground_sensitivity: Option<String>,
    pub antennamodel: Option<String>,
    pub antennapinpartialmetalarea: Option<Vec<(f32, Option<String>)>>,
    pub antennapinpartialmetalsidearea: Option<Vec<(f32, Option<String>)>>,
    pub antennapinpartialcutarea: Option<Vec<(f32, Option<String>)>>,
    pub antennapingatearea: Option<Vec<(f32, Option<String>)>>,
    pub antennapindiffarea: Option<Vec<(f32, Option<String>)>>,
    pub antennapinmaxareacar: Option<Vec<(f32, String)>>,
    pub antennapinmaxsideareacar: Option<Vec<(f32, String)>>,
    pub antennapinmaxcutcar: Option<Vec<(f32, String)>>,
    pub placement: Option<PinOptPlacement>,
    pub ports: Option<Vec<PinPort>>,
    pub properties: Option<Vec<Property>>,
}

impl PinOpts {
    pub fn apply(&mut self, opt: PinOptsEnum) {
        match opt {
            PinOptsEnum::Direction(d) => self.direction = Some(d),
            PinOptsEnum::Use(u) => self.use_type = Some(u),
            PinOptsEnum::NetExpr(n) => self.net_expr = Some(n),
            PinOptsEnum::SupplySensitivity(s) => self.supply_sensitivity = Some(s),
            PinOptsEnum::GroundSensitivity(g) => self.ground_sensitivity = Some(g),
            PinOptsEnum::Placement(pl) => self.placement = Some(pl),
            PinOptsEnum::Ports(ppl) => self.ports = Some(ppl),
            PinOptsEnum::Properties(pl) => self.properties = Some(pl),
        }
    }
}

#[derive(Debug)]
pub struct Pin {
    pub name: String,
    pub net: String,
    pub special: bool,
    pub opts: PinOpts,
}
impl Pin {
    pub fn new(name: String, net: String, special: bool, opts: PinOpts) -> Self {
        Self {
            name,
            net,
            special,
            opts,
        }
    }
}

// PINPROPERTIES section
#[derive(Debug)]
pub struct PinProperty {
    pub comp_name: String,
    pub pin_name: String,
    pub properties: Vec<Property>,
}

// BLOCKAGES section
#[derive(Debug)]
pub enum BlockagePlacementType {
    SOFT,
    PARTIAL(f32),
}

#[derive(Debug)]
pub struct BlockagePlacement {
    pub component: Option<String>,
    pub pushdown: Option<bool>,
    pub blockage_type: Option<BlockagePlacementType>,
    pub rects: Vec<Rect>,
}

#[derive(Debug)]
pub struct BlockageRouting {
    pub layer: String,
    pub component: Option<String>,
    pub slots: Option<bool>,
    pub fills: Option<bool>,
    pub pushdown: Option<bool>,
    pub exceptpgnet: Option<bool>,
    pub spacing: Option<i32>,
    pub designrulewidth: Option<i32>,
    pub mask: Option<i32>,
    pub rects: Vec<Rect>,
    pub polygons: Option<Vec<Polygon>>,
}

#[derive(Debug)]
pub enum Blockage {
    Placement(BlockagePlacement),
    Routing(BlockageRouting),
}

// SLOTS section
#[derive(Debug)]
pub struct Slot {
    pub layer: String,
    pub rects: Vec<Rect>,
    pub polygons: Option<Vec<Polygon>>,
}

// FILLS section
#[derive(Debug)]
pub struct FillRect {
    pub layer: String,
    pub mask: Option<i32>,
    pub opc: Option<bool>,
    pub rects: Vec<Rect>,
}

#[derive(Debug)]
pub struct FillPolygon {
    pub layer: String,
    pub mask: Option<i32>,
    pub opc: Option<bool>,
    pub polygons: Vec<Polygon>,
}

#[derive(Debug)]
pub struct FillVia {
    pub mask: Option<i32>,
    pub opc: Option<bool>,
    pub via: String,
    pub points: Vec<Point>,
}

#[derive(Debug)]
pub enum Fill {
    Rect(FillRect),
    Polygon(FillPolygon),
    Via(FillVia),
}

// SPECIALNETS section

// Routing point elements
#[derive(Debug)]
pub struct RoutingVia {
    pub via_name: String,
    pub orient: Option<Orient>,
    pub mask: Option<i32>, // viaMaskNum (hex-encoded)
    pub do_step_by: Option<DoStepBy>,
}

#[derive(Debug)]
pub enum RoutingPointItem {
    Point(Point, Option<i32>), // point with optional mask
    Via(RoutingVia),
}

// Special Wiring Enums

#[derive(Debug)]
pub enum SpecialWireType {
    COVER,
    FIXED,
    ROUTED,
    SHIELD(String),
    NOSHIELD,
}

// Special Wiring structures
#[derive(Debug)]
pub enum SpecialWireShape {
    RING,
    PADRING,
    BLOCKRING,
    STRIPE,
    FOLLOWPIN,
    IOWIRE,
    COREWIRE,
    BLOCKWIRE,
    BLOCKAGEWIRE,
    FILLWIRE,
    FILLWIREOPC,
    DRCFILL,
}

#[derive(Debug)]
pub enum SpecialNetSource {
    DIST,
    NETLIST,
    TIMING,
    USER,
}

#[derive(Debug)]
pub enum SpecialNetUse {
    ANALOG,
    CLOCK,
    GROUND,
    POWER,
    RESET,
    SCAN,
    SIGNAL,
    TIEOFF,
}

#[derive(Debug)]
pub enum SpecialNetPattern {
    BALANCED,
    STEINER,
    TRUNK,
    WIREDLOGIC,
}

// Geometry-based wiring (POLYGON, RECT, VIA)
#[derive(Debug)]
pub struct SpecialWirePolygonGeom {
    pub layer: String,
    pub points: Vec<Point>,
    pub shape: Option<SpecialWireShape>,
    pub mask: Option<i32>,
}

#[derive(Debug)]
pub struct SpecialWireRectGeom {
    pub layer: String,
    pub pt1: Point,
    pub pt2: Point,
    pub shape: Option<SpecialWireShape>,
    pub mask: Option<i32>,
}

#[derive(Debug)]
pub struct SpecialWireViaGeom {
    pub via: String,
    pub orient: Option<Orient>,
    pub points: Vec<Point>,
    pub shape: Option<SpecialWireShape>,
    pub mask: Option<i32>,
}

#[derive(Debug)]
pub struct SpecialWirePathSegment {
    pub layer: String,
    pub width: i32,
    pub shape: Option<SpecialWireShape>,
    pub style: Option<i32>,
    pub points: Vec<RoutingPointItem>,
}

#[derive(Debug)]
pub enum SpecialWireSegment {
    Polygon(SpecialWirePolygonGeom),
    Rect(SpecialWireRectGeom),
    Via(SpecialWireViaGeom),
    Path(SpecialWirePathSegment),
}

#[derive(Debug)]
pub struct SpecialWiring {
    pub wire_type: SpecialWireType,
    pub segments: Vec<SpecialWireSegment>,
}

#[derive(Debug, Default)]
pub struct SpecialNetOpts {
    pub connections: Option<Vec<(String, String)>>,
    pub voltage: Option<f32>,
    pub wires: Option<Vec<SpecialWiring>>,
    pub source: Option<SpecialNetSource>,
    pub fixedbump: Option<bool>,
    pub original: Option<String>,
    pub use_type: Option<SpecialNetUse>,
    pub pattern: Option<SpecialNetPattern>,
    pub estcap: Option<f32>,
    pub weight: Option<i32>,
    pub properties: Option<Vec<Property>>,
}

#[derive(Debug)]
pub struct SpecialNet {
    pub name: String,
    pub opts: SpecialNetOpts,
}

// NETS section
#[derive(Debug)]
pub enum WireType {
    COVER,
    FIXED,
    ROUTED,
    NOSHIELD,
}

#[derive(Debug)]
pub struct WireSegment {
    pub layer: Option<String>,
    pub width: Option<i32>,
    pub shape: Option<String>,
    pub style: Option<i32>,
    pub mask: Option<i32>,
    pub points: Vec<Point>,
    pub via: Option<String>,
    pub viastyle: Option<i32>,
}

#[derive(Debug)]
pub struct Wire {
    pub wire_type: WireType,
    pub taper: Option<bool>,
    pub taperrule: Option<String>,
    pub segments: Vec<WireSegment>,
}

#[derive(Debug)]
pub enum NetSource {
    NETLIST,
    DIST,
    USER,
    TIMING,
    TEST,
}

#[derive(Debug)]
pub enum NetUse {
    ANALOG,
    CLOCK,
    GROUND,
    POWER,
    RESET,
    SCAN,
    SIGNAL,
    TIEOFF,
}

#[derive(Debug, Default)]
pub struct NetOpts {
    pub connections: Option<Vec<(String, String)>>,
    pub wires: Option<Vec<Wire>>,
    pub source: Option<NetSource>,
    pub fixedbump: Option<bool>,
    pub frequency: Option<f32>,
    pub original: Option<String>,
    pub use_type: Option<NetUse>,
    pub pattern: Option<String>,
    pub estcap: Option<f32>,
    pub weight: Option<i32>,
    pub xtalk: Option<i32>,
    pub nondefaultrule: Option<String>,
    pub subnet: Option<Vec<(String, Vec<(String, String)>, bool)>>,
    pub properties: Option<Vec<Property>>,
}

#[derive(Debug)]
pub struct Net {
    pub name: String,
    pub opts: NetOpts,
}

// SCANCHAINS section
#[derive(Debug)]
pub enum ScanChainDirection {
    IN,
    OUT,
}

#[derive(Debug)]
pub struct ScanChainPin {
    pub comp: String,
    pub pin: String,
}

#[derive(Debug)]
pub struct ScanChain {
    pub name: String,
    pub start: Option<ScanChainPin>,
    pub stop: Option<ScanChainPin>,
    pub floating: Option<Vec<ScanChainPin>>,
    pub ordered: Option<Vec<ScanChainPin>>,
    pub commonscanpins: Option<Vec<(ScanChainDirection, String)>>,
    pub partition: Option<(String, Option<i32>)>,
}

// GROUPS section
#[derive(Debug)]
pub struct Group {
    pub name: String,
    pub components: Vec<String>,
    pub region: Option<String>,
    pub properties: Option<Vec<Property>>,
}

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

// SITES section
#[derive(Debug)]
pub struct Site {
    pub name: String,
    pub size: Size,
    pub orient: Option<Orient>,
    pub do_step_by: Option<DoStepBy>,
}
impl Site {
    pub fn new(
        name: String,
        size: Size,
        orient: Option<Orient>,
        do_step_by: Option<DoStepBy>,
    ) -> Self {
        Self {
            name,
            size,
            orient,
            do_step_by,
        }
    }
}

// BEGINEXT section
#[derive(Debug)]
pub struct Extension {
    pub tag: String,
    pub content: Vec<String>,
}

// Main DEF structure
#[derive(Debug, Default)]
pub struct DEF {
    pub version: Option<f32>,
    pub dividerchar: Option<String>,
    pub busbitchars: Option<String>,
    pub design_name: Option<String>,
    pub technology: Option<String>,
    pub units: Option<u32>,
    pub history: Option<Vec<String>>,
    pub propertydefinitions: Option<Vec<PropertyDefinition>>,
    pub diearea: Option<DieArea>,
    pub rows: Option<Vec<Row>>,
    pub tracks: Option<Vec<Track>>,
    pub gcellgrids: Option<Vec<GCellGrid>>,
    pub vias: Option<Vec<Via>>,
    pub styles: Option<Vec<Style>>,
    pub nondefaultrules: Option<Vec<NonDefaultRule>>,
    pub regions: Option<Vec<Region>>,
    pub componentmaskshift: Option<ComponentMaskShift>,
    pub components: Option<Vec<Component>>,
    pub pins: Option<Vec<Pin>>,
    pub pinproperties: Option<Vec<PinProperty>>,
    pub blockages: Option<Vec<Blockage>>,
    pub slots: Option<Vec<Slot>>,
    pub fills: Option<Vec<Fill>>,
    pub specialnets: Option<Vec<SpecialNet>>,
    pub nets: Option<Vec<Net>>,
    pub scanchains: Option<Vec<ScanChain>>,
    pub groups: Option<Vec<Group>>,
    pub sites: Option<Vec<Site>>,
    pub extensions: Option<Vec<Extension>>,
}

impl DEF {
    pub fn new() -> Self {
        Self::default()
    }
}

impl fmt::Display for DEF {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "DEF Summary:")?;

        if let Some(ref v) = self.version {
            writeln!(f, "  VERSION: {}", v)?;
        }
        if let Some(ref n) = self.design_name {
            writeln!(f, "  DESIGN: {}", n)?;
        }
        if let Some(ref t) = self.technology {
            writeln!(f, "  TECHNOLOGY: {}", t)?;
        }
        if let Some(ref d) = self.diearea {
            writeln!(f, "  DIEAREA: {}", d)?;
        }
        if let Some(ref u) = self.units {
            writeln!(f, "  UNITS: {}", u)?;
        }
        if let Some(ref c) = self.components {
            writeln!(f, "  COMPONENTS: {}", c.len())?;
        }
        if let Some(ref p) = self.pins {
            writeln!(f, "  PINS: {}", p.len())?;
        }
        if let Some(ref n) = self.nets {
            writeln!(f, "  NETS: {}", n.len())?;
        }
        if let Some(ref sn) = self.specialnets {
            writeln!(f, "  SPECIALNETS: {}", sn.len())?;
        }

        Ok(())
    }
}

pub fn read_def(input: &OsString) -> Result<DEF> {
    let uppercase = read_file(input)?;

    let time_stamp = Instant::now();
    let result = def::defParser::new().parse(&uppercase);

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
