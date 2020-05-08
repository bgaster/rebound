extern crate lyon;
use lyon::math::{point, Point, Vector, vector, Scale};
use lyon::path::{Builder};
use lyon::tessellation::*;

pub fn circle(builder: &mut Builder) {
    let p1 = point(1.0000,0.0000) ;
    let (p2, p3, p4) = (point(1.0000, 0.2761), point(0.9024, 0.5118), point(0.7071, 0.7071));
    let (p5, p6, p7) = (point(0.5118, 0.9024), point(0.2761, 1.0000), point(-0.0000, 1.0000));
    let (p8, p9, p10) = (point(-0.1786, 1.0000), point(-0.3453, 0.9553), point(-0.5000, 0.8660));
    let (p11, p12, p13) = (point(-0.6547, 0.7767), point(-0.7767, 0.6547), point(-0.8660, 0.5000));
    let (p14, p15, p16) = (point(-0.9553, 0.3453), point(-1.0000, 0.1786), point(-1.0000, -0.0000));
    let (p17, p18, p19) = (point(-1.0000, -0.2761), point(-0.9024, -0.5118), point(-0.7071, -0.7071));
    let (p20, p21, p22) = (point(-0.5118, -0.9024), point(-0.2761, -1.0000), point(-0.0000, -1.0000));
    let (p23, p24, p25) = (point(0.1786, -1.0000), point(0.3453, -0.9553), point(0.5000, -0.8660));
    let (p26, p27, p28) = (point(0.6547, -0.7767), point(0.7767, -0.6547), point(0.8660, -0.5000));
    let (p29, p30, p31) = (point(0.9553, -0.3453), point(1.0000, -0.1786), point(1.0000, 0.0000));

    builder.move_to(p1);
    builder.cubic_bezier_to(p2, p3, p4);
    builder.cubic_bezier_to(p5, p6, p7);
    builder.cubic_bezier_to(p8, p9, p10);
    builder.cubic_bezier_to(p11, p12, p13);
    builder.cubic_bezier_to(p14, p15, p16);
    builder.cubic_bezier_to(p17, p18, p19);
    builder.cubic_bezier_to(p20, p21, p22);
    builder.cubic_bezier_to(p23, p24, p25);
    builder.cubic_bezier_to(p26, p27, p28);
    builder.cubic_bezier_to(p29, p30, p31);
    builder.close();
}