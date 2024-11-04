struct Pt {
	x: i32,
	y: i32,
}

struct Rect {
	bl: Pt,
	tr: Pt,
}

impl Rect {
	fn area(&self) -> i32 {
    	let w = self.tr.x - self.bl.x;
    	let h = self.tr.y - self.bl.y;
    	w * h
	}
}

fn overlap(r1: &Rect, r2: &Rect) -> i32 {
	let x_overlap = (r1.tr.x.min(r2.tr.x) - r1.bl.x.max(r2.bl.x)).max(0);
	let y_overlap = (r1.tr.y.min(r2.tr.y) - r1.bl.y.max(r2.bl.y)).max(0);
	x_overlap * y_overlap
}

fn total_area(rects: &[Rect]) -> i32 {
	let mut total = 0;
	let mut overlap_total = 0;

	for rect in rects {
    	total += rect.area();
	}

	for i in 0..rects.len() {
    	for j in (i + 1)..rects.len() {
        	overlap_total += overlap(&rects[i], &rects[j]);
    	}
	}

	total - overlap_total
}

fn test_data() -> Vec<Rect> {
	vec![
    	Rect {
        	bl: Pt { x: 1, y: 3 },
        	tr: Pt { x: 6, y: 8 },
    	},
    	Rect {
        	bl: Pt { x: 4, y: 2 },
        	tr: Pt { x: 10, y: 4 },
    	},
    	Rect {
        	bl: Pt { x: 8, y: 6 },
        	tr: Pt { x: 14, y: 10 },
    	},
	]
}

fn main() {
	let rects = test_data();
	let area = total_area(&rects);
	println!("Total area covered: {}", area);
}
