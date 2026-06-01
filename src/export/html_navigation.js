// HTML Navigation JavaScript for PowerPoint Exports

let currentSlide = 0;
let slides = [];

// Initialize on page load
document.addEventListener('DOMContentLoaded', function() {
    slides = document.querySelectorAll('.slide');
    currentSlide = 0;
    showSlide(currentSlide);
    updateCounter();

    // Add keyboard navigation
    document.addEventListener('keydown', handleKeyPress);

    // Add touch/swipe support for mobile
    let touchStartX = 0;
    let touchEndX = 0;

    document.addEventListener('touchstart', function(e) {
        touchStartX = e.changedTouches[0].screenX;
    });

    document.addEventListener('touchend', function(e) {
        touchEndX = e.changedTouches[0].screenX;
        handleSwipe();
    });

    function handleSwipe() {
        const swipeThreshold = 50;
        const diff = touchStartX - touchEndX;

        if (diff > swipeThreshold) {
            nextSlide();
        } else if (diff < -swipeThreshold) {
            previousSlide();
        }
    }
});

function showSlide(index) {
    // Hide all slides
    slides.forEach((slide, i) => {
        slide.classList.remove('active');
        if (i === index) {
            slide.classList.add('active');
        }
    });

    // Update slide counter
    updateCounter();

    // Update button states
    const prevBtn = document.getElementById('prevBtn');
    const nextBtn = document.getElementById('nextBtn');

    if (prevBtn) prevBtn.disabled = (index === 0);
    if (nextBtn) nextBtn.disabled = (index === slides.length - 1);
}

function nextSlide() {
    if (currentSlide < slides.length - 1) {
        currentSlide++;
        showSlide(currentSlide);
    }
}

function previousSlide() {
    if (currentSlide > 0) {
        currentSlide--;
        showSlide(currentSlide);
    }
}

function goToSlide(index) {
    if (index >= 0 && index < slides.length) {
        currentSlide = index;
        showSlide(currentSlide);
    }
}

function updateCounter() {
    const counter = document.getElementById('slideCounter');
    if (counter) {
        counter.textContent = `${currentSlide + 1} / ${slides.length}`;
    }
}

function handleKeyPress(event) {
    switch(event.key) {
        case 'ArrowRight':
        case ' ':
        case 'Enter':
            event.preventDefault();
            nextSlide();
            break;
        case 'ArrowLeft':
            event.preventDefault();
            previousSlide();
            break;
        case 'Home':
            event.preventDefault();
            goToSlide(0);
            break;
        case 'End':
            event.preventDefault();
            goToSlide(slides.length - 1);
            break;
        case 'n':
            // Toggle notes mode
            document.body.classList.toggle('notes-mode');
            document.body.classList.toggle('presentation-mode');
            break;
        case 'f':
            // Toggle fullscreen
            if (document.fullscreenElement) {
                document.exitFullscreen();
            } else {
                document.documentElement.requestFullscreen();
            }
            break;
    }
}

// Auto-play functionality (optional)
function startAutoPlay(interval = 5000) {
    return setInterval(() => {
        if (currentSlide < slides.length - 1) {
            nextSlide();
        } else {
            goToSlide(0); // Loop back to start
        }
    }, interval);
}

function stopAutoPlay(timerId) {
    clearInterval(timerId);
}