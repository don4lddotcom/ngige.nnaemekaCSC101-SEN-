script.js

// Add event listener to nav links
document.querySelectorAll('nav a').forEach(link => {
	link.addEventListener('click', event => {
		event.preventDefault();
		const target = document.querySelector(link.getAttribute('href'));
		target.scrollIntoView({ behavior: 'smooth' });
	});
});
