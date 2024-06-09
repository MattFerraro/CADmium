use std::fs;

use cadmium::archetypes::PlaneDescription;
use cadmium::isketch::face::FaceSelector;
use cadmium::isketch::primitive::SketchAddPointMessage;
use cadmium::message::idwrap::IDWrap;
use cadmium::message::ProjectMessageHandler;
use cadmium::project::Project;
use cadmium::workbench::AddSketch;
use cadmium::IDType;

pub trait TestCase: std::fmt::Debug {
    fn pre_selection(&self, p: &mut Project, sketch_id: IDType);
    fn post_selection(&self, p: &mut Project, sketch_id: IDType);
}

mod draw;
mod report;

mod simple_circles;

use draw::*;
use report::*;

fn create_project() -> (Project, IDType) {
    let mut p = Project::new("Test Project");
    let plane_description = PlaneDescription::PlaneId(0);
    let sketch_id = IDWrap { id: 0, inner: AddSketch { plane_description } }.handle_project_message(&mut p).unwrap().unwrap();
    IDWrap { id: 0, inner: IDWrap { id: sketch_id, inner: SketchAddPointMessage { x: 0.0, y: 0.0 } } }.handle_project_message(&mut p).unwrap().unwrap();

    (p, sketch_id)
}

#[derive(Debug)]
pub enum FaceSelectorType {
    ID(cadmium::isketch::face::IDSelector),
    Centroid(cadmium::isketch::face::CentroidSelector),
}

impl FaceSelector for FaceSelectorType {
    fn get_selected_faces(&self, isketch: &cadmium::isketch::ISketch) -> Vec<cadmium::isketch::face::Face> {
        match self {
            FaceSelectorType::ID(selector) => selector.get_selected_faces(isketch),
            FaceSelectorType::Centroid(selector) => selector.get_selected_faces(isketch),
        }
    }

    fn from_face_ids(_sketch: &cadmium::isketch::ISketch, _ids: Vec<IDType>) -> Self {
        unimplemented!()
    }
}

fn main() {
    // Create report dir
    fs::create_dir_all("bench-faceselector-report").unwrap();

    let mut results = vec![];
    let cases: Vec<(Box<dyn TestCase>, IDType)> = vec![
        (Box::new(simple_circles::SingleCircle()), 0),
        (Box::new(simple_circles::SingleCircleAddAnother()), 0),
    ];
    for case in cases.iter() {
        let (case_struct, index) = case;
        let (mut p, sketch_id) = create_project();

        case_struct.pre_selection(&mut p, sketch_id);
        let sketch_ref = p.get_workbench_by_id(0).unwrap().borrow().get_sketch_by_id(sketch_id).unwrap();

        let selectors = vec![
            FaceSelectorType::ID(cadmium::isketch::face::IDSelector::from_face_ids(&sketch_ref.borrow(), vec![*index])),
            FaceSelectorType::Centroid(cadmium::isketch::face::CentroidSelector::from_face_ids(&sketch_ref.borrow(), vec![*index])),
        ];

        for selector in selectors.iter() {
            println!("Drawing faces for selector: {:?}", selector);
            let case_name = format!("{:?}", case_struct);
            let selector_name_full = format!("{:?}", selector);
            let selector_name_variant = selector_name_full.split_once(" ").unwrap().0;
            let selector_name = selector_name_variant.split_once("(").unwrap().1;
            let name = format!("{}_{}", selector_name, case_name);
            results.push((selector_name.to_string(), case_name.to_string(), name.clone()));

            draw_sketch_faces(&mut p, selector, *index, format!("{}_before", name));
        }

        case_struct.post_selection(&mut p, sketch_id);

        for (id, selector) in selectors.iter().enumerate() {
            let name = results[id].2.clone();
            println!("Name: {}", name);
            draw_sketch_faces(&mut p, selector, *index, format!("{}_after", name));
        }
    }

    println!("results: {:?}", results);
    save_report_html(results);
}
