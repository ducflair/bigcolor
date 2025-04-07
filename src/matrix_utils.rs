// Matrix utilities for color space conversions

pub type Matrix3x3 = [[f32; 3]; 3];
pub type Vector3 = [f32; 3];

/// Multiply a 3D vector by a 3x3 matrix
pub fn multiply_v3_m3x3(v: Vector3, m: Matrix3x3) -> Vector3 {
    [
        v[0] * m[0][0] + v[1] * m[0][1] + v[2] * m[0][2],
        v[0] * m[1][0] + v[1] * m[1][1] + v[2] * m[1][2],
        v[0] * m[2][0] + v[1] * m[2][1] + v[2] * m[2][2],
    ]
}

// Recalculated for consistent reference white
// see https://github.com/w3c/csswg-drafts/issues/6642#issuecomment-943521484
pub const XYZ_TO_LMS_M: Matrix3x3 = [
    [0.8190224379967030, 0.3619062600528904, -0.1288737815209879],
    [0.0329836539323885, 0.9292868615863434, 0.0361446663506424],
    [0.0481771893596242, 0.2642395317527308, 0.6335478284694309],
];

// inverse of XYZ_TO_LMS_M
pub const LMS_TO_XYZ_M: Matrix3x3 = [
    [1.2268798758459243, -0.5578149944602171, 0.2813910456659647],
    [-0.0405757452148008, 1.1122868032803170, -0.0717110580655164],
    [-0.0763729366746601, -0.4214933324022432, 1.5869240198367816],
];

pub const LMS_TO_LAB_M: Matrix3x3 = [
    [0.2104542683093140, 0.7936177747023054, -0.0040720430116193],
    [1.9779985324311684, -2.4285922420485799, 0.4505937096174110],
    [0.0259040424655478, 0.7827717124575296, -0.8086757549230774],
];

pub const LAB_TO_LMS_M: Matrix3x3 = [
    [1.0000000000000000, 0.3963377773761749, 0.2158037573099136],
    [1.0000000000000000, -0.1055613458156586, -0.0638541728258133],
    [1.0000000000000000, -0.0894841775298119, -1.2914855480194092],
];

// Bradford CAT matrices for D65 to D50 and vice versa
pub const D65_TO_D50_M: Matrix3x3 = [
    [1.0479297925449969, 0.022946870601609652, -0.05019226628920524],
    [0.02962780877005599, 0.9904344267538799, -0.017073799063418826],
    [-0.009243040646204504, 0.015055191490298152, 0.7518742814281371],
];

pub const D50_TO_D65_M: Matrix3x3 = [
    [0.955473421488075, -0.02309845494876471, 0.06325924320057072],
    [-0.0283697093338637, 1.0099953980813041, 0.021041441191917323],
    [0.012314014864481998, -0.020507649298898964, 1.330365926242124],
];

// White points (standard illuminants)
pub const WHITE_D65: Vector3 = [0.95047, 1.0, 1.08883]; // Standard D65 white point
pub const WHITE_D50: Vector3 = [0.96422, 1.0, 0.82521]; // Standard D50 white point

/// Adapt XYZ from one white point to another using Bradford transformation
pub fn adapt_xyz(xyz: Vector3, from_white: Vector3, to_white: Vector3) -> Vector3 {
    if from_white == to_white {
        return xyz;
    }
    
    if from_white == WHITE_D65 && to_white == WHITE_D50 {
        multiply_v3_m3x3(xyz, D65_TO_D50_M)
    } else if from_white == WHITE_D50 && to_white == WHITE_D65 {
        multiply_v3_m3x3(xyz, D50_TO_D65_M)
    } else {
        // For other white points, we would need a more general implementation
        // This is just a simplified version supporting D65<->D50
        xyz
    }
}

/// Constrain an angle to [0, 360) degrees
pub fn constrain_angle(angle: f32) -> f32 {
    let mut a = angle % 360.0;
    if a < 0.0 {
        a += 360.0;
    }
    a
} 