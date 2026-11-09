const tooltip = document.querySelector("#tooltip");
document.addEventListener("mousemove", (e) => {
	const hoveredElement = document.elementFromPoint(e.clientX, e.clientY);
	const title = hoveredElement?.dataset.title || hoveredElement?.title || hoveredElement?.alt;
	
	tooltip.style.left = `${e.pageX + 25}px`;
	tooltip.style.top = `${e.pageY}px`;
	
	if (title) {
		if (!hoveredElement.dataset.title) {
			hoveredElement.dataset.title = title;
		}
		hoveredElement.removeAttribute('title');
		tooltip.innerHTML = title;
		tooltip.style.opacity = 1;
		tooltip.style.rotate = "0deg";
	} else {
		tooltip.style.opacity = 0;
	}
});

function fuuuck(max) {
	return Math.floor(Math.random() * max)
}

function updateCell(div, x,w) {
	let awa = fuuuck(10);
	div.style.border = "1px solid transparent";
	div.style.margin = "0px";
	if (awa >= 8) {
		div.style.background = `var(--col2)`;
	} else if (awa >= 5) {
		div.style.border = "1px solid var(--col3)";
		div.style.margin = `${awa / 10 * 10}px`;
		div.style.background = `transparent`;
	} else if (awa >= 3) {
		div.style.background = `var(--col1)`;
	} else {
		
	}
	div.style.opacity = awa / 5 - (1.0 - x / w);
}

function setupGrid(q,w,h,inv,stagger,update) {
	let grid = document.querySelector(q);
	let cells = [];
	for (let y = 0; y < h; y++) {
		for (let x = 0; x < w; x++) {
			let new_div = document.createElement("div");
			grid.appendChild(new_div);
			cells.push({el: new_div, x});
		}
	}
	cells.forEach(c => {
		setTimeout(() => {
			updateCell(c.el, c.x, w);
			setInterval(() => updateCell(c.el, c.x, w), update)
		}, (inv ? (w - c.x) : c.x) * stagger);
	});
}

document.addEventListener("DOMContentLoaded", (e) => {
	setupGrid("#underscores", 7, 3,true,120,2000)
	setupGrid("#underscores-footer", 16, 2,false,230,4000)
});