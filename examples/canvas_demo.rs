use rkglib::graphics::{canvas::{CanvasObject, Cord}, color::ColorARGB32, drawing_primitives::{BorderType, Circle, DrawOptions, FillType, Line, LineStyle, LineType, Rectange, VertexType}, window::RkgWindow};



fn main() {
	let mut window = RkgWindow::new("Demo", 400, 400, ColorARGB32(0xFF050505));
	window.canvas.origin = rkglib::graphics::canvas::CanvasOrigin::BottomLeft;
	let draw_options = DrawOptions{
		offset: Cord{x:0,y:0},
		scale: Cord { x: 1, y: 1 },
		rotation: 0.0,
		color: ColorARGB32(0xFFF00505),
		fill: FillType::Fill(None),
		border: BorderType::None,
		boarder_width: 1.0,
		line_type: LineType::Segment,
		vertex: VertexType::None
	};

	let rect = Rectange {
		options: draw_options,
		point_1: Cord { x: 0, y: 0 },
		point_2: Cord { x: 30, y: 50 }
	};

	let circle = Circle {
		options: draw_options,
		center: Cord::zero(),
		radius: 25
	};

	let line = Line {
		options: draw_options,
		point_1: Cord { x: 0, y: 0 },
		point_2: Cord { x: 70, y: 3 },
		style: LineStyle::Solid,
		width: 1
	};

	rect.draw(&mut window.canvas, Cord { x: 10, y: 10 });
	circle.draw(&mut window.canvas, Cord { x: 100, y: 200 });
	line.draw(&mut window.canvas, Cord { x: 10, y: 80 });

	window.show();
}