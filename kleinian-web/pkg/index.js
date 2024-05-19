import init, { draw } from './kleinian_web.js';

async function run() {
	await init();
	const canvas = document.getElementById('drawing');
	const ctx = canvas.getContext('2d');

	const renderBtn = document.getElementById('render');
	document.getElementById('type').addEventListener('change', () => {
		if (document.getElementById('type').value == 'xii') {
			document.getElementById('b-input').style.display = 'none';
		}
		else {
			document.getElementById('b-input').style.display = '';
		}
	});

	renderBtn.addEventListener('click', () => {
		const width = parseInt(document.getElementById("width").value) || 800;
		const height = parseInt(document.getElementById("height").value) || 800;
		const a_re = parseFloat(document.getElementById("a-re").value) || 2;
		const a_im = parseFloat(document.getElementById("a-im").value) || 0;
		const b_re = parseFloat(document.getElementById("b-re").value) || 2;
		const b_im = parseFloat(document.getElementById("b-im").value) || 0;
		const typ = document.getElementById("type").value;
		const iters = parseInt(document.getElementById("iters").value) || 100000;
		canvas.width = width;
		canvas.height = height;
		draw(ctx, width, height, a_re, a_im, b_re, b_im, typ, iters);
	});
}

run();
