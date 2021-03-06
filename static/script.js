var mazeType;
var mazeSize;
var phase = 'main';
var request = {};
var id;
var maze = {};

let initPage = function () {
	setMazeType('Rectangular')
	setMazeSize('Medium');
	setPhase('main');
	Spinner();
	Spinner.hide();
}

let moveMazeType = function (delta) {
	const types = ["Rectangular", "Triangular", "Circular", "Hexagonal"];
	const sz = types.length;
	var i = (types.indexOf(mazeType) + delta + sz) % sz;
	setMazeType(types[i]);
}

let setMazeType = function (tp) {
	mazeType = tp;
	for (const b of document.getElementsByClassName("typesel")) { b.style.display = "none"; }
	document.getElementById("type-" + tp).style.display = "block";
}

let setMazeSize = function (sz) {
	mazeSize = sz;
	for (const b of document.getElementsByClassName("busize")) { b.style.background = "white"; }
	document.getElementById("size-" + sz).style.background = "#ddd";
}

async function setPhase(ph) {
	if (phase == "wait") {
		Spinner.hide();
	}
	phase = ph;
	for (const d of document.getElementsByClassName("phasesel")) { d.style.display = "none"; }
	document.getElementById("phase-" + ph).style.display = "block";
	if (ph == "wait") {
		Spinner.show();
		await callPost();
	}
	if (ph == "done") {

		document.getElementById("guarantee-warning").style.display = maze.guaranteed ? "none" : "block";
		document.getElementById("proof").style.display = maze.guaranteed ? "block" : "none";
		document.getElementById("svg-slot").setAttribute("src", 'api/file/' + maze.svg);
		document.getElementById("svg-link").setAttribute("href", 'api/file/' + maze.svg);
		document.getElementById("pdf-link").setAttribute("href", 'api/file/' + maze.pdf);
		document.getElementById("mas-link").setAttribute("href", 'api/file/' + maze.maze_structure);
		document.getElementById("mai-link").setAttribute("href", 'api/file/' + maze.maze_instance);
		document.getElementById("protocol-link").setAttribute("href", 'api/file/' + maze.protocol);
	}
}

async function callPost() {
	try {
		let response = await fetch('api/maze', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json;charset=utf-8'
			},
			body: JSON.stringify(request)
		});

		id = await response.json();
		console.log(id);

		checkResult();

	} catch (e) {
		console.error(e);
		setPhase("error");
	}
}

async function checkResult() {
	console.log("checkResult");
	try {
		let response = await fetch('api/maze/' + id);
		console.log("checkResult response " + response.status);

		if (response.status != 200) {
			console.log(response.statusText);
			setPhase("error");
		} else {
			let message = await response.text();
			maze = JSON.parse(message);
			console.log(maze);
			if (maze.state == "Done") {
				await setPhase("done");
			} else if (maze.state == "Error") {
				await setPhase("error");
			} else {
				console.log(" next poll");
				await new Promise(resolve => setTimeout(resolve, 1000));
				await checkResult();
			}
		}
	} catch (e) {
		console.error(e);
		setPhase("error");
	}
}

let getMaze = function (guaranteed) {
	request = {
		guaranteed: guaranteed,
		payment: "free",
		size: mazeSize,
		type: mazeType
	};

	if (guaranteed) {
		/*
		if (typeof window.ethereum === 'undefined') {
			setPhase("install");
		} else 
		*/
		{
			setPhase("connect");
		}
	} else {
		setPhase("wait");
	}
}


function Spinner() {
	Spinner.element = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
	let c = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
	Spinner.element.setAttribute('width', '100');
	Spinner.element.setAttribute('height', '100');
	c.setAttribute('viewBox', '0 0 100 100');
	c.setAttribute('cx', '50');
	c.setAttribute('cy', '50');
	c.setAttribute('r', '42');
	c.setAttribute('stroke-width', '16');
	c.setAttribute('stroke', '#2196f3');
	c.setAttribute('fill', 'transparent');
	Spinner.element.appendChild(c);
	Spinner.element.style.cssText = 'position:absolute;left:calc(50% - 50px);top:calc(50% - 50px)';
	document.body.appendChild(Spinner.element)
}
Spinner.id = null;
Spinner.element = null;
Spinner.show = function () {
	const c = 264, m = 15;
	Spinner.element.style.display = 'block';
	move1();
	function move1() {
		let i = 0, o = 0;
		move();
		function move() {
			if (i == c) move2();
			else {
				i += 4; o += 8;
				Spinner.element.setAttribute('stroke-dasharray', i + ' ' + (c - i));
				Spinner.element.setAttribute('stroke-dashoffset', o)
				Spinner.id = setTimeout(move, m)
			}
		}
	}
	function move2() {
		let i = c, o = c * 2;
		move();
		function move() {
			if (i == 0) move1();
			else {
				i -= 4; o += 4;
				Spinner.element.setAttribute('stroke-dasharray', i + ' ' + (c - i));
				Spinner.element.setAttribute('stroke-dashoffset', o)
				Spinner.id = setTimeout(move, m)
			}
		}
	}
};
Spinner.hide = function () {
	Spinner.element.style.display = 'none';
	if (Spinner.id) {
		clearTimeout(Spinner.id);
		Spinner.id = null
	}
	Spinner.element.setAttribute('stroke-dasharray', '0 264');
	Spinner.element.setAttribute('stroke-dashoffset', '0')
};

