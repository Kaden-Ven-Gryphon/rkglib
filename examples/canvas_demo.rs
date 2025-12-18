use rkglib::graphics::{canvas::{CanvasObject, Cord}, color::ColorARGB32, drawing_primitives::{BorderType, Circle, DrawOptions, FillType, Line, LineStyle, LineType, Rectange, VertexType}, drawing_text::{Text, TextEngine}, window::RkgWindow};



fn main() {
	let mut window = RkgWindow::new("Demo", 400, 400, ColorARGB32(0xFF050505));
	window.canvas.origin = rkglib::graphics::canvas::CanvasOrigin::BottomLeft;
	let draw_options = DrawOptions{
		offset: Cord{x:0,y:0},
		scale: Cord { x: 1, y: 1 },
		rotation:rkglib::graphics::drawing_primitives::RotationType::None,
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
		point_2: Cord { x: 70, y: -3 },
		style: LineStyle::Solid,
		width: 1
	};

	let text_engine = TextEngine::new();
	let text1 = Text::new(&"This is a test message", Cord::zero());
	let text2 = Text::new(&"Hello, World!", Cord::zero());

	rect.draw(&mut window.canvas, Cord { x: 10, y: 10 });
	circle.draw(&mut window.canvas, Cord { x: 100, y: 200 });
	line.draw(&mut window.canvas, Cord { x: 30, y: 399 });
	line.draw(&mut window.canvas, Cord { x: 50, y: 300 });
	text_engine.draw_text(&mut window.canvas, text1, Cord { x: 50, y: 300 }, rkglib::graphics::drawing_primitives::RotationType::None);
	text_engine.draw_text(&mut window.canvas, text2, Cord { x: 30, y: 400 }, rkglib::graphics::drawing_primitives::RotationType::Right);

	window.show();
}