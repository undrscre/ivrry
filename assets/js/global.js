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
