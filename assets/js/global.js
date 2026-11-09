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

let currentOffset = 3;
let targetOffset = 3;
let isAnimating = false;

document.querySelector('.ring-wrapper')?.style.setProperty('--scroll-y', `${currentOffset}px`);
document.querySelector('.ring-wrapper')?.style.setProperty('--speed-offset', `${currentOffset / 40}s`);

function updateInertia() {
    currentOffset += (targetOffset - currentOffset) * 0.04;
    
    document.querySelector('.ring-wrapper')?.style.setProperty('--speed-offset', `${currentOffset / 10}s`);
    
    if (Math.abs(targetOffset - currentOffset) > 0.001) {
        requestAnimationFrame(updateInertia);
        isAnimating = true;
    } else {
        isAnimating = false;
    }
}

window.addEventListener('scroll', () => {
    const scrollOffset = window.scrollY * -0.15;
    document.querySelector('.ring-wrapper')?.style.setProperty('--scroll-y', `${scrollOffset}px`);
    targetOffset = scrollOffset;
    
    if (!isAnimating) {
        requestAnimationFrame(updateInertia);
        isAnimating = true;
    }
});