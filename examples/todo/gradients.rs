// pub(super) fn gradient_extend(scene: &mut Scene, params: &mut SceneParams<'_>) {
//     enum Kind {
//         Linear,
//         Radial,
//         Sweep,
//     }
//     pub(super) fn square(scene: &mut Scene, kind: Kind, transform: Affine, extend: Extend) {
//         let colors = [palette::css::RED, palette::css::LIME, palette::css::BLUE];
//         let width = 300_f64;
//         let height = 300_f64;
//         let gradient: Brush = match kind {
//             Kind::Linear => {
//                 Gradient::new_linear((width * 0.35, height * 0.5), (width * 0.65, height * 0.5))
//                     .with_stops(colors)
//                     .with_extend(extend)
//                     .into()
//             }
//             Kind::Radial => {
//                 let center = (width * 0.5, height * 0.5);
//                 let radius = (width * 0.25) as f32;
//                 Gradient::new_two_point_radial(center, radius * 0.25, center, radius)
//                     .with_stops(colors)
//                     .with_extend(extend)
//                     .into()
//             }
//             Kind::Sweep => Gradient::new_sweep(
//                 (width * 0.5, height * 0.5),
//                 30_f32.to_radians(),
//                 150_f32.to_radians(),
//             )
//             .with_stops(colors)
//             .with_extend(extend)
//             .into(),
//         };
//         scene.fill(
//             Fill::NonZero,
//             transform,
//             &gradient,
//             None,
//             &Rect::new(0.0, 0.0, width, height),
//         );
//     }
//     let extend_modes = [Extend::Pad, Extend::Repeat, Extend::Reflect];
//     for (x, extend) in extend_modes.iter().enumerate() {
//         for (y, kind) in [Kind::Linear, Kind::Radial, Kind::Sweep]
//             .into_iter()
//             .enumerate()
//         {
//             let transform =
//                 Affine::translate((x as f64 * 350.0 + 50.0, y as f64 * 350.0 + 100.0));
//             square(scene, kind, transform, *extend);
//         }
//     }
//     for (i, label) in ["Pad", "Repeat", "Reflect"].iter().enumerate() {
//         let x = i as f64 * 350.0 + 50.0;
//         params.text.add(
//             scene,
//             None,
//             32.0,
//             Some(&palette::css::WHITE.into()),
//             Affine::translate((x, 70.0)),
//             label,
//         );
//     }
//     params.resolution = Some((1200.0, 1200.0).into());
// }

// pub(super) fn two_point_radial(scene: &mut Scene, _params: &mut SceneParams<'_>) {
//     pub(super) fn make(
//         scene: &mut Scene,
//         x0: f64,
//         y0: f64,
//         r0: f32,
//         x1: f64,
//         y1: f64,
//         r1: f32,
//         transform: Affine,
//         extend: Extend,
//     ) {
//         let colors = [
//             palette::css::RED,
//             palette::css::YELLOW,
//             Color::from_rgb8(6, 85, 186),
//         ];
//         let width = 400_f64;
//         let height = 200_f64;
//         let rect = Rect::new(0.0, 0.0, width, height);
//         scene.fill(Fill::NonZero, transform, palette::css::WHITE, None, &rect);
//         scene.fill(
//             Fill::NonZero,
//             transform,
//             &Gradient::new_two_point_radial((x0, y0), r0, (x1, y1), r1)
//                 .with_stops(colors)
//                 .with_extend(extend),
//             None,
//             &Rect::new(0.0, 0.0, width, height),
//         );
//         let r0 = r0 as f64 - 1.0;
//         let r1 = r1 as f64 - 1.0;
//         let stroke_width = 1.0;
//         scene.stroke(
//             &Stroke::new(stroke_width),
//             transform,
//             palette::css::BLACK,
//             None,
//             &Ellipse::new((x0, y0), (r0, r0), 0.0),
//         );
//         scene.stroke(
//             &Stroke::new(stroke_width),
//             transform,
//             palette::css::BLACK,
//             None,
//             &Ellipse::new((x1, y1), (r1, r1), 0.0),
//         );
//     }

//     // These demonstrate radial gradient patterns similar to the examples shown
//     // at <https://learn.microsoft.com/en-us/typography/opentype/spec/colr#radial-gradients>

//     for (i, mode) in [Extend::Pad, Extend::Repeat, Extend::Reflect]
//         .iter()
//         .enumerate()
//     {
//         let y = 100.0;
//         let x0 = 140.0;
//         let x1 = x0 + 140.0;
//         let r0 = 20.0;
//         let r1 = 50.0;
//         make(
//             scene,
//             x0,
//             y,
//             r0,
//             x1,
//             y,
//             r1,
//             Affine::translate((i as f64 * 420.0 + 20.0, 20.0)),
//             *mode,
//         );
//     }

//     for (i, mode) in [Extend::Pad, Extend::Repeat, Extend::Reflect]
//         .iter()
//         .enumerate()
//     {
//         let y = 100.0;
//         let x0 = 140.0;
//         let x1 = x0 + 140.0;
//         let r0 = 20.0;
//         let r1 = 50.0;
//         make(
//             scene,
//             x1,
//             y,
//             r1,
//             x0,
//             y,
//             r0,
//             Affine::translate((i as f64 * 420.0 + 20.0, 240.0)),
//             *mode,
//         );
//     }

//     for (i, mode) in [Extend::Pad, Extend::Repeat, Extend::Reflect]
//         .iter()
//         .enumerate()
//     {
//         let y = 100.0;
//         let x0 = 140.0;
//         let x1 = x0 + 140.0;
//         let r0 = 50.0;
//         let r1 = 50.0;
//         make(
//             scene,
//             x0,
//             y,
//             r0,
//             x1,
//             y,
//             r1,
//             Affine::translate((i as f64 * 420.0 + 20.0, 460.0)),
//             *mode,
//         );
//     }

//     for (i, mode) in [Extend::Pad, Extend::Repeat, Extend::Reflect]
//         .iter()
//         .enumerate()
//     {
//         let x0 = 140.0;
//         let y0 = 125.0;
//         let r0 = 20.0;
//         let x1 = 190.0;
//         let y1 = 100.0;
//         let r1 = 95.0;
//         make(
//             scene,
//             x0,
//             y0,
//             r0,
//             x1,
//             y1,
//             r1,
//             Affine::translate((i as f64 * 420.0 + 20.0, 680.0)),
//             *mode,
//         );
//     }

//     for (i, mode) in [Extend::Pad, Extend::Repeat, Extend::Reflect]
//         .iter()
//         .enumerate()
//     {
//         let x0 = 140.0;
//         let y0 = 125.0;
//         let r0 = 20.0;
//         let x1 = 190.0;
//         let y1 = 100.0;
//         let r1 = 96.0;
//         // Shift p0 so the outer edges of both circles touch
//         let p0 = Point::new(x1, y1)
//             + ((Point::new(x0, y0) - Point::new(x1, y1)).normalize() * (r1 - r0));
//         make(
//             scene,
//             p0.x,
//             p0.y,
//             r0 as f32,
//             x1,
//             y1,
//             r1 as f32,
//             Affine::translate((i as f64 * 420.0 + 20.0, 900.0)),
//             *mode,
//         );
//     }
// }
