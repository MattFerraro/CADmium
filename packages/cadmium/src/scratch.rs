fn topological_naming() {
    // Let's replicate the flow seen here: https://wiki.freecad.org/Topological_naming_problem
    let mut el = EvolutionLog::new();

    // Create the Top Plane
    let top_plane_id = el.append(Operation::CreatePlane {
        nonce: "the top plane".to_string(),
    });
    el.append(Operation::SetPlaneName {
        plane_id: top_plane_id.clone(),
        name: "Top".to_string(),
    });
    let set_plane = el.append(Operation::SetPlane {
        plane_id: top_plane_id.clone(),
        plane: Plane::top(),
    });

    // Create the sketch
    let sketch_id = el.append(Operation::CreateSketch {
        nonce: "top sketch".to_string(),
    });
    el.append(Operation::SetSketchName {
        sketch_id: sketch_id.clone(),
        name: "Sketch1".to_string(),
    });
    el.append(Operation::SetSketchPlane {
        sketch_id: sketch_id.clone(),
        plane_id: top_plane_id.clone(),
    });

    // Create the base "L" shape
    el.append(Operation::AddSketchLine {
        sketch_id: sketch_id.clone(),
        start: (0.0, 0.0),
        end: (0.0, 100.0),
    });
    el.append(Operation::AddSketchLine {
        sketch_id: sketch_id.clone(),
        start: (0.0, 100.0),
        end: (50.0, 100.0),
    });
    el.append(Operation::AddSketchLine {
        sketch_id: sketch_id.clone(),
        start: (50.0, 100.0),
        end: (50.0, 50.0),
    });
    el.append(Operation::AddSketchLine {
        sketch_id: sketch_id.clone(),
        start: (50.0, 50.0),
        end: (100.0, 50.0),
    });
    el.append(Operation::AddSketchLine {
        sketch_id: sketch_id.clone(),
        start: (100.0, 50.0),
        end: (100.0, 0.0),
    });
    el.append(Operation::AddSketchLine {
        sketch_id: sketch_id.clone(),
        start: (100.0, 0.0),
        end: (0.0, 0.0),
    });
    let handle_id = el.append(Operation::AddSketchHandle {
        sketch_id: sketch_id.clone(),
        position: (20.0, 20.0),
    });

    // Create a new extrusion and set it to the sketch
    let extrusion_id = el.append(Operation::CreateExtrusion {
        nonce: "top extrusion".to_string(),
    });
    el.append(Operation::SetExtrusionName {
        extrusion_id: extrusion_id.clone(),
        name: "Extrude1".to_string(),
    });
    el.append(Operation::SetExtrusionSketch {
        extrusion_id: extrusion_id.clone(),
        sketch_id: sketch_id.clone(),
    });
    // el.append(Operation::SetExtrusionHandles {
    //     extrusion_id: extrusion_id.clone(),
    //     handles: vec![handle_id.clone()],
    // });

    el.git_log();
}
fn main_good() {
    let mut el = EvolutionLog::new();

    // Create the Top Plane
    let top_plane_id = el.append(Operation::CreatePlane {
        nonce: "a".to_string(),
    });
    el.append(Operation::SetPlaneName {
        plane_id: top_plane_id.clone(),
        name: "Top".to_string(),
    });
    let set_plane = el.append(Operation::SetPlane {
        plane_id: top_plane_id.clone(),
        plane: Plane::top(),
    });
    let new_extrusion = el.append(Operation::CreateExtrusion {
        nonce: "b".to_string(),
    });
    let set_ext_name = el.append(Operation::SetExtrusionName {
        extrusion_id: new_extrusion.clone(),
        name: "Extrusion1".to_string(),
    });

    // Rewind to an earlier commit
    el.checkout(top_plane_id.clone());
    el.append(Operation::SetPlaneName {
        plane_id: top_plane_id.clone(),
        name: "Bottom".to_string(),
    });
    el.cherry_pick(set_plane);
    el.cherry_pick(new_extrusion);
    el.cherry_pick(set_ext_name);

    el.git_log()
}

fn main_2() {
    let mut el = EvolutionLog::new();

    // Create the Top Plane
    let top_plane_id = el.append(Operation::CreatePlane {
        nonce: "a".to_string(),
    });
    // Note that top_plane_id is just a sha. the plane doesn't have any unique ID outside of its commit sha
    el.append(Operation::SetPlaneName {
        plane_id: top_plane_id.clone(),
        name: "Top".to_string(),
    });
    el.append(Operation::SetPlane {
        plane_id: top_plane_id.clone(),
        plane: Plane::top(),
    });

    // Create the Front Plane
    let front_plane_id = el.append(Operation::CreatePlane {
        nonce: "b".to_string(),
    });
    el.append(Operation::SetPlaneName {
        plane_id: front_plane_id.clone(),
        name: "Front".to_string(),
    });
    el.append(Operation::SetPlane {
        plane_id: front_plane_id.clone(),
        plane: Plane::front(),
    });

    // Create the main sketch
    let sketch_id = el.append(Operation::CreateSketch {
        nonce: "a".to_string(),
    });
    let name_sketch_commit = el.append(Operation::SetSketchName {
        sketch_id: sketch_id.clone(),
        name: "Sketch1".to_string(),
    });
    let set_sketch_plane_commit = el.append(Operation::SetSketchPlane {
        sketch_id: sketch_id.clone(),
        plane_id: front_plane_id.clone(),
    });

    el.append(Operation::AddSketchRectangle {
        sketch_id: sketch_id.clone(),
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 100.0,
    });

    let extrusion_id = el.append(Operation::CreateExtrusion {
        nonce: "c".to_string(),
    });
    let name_ext_commit = el.append(Operation::SetExtrusionName {
        extrusion_id: extrusion_id.clone(),
        name: "Extrude1".to_string(),
    });
    let set_ext_sketch_commit = el.append(Operation::SetExtrusionSketch {
        extrusion_id: extrusion_id.clone(),
        sketch_id: sketch_id.clone(),
    });
    // let set_ext_clicks_commit = el.append(Operation::SetExtrusionClicks {
    //     extrusion_id: extrusion_id.clone(),
    //     clicks: vec![(50.0, 50.0)],
    // });
    let finished_rectangle_commit = el.append(Operation::SetExtrusionDepth {
        extrusion_id: extrusion_id.clone(),
        depth: 10.0,
    });

    // Oops, our sketch was on the wrong plane. Fix that!
    let rotated_rectangle_commit = el.append(Operation::SetSketchPlane {
        sketch_id: sketch_id.clone(),
        plane_id: front_plane_id,
    });

    // Actually, let's try an alternate approach using a circle instead of a rectangle
    el.checkout(sketch_id.clone());

    // Re-use the commits that specified the sketch name and plane
    el.cherry_pick(name_sketch_commit);
    el.cherry_pick(set_sketch_plane_commit);

    // Add a circle to the sketch
    el.append(Operation::AddSketchCircle {
        sketch_id: sketch_id.clone(),
        x: 50.0,
        y: 50.0,
        radius: 50.0,
    });

    // Re-use all the extrusion commits
    el.cherry_pick(extrusion_id);
    el.cherry_pick(name_ext_commit);
    el.cherry_pick(set_ext_sketch_commit);
    // el.cherry_pick(set_ext_clicks_commit);
    let finished_circle_commit = el.cherry_pick(finished_rectangle_commit).unwrap();
    let rotated_circle_commit = el.cherry_pick(rotated_rectangle_commit).unwrap();

    // el.pretty_print();

    el.git_log();
}

fn main_old() {
    let point_a = vertex(Point3::new(0.0, 0.0, 0.0));
    let line_a = tsweep(&point_a, Vector3::new(1.0, 0.0, 0.0));
    let square_a = tsweep(&line_a, Vector3::new(0.0, 1.0, 0.0));
    let cube_a = tsweep(&square_a, Vector3::new(0.0, 0.0, 1.0));

    // simplest case!
    // let point_b = vertex(Point3::new(0.4, 0.4, 1.0));
    // let line_b = tsweep(&point_b, Vector3::new(0.2, 0.0, 0.0));
    // let square_b = tsweep(&line_b, Vector3::new(0.0, 0.2, 0.0));
    // let cube_b: Solid<
    //     truck_meshalgo::prelude::cgmath::Point3<f64>,
    //     truck_modeling::Curve,
    //     truck_modeling::Surface,
    // > = tsweep(&square_b, Vector3::new(0.0, 0.0, 0.2));

    // one flush side!
    let point_b = vertex(Point3::new(0.4, 0.4, 1.0));
    let line_b = tsweep(&point_b, Vector3::new(0.6, 0.0, 0.0));
    let square_b = tsweep(&line_b, Vector3::new(0.0, 0.2, 0.0));
    let cube_b: Solid<
        truck_meshalgo::prelude::cgmath::Point3<f64>,
        truck_modeling::Curve,
        truck_modeling::Surface,
    > = tsweep(&square_b, Vector3::new(0.0, 0.0, 0.2));

    // two flush sides!
    // let point_b = vertex(Point3::new(0.4, 0.4, 1.0));
    // let line_b = tsweep(&point_b, Vector3::new(0.6, 0.0, 0.0));
    // let square_b = tsweep(&line_b, Vector3::new(0.0, 0.6, 0.0));
    // let cube_b: Solid<
    //     truck_meshalgo::prelude::cgmath::Point3<f64>,
    //     truck_modeling::Curve,
    //     truck_modeling::Surface,
    // > = tsweep(&square_b, Vector3::new(0.0, 0.0, 0.2));

    // extend the cube to be just 0.01 longer than it needs to be
    // let cube_b = tsweep(&square_b, Vector3::new(0.0, 0.0, 1.01));
    // let bad_volume = tsweep(&square_b, Vector3::new(0.0, 0.0, -0.01));
    // then translate it down
    // let cube_b = translated(&cube_b, Vector3::new(0.0, 0.0, -0.01));
    // let combined_big = or(&cube_a, &cube_b, 0.01).unwrap();

    // let combined = or(&cube_a, &cube_b, 0.01).unwrap();
    let combined = fuse(&cube_a, &cube_b).unwrap();

    println!(
        "combined_cube_or has {:?} shell boundaries",
        combined.boundaries().len()
    );

    let mut mesh = combined.triangulation(0.01).to_polygon();
    mesh.put_together_same_attrs();
    let file = std::fs::File::create("combined_cube.obj").unwrap();
    obj::write(&mesh, file).unwrap();
}