use std::fs;
use pathfinder_export::{Export, FileFormat};
use pathfinder_renderer::scene::Scene;
// use vector::{Rect, Vector};
use crate::render::{Renderer, SceneWrapper};
use crate::layout::{Grid, Layout, engine, LayoutSettings, Style};
use crate::ast::ParseNode;
use crate::font::FontContext;
use font::OpenTypeFont;

use pathfinder_geometry::{
    transform2d::Transform2F,
    vector::Vector2F,
    rect::RectF,
};

fn v_xy(x: f64, y: f64) -> Vector2F {
    Vector2F::new(x as f32, y as f32)
}

pub fn dev() {
    let samples = include_str!("../source.tex")
        .lines()
        .map(|line| -> Vec<ParseNode> {
            // let parsed: Vec<ParseNode> = parse(dbg!(line)).unwrap();
            // println!("parsed: {:#?}", parsed);
            // parsed
            unimplemented!()
        })
        .collect::<Vec<_>>();
    let font1 = OpenTypeFont::parse(
        include_bytes!("../data/xits/rex-xits.otf")
    );
    let font2 = OpenTypeFont::parse(
        include_bytes!("../data/fonts/modern.otf")
    );
    let font3 = OpenTypeFont::parse(
        include_bytes!("../data/fonts/STIX2Math.otf")
    );

    let fonts = vec![font1, font2, font3];

    let mut grid = Grid::new();

    for (row, font) in fonts.iter().enumerate() {
        let ctx = FontContext::new(&font);
        let layout_settings = LayoutSettings::new(
            &ctx,
            10.0,
            Style::Display
        );

        for (column, sample) in samples.iter().enumerate() {
            let res = engine::layout(
                sample,
                layout_settings
            );
            if let Ok(node) = res.map(|l| l.as_node()) {
                grid.insert(row, column, node);
            }
        }
    }

    let mut layout = Layout::new();
    layout.add_node(grid.build());

    let mut renderer = Renderer::new();
    let (x0, y0, x1, y1) = renderer.size(&layout);
    let mut scene = Scene::new();
    scene.set_view_box(RectF::from_points(
        v_xy(x0, y0),
        v_xy(x1, y1)
    ));
    let mut backend = SceneWrapper::new(&mut scene);
    renderer.render(&layout, &mut backend);
    
    let mut buf = Vec::new();
    scene.export(&mut buf, FileFormat::PDF).unwrap();

    fs::write("qc.pdf", &buf).unwrap();
}