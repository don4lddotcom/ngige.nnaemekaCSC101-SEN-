// script.js

document.addEventListener('DOMContentLoaded', () => {
    console.log('Welcome to the Controllers website!');
    const navLinks = document.querySelectorAll('nav ul li a');

    navLinks.forEach(link => {
        link.addEventListener('click', (e) => {
            e.preventDefault();
            const targetSection = document.querySelector(link.getAttribute('href'));
            targetSection.scrollIntoView({ behavior: 'smooth' });
        });
    });
});
