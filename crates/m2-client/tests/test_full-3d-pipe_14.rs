use m2_client::lens_flare_screen_pipeline::LensFlareScreenPipeline;
use m2_client::lens_sun_flare_v2::FlarePiece;

#[test]
fn test_generate_vertices_ignores_zero_size() {
    let pieces = [
        FlarePiece {
            position_x: 100.0,
            position_y: 100.0,
            size: 0.0,
            color_r: 1.0, color_g: 1.0, color_b: 1.0, color_a: 1.0,
        }
    ];

    let vertices = LensFlareScreenPipeline::generate_vertices(pieces.as_slice(), 800.0, 600.0);
    assert_eq!(vertices.len(), 0);
}

#[test]
fn test_generate_vertices_ignores_zero_alpha() {
    let pieces = [
        FlarePiece {
            position_x: 100.0,
            position_y: 100.0,
            size: 50.0,
            color_r: 1.0, color_g: 1.0, color_b: 1.0, color_a: 0.0,
        }
    ];

    let vertices = LensFlareScreenPipeline::generate_vertices(pieces.as_slice(), 800.0, 600.0);
    assert_eq!(vertices.len(), 0);
}

#[test]
fn test_generate_vertices_screen_to_ndc_mapping() {
    let pieces = [
        FlarePiece {
            position_x: 400.0, // Center X
            position_y: 300.0, // Center Y
            size: 100.0,
            color_r: 1.0, color_g: 1.0, color_b: 1.0, color_a: 1.0,
        }
    ];

    let vertices = LensFlareScreenPipeline::generate_vertices(pieces.as_slice(), 800.0, 600.0);
    assert_eq!(vertices.len(), 6);

    let tl = vertices[0];
    
    // x0 = 400 - 50 = 350
    // y0 = 300 - 50 = 250
    // ndc_x0 = (350/800)*2 - 1 = -0.125
    // ndc_y0 = 1 - (250/600)*2 = 0.1666666
    assert!((tl.position[0] - (-0.125)).abs() < 0.001);
    assert!((tl.position[1] - (1.0 - (250.0 / 600.0) * 2.0)).abs() < 0.001);
}

#[test]
fn test_generate_vertices_tex_coord_and_color() {
    let color = [0.1, 0.2, 0.3, 0.4];
    let pieces = [
        FlarePiece {
            position_x: 0.0,
            position_y: 0.0,
            size: 100.0,
            color_r: color[0], color_g: color[1], color_b: color[2], color_a: color[3],
        }
    ];

    let vertices = LensFlareScreenPipeline::generate_vertices(pieces.as_slice(), 800.0, 600.0);
    assert_eq!(vertices.len(), 6);

    // TL
    assert_eq!(vertices[0].tex_coord, [0.0, 0.0]);
    assert_eq!(vertices[0].color, color);

    // BL
    assert_eq!(vertices[1].tex_coord, [0.0, 1.0]);
    assert_eq!(vertices[1].color, color);

    // BR
    assert_eq!(vertices[2].tex_coord, [1.0, 1.0]);
    assert_eq!(vertices[2].color, color);
}

#[test]
fn test_generate_vertices_multiple_pieces() {
    let pieces = [
        FlarePiece {
            position_x: 100.0, position_y: 100.0, size: 50.0,
            color_r: 1.0, color_g: 1.0, color_b: 1.0, color_a: 1.0,
        },
        FlarePiece {
            position_x: 200.0, position_y: 200.0, size: 0.0, // Skipped
            color_r: 1.0, color_g: 1.0, color_b: 1.0, color_a: 1.0,
        },
        FlarePiece {
            position_x: 300.0, position_y: 300.0, size: 20.0,
            color_r: 1.0, color_g: 1.0, color_b: 1.0, color_a: 1.0,
        }
    ];

    let vertices = LensFlareScreenPipeline::generate_vertices(pieces.as_slice(), 800.0, 600.0);
    assert_eq!(vertices.len(), 12); // 2 valid pieces * 6 vertices
}
