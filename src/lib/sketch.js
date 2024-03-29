import * as THREE from "three"
import { Point } from "./point.js"
import { LineSegment } from "./line_segment.js"
import { Arc } from "./arc.js"
import { Circle } from "./circle.js"
import { Face } from "./face.js"
import { Constraint } from "./constraint.js"
import { sketch_being_edited } from "../routes/cadmium/stores.js"

class Sketch {
	constructor(name, real_sketch, real_plane, points, lines, arcs, circles, faces, element) {
		this.being_edited = false
		this.always_visible = false
		this.name = name
		this.real_plane = real_plane
		this.points = real_sketch.points
		this.line_segments = real_sketch.line_segments
		this.circles = real_sketch.circles
		this.arcs = real_sketch.arcs
		this.group = new THREE.Group()
		for (let [point_id, point] of Object.entries(this.points)) {
			let point_2d = real_sketch.points_2d[point_id]
			let newPoint = new Point(point_id, point, point_2d, (parent = name))
			let extendedKey = `${name}:${point_id}`
			points[extendedKey] = newPoint
			if (point.hidden) {
				continue
			}
			newPoint.addTo(this.group)
		}

		for (let [line_segment_id, line_segment] of Object.entries(this.line_segments)) {
			let extendedKey = `${name}:${line_segment_id}`
			let newLineSegment = new LineSegment(extendedKey, line_segment, this.real_plane, name, points, element)
			lines[extendedKey] = newLineSegment
			newLineSegment.addTo(this.group)
		}

		for (let [circle_id, circle] of Object.entries(this.circles)) {
			let newCircle = new Circle(circle_id, circle, this.real_plane, name, points, element)
			let extendedKey = `${name}:${circle_id}`
			circles[extendedKey] = newCircle
			newCircle.addTo(this.group)
		}

		for (let [arc_id, arc] of Object.entries(this.arcs)) {
			let newArc = new Arc(arc_id, arc, this.real_plane, name, points, element)
			let extendedKey = `${name}:${arc_id}`
			arcs[extendedKey] = newArc
			newArc.addTo(this.group)
		}

		for (let face of real_sketch.faces) {
			let newFace = new Face(face, this.real_plane, name, points)
			// TODO: add to faces dict?
			newFace.addTo(this.group)
		}

		// console.log('constraints:', real_sketch.constraints)
		for (let [id, constraint] of Object.entries(real_sketch.constraints)) {
			let constraint2 = new Constraint(name, constraint, this.real_plane, name, points, lines, circles)
			constraint2.addTo(this.group)
		}

		// by default, hide the sketch
		this.group.visible = false
	}

	setEditing(editing) {
		console.log("sketch set editing:", editing)
		// sketch_being_edited.set(editing)
		this.being_edited = editing
		if (this.always_visible || this.being_edited) {
			this.group.visible = true
		} else {
			this.group.visible = false
		}
	}

	addTo(object) {
		object.add(this.group)
	}

	removeFrom(object) {
		object.remove(this.group)
	}
}

export { Sketch }
