async function updateSiteStats() {
	const username = "ivrry";
	try {
		const request = await fetch(`https://nekoweb.org/api/site/info/${username}`);
		const response = await request.json();
		const date = new Date(response.updated_at).toLocaleDateString('en-US', { month: 'numeric', day: 'numeric' });

		document.getElementById('visitors').textContent = response.views.toString().padStart(4, '0');
		document.getElementById('updated').textContent = date;
		document.getElementById('followers').textContent = response.followers.toString().padStart(4, '0');

	} catch (err) {
		console.error(err)
	}
}

const decoCheckbox = document.querySelector("#deco");

decoCheckbox.addEventListener('change', () => {
	localStorage.setItem('decoEnabled', !decoCheckbox.checked);
	toggleDeco(!decoCheckbox.checked);
});

function toggleDeco(enabled) {
	const decoration = document.querySelectorAll('.deco');
	decoration.forEach(deco => {
		deco.style.display = enabled ? 'block' : 'none';
	});
}

document.querySelector(".niko-button").addEventListener('click', () => {
	const audio = new Audio("/assets/audio/squeak.mp3");
	
	audio.volume = .6
	audio.play();
});

document.addEventListener("DOMContentLoaded", () => {

	const decoEnabled = localStorage.getItem('decoEnabled') === null ? true : localStorage.getItem('decoEnabled') === 'true';
	decoCheckbox.checked = !decoEnabled;
	// decoLabel.style.display = !decoEnabled ? 'inline' : 'none';
	toggleDeco(decoEnabled);
	updateSiteStats();

});

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
